use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;
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

use crate::state::*;

pub fn run(app: App) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    // let app = App::default();
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
        terminal.draw(|f| ui(f, &mut app))?;
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
                            if app.selected_task < app.tasks.len() - 1 {
                                app.selected_task += 1;
                            }
                        }
                        KeyCode::Char('l') => {
                            if let Some(task) = app.tasks.get_mut(app.selected_task) {
                                if task.timer.is_running() {
                                    task.timer.stop();
                                } else {
                                    task.timer.start();
                                }
                                crate::storage::save_state(&app).unwrap();
                            }
                        }
                        KeyCode::Char('x') => {
                            let _ = app.delete_selected_task();
                        }
                        KeyCode::Char('r') => {
                            if let Some(task) = app.tasks.get_mut(app.selected_task) {
                                task.timer.reset();
                                crate::storage::save_state(&app).unwrap();
                            }
                        }
                        KeyCode::Char('C') => {
                            if let Some(task) = app.tasks.get_mut(app.selected_task) {
                                task.complete().unwrap(); // todo: error management
                                crate::storage::save_state(&app).unwrap();
                            }
                        }
                        KeyCode::Char('s') => {
                            crate::storage::save_state(&app).unwrap();
                        }
                        KeyCode::Char('?') => {
                            // mini event loop just for the popup
                            terminal.draw(|f| draw_popup(f))?;

                            loop {
                                if let Event::Key(key) = event::read()? {
                                    if  key.code == KeyCode::Esc ||
                                        key.code == KeyCode::Char('?') ||
                                        key.code == KeyCode::Char('q') {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            app.tasks.push(Task {
                                description: app.input.drain(..).collect(),
                                timer: crate::timer::Stopwatch::start_new(),
                            });
                            app.input_mode = InputMode::Normal;
                            crate::storage::save_state(&app).unwrap();
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let task_running_color = Color::Rgb(255, 0, 200);
    let title_text = " ♞ TimeKnight ";
    let total_time = humantime::format_duration(Duration::new(app.active_elapsed().as_secs(), 0));
    let time_text = format!(" Total Time: {} ", total_time);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
        ].as_ref())
        .split(f.size());

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(title_text.len() as u16),
            Constraint::Min(1),
            Constraint::Length(time_text.len() as u16),
        ].as_ref())
        .split(tui::layout::Rect {
            x: 0,
            y: 0,
            width: f.size().width,
            height: 1,
        });

    // Title
    let titlebar = Block::default().title(
        Span::styled(title_text,
            Style::default()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD))
    ).style(Style::default().bg(Color::Rgb(20,20,20)));
    f.render_widget(titlebar, header_layout[0]);

    // Title Spacer
    let spacer = Block::default()
        .style(Style::default()
            .bg(Color::Rgb(20,20,20)));
    f.render_widget(spacer, header_layout[1]);

    // Total Time
    let spacer = Paragraph::new(time_text)
        .alignment(tui::layout::Alignment::Right)
        .style(Style::default()
            .fg(Color::Black)
            .bg(Color::LightYellow)
            .add_modifier(Modifier::BOLD));
    f.render_widget(spacer, header_layout[2]);

    // Active Tasks List
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let running_icon = if m.timer.is_running() {
                " ► "
            } else {
                "  "
            };
            let content = vec![Spans::from(Span::raw(format!(
                "{}{} - {}",
                running_icon,
                m.description,
                humantime::format_duration(Duration::new(m.timer.elapsed().as_secs(), 0))
            )))];
            ListItem::new(content).style(match app.selected_task == i {
                true => match m.timer.is_running() {
                    true => Style::default().bg(task_running_color).fg(Color::White),
                    false => Style::default().bg(Color::White).fg(Color::Black),
                },
                false => match m.timer.is_running() {
                    true => Style::default().fg(task_running_color),
                    false => Style::default().fg(Color::White),
                },
            })
        })
        .collect();
    let tasks = List::new(tasks).block(
        Block::default()
            .borders(Borders::NONE)
            .title(" Timers ")
            .style(
                match app.input_mode {
                    InputMode::Normal => Style::default().fg(Color::White),
                    InputMode::Editing => Style::default(),
                }
                .add_modifier(Modifier::BOLD),
            ),
    );
    f.render_widget(tasks, vertical_layout[1]);

    // Add Task input
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default(),
        })
        .block(
            Block::default().title(Span::styled(
                "  Add Timer ",
                match app.input_mode {
                    InputMode::Editing => Style::default()
                        .bg(Color::LightYellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                    InputMode::Normal => Style::default(),
                },
            )),
        );
    f.render_widget(input, vertical_layout[2]);

    // Help Text
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw(""),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": quit, "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": add, "),
                Span::styled("j/k", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": select, "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": start/stop, "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": reset, "),
                Span::styled("x", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": delete, "),
                Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": help"),
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
    f.render_widget(help_message, vertical_layout[3]);


    // cursor
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                vertical_layout[2].x + app.input.width() as u16,
                // Move one line down, from the border to the input line
                vertical_layout[2].y + 1,
            )
        }
    }
}


fn draw_popup<B: Backend>(f: &mut Frame<B>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([
                Constraint::Min(1),
        ].as_ref())
        .split(f.size());
    let backdrop = Block::default().style(Style::default().bg(Color::Rgb(20,20,20)));

    let popup = List::new(
            vec![
                ListItem::new(
                    Span::styled(" a:   add timer            j/k: select timer",
                        Style::default().add_modifier(Modifier::BOLD))),
                ListItem::new(
                    Span::styled(" l:   start/stop timer     x: delete timer",
                        Style::default().add_modifier(Modifier::BOLD))),
                ListItem::new(
                    Span::styled(" ?:   help                 q: quit",
                        Style::default().add_modifier(Modifier::BOLD))),
                
            ]
        )
        .block(Block::default().title(Span::styled("Help:Keymaps",
            Style::default().add_modifier(Modifier::BOLD)
        )).borders(Borders::ALL))
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Rgb(0,0,0))
            .add_modifier(Modifier::BOLD));

    f.render_widget(backdrop, f.size());
    f.render_widget(tui::widgets::Clear, layout[0]);
    f.render_widget(popup, layout[0]);
}
