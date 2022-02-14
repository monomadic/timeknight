use std::time::Duration;

use serde::{Deserialize, Serialize};

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

impl App {
    pub fn delete_selected_task(&mut self) -> Result<(), crate::Error> {
        if let Some(_) = self.tasks.get_mut(self.selected_task) {
            self.tasks.remove(self.selected_task);
            crate::storage::save_state(&self)
        } else {
            unimplemented!();
        }
    }

    pub fn complete_selected_task(&mut self) -> Result<(), crate::Error> {
        Ok(())
    }
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

impl Task {
    pub fn complete(&mut self) -> Result<(), crate::Error> {
        // self.timer
        Ok(())
    }
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

impl App {
    pub fn active_elapsed(&self) -> Duration {
        self.tasks
            .iter()
            .fold(Duration::new(0, 0), |acc, task| acc + task.timer.elapsed())
    }
}
