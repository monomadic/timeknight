use serde::{Serialize, Deserialize};

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded tasks
    pub tasks: Vec<Task>,
    /// Currently selected task
    pub selected_task: usize,
}

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub timer: crate::timer::Stopwatch,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            tasks: Vec::new(),
            selected_task: 0,
        }
    }
}
