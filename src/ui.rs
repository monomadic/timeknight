use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use std::time::Duration;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<Task>,
    /// Currently selected task
    selected_task: usize,
}

struct Task {
    description: String,
    timer: stopwatch::Stopwatch,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            selected_task: 0,
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('a') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('k') => {
                            if app.selected_task > 0 {
                                app.selected_task -= 1;
                            }
                        }
                        KeyCode::Char('j') => {
                            app.selected_task += 1;
                        }
                        KeyCode::Char('l') => {
                            if let Some(task) = app.messages.get_mut(app.selected_task) {
                                if task.timer.is_running() {
                                    task.timer.stop();
                                } else {
                                    task.timer.start();
                                }
                            }
                        }
                        KeyCode::Char('x') => {
                            if let Some(_) = app.messages.get_mut(app.selected_task) {
                                app.messages.remove(app.selected_task);
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            app.messages.push(Task {
                                description: app.input.drain(..).collect(),
                                timer: stopwatch::Stopwatch::start_new(),
                            });
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let task_selected_color = Color::Rgb(255, 0, 200);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Add Task input
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Add Task"));
    f.render_widget(input, chunks[1]);

    // help text
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw(""),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": quit, "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": add task, "),
                Span::styled("j/k", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": select task, "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": start/stop task, "),
                Span::styled("x", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": delete task"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[2]);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    // task list
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let running_icon = if m.timer.is_running() {"▶️▶️ "} else {"❚❚ "};

            let content = vec![Spans::from(Span::raw(format!(
                "{}{} - {}",
                running_icon,
                m.description,
                humantime::format_duration(Duration::new(m.timer.elapsed().as_secs(), 0))
            )))];
            ListItem::new(content).style(match app.selected_task == i {
                true => Style::default().fg(match app.input_mode {
                    InputMode::Normal => task_selected_color,
                    InputMode::Editing => Color::White,
                }),
                false => Style::default(),
            })
        })
        .collect();

    let messages = List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .title("♘TimeKnight")
            .style(match app.input_mode {
                InputMode::Normal => Style::default().fg(Color::White),
                InputMode::Editing => Style::default(),
            }),
    );
    f.render_widget(messages, chunks[0]);
}
