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
    pub fn add_task(&mut self, description: &str) -> Result<(), crate::Error> {
        self.tasks.push(Task {
            description: description.into(),
            timer: crate::timer::Stopwatch::start_new(),
        });
        self.input_mode = InputMode::Normal;
        crate::storage::save_state(&self)
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        crate::storage::save_state(&self)
    }

    pub fn move_up(&mut self) {
        if self.selected_task > 0 {
            self.selected_task -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_task < self.tasks.len() - 1 {
            self.selected_task += 1;
        }
    }

    pub fn delete_selected_task(&mut self) -> Result<(), crate::Error> {
        if let Some(_) = self.tasks.get_mut(self.selected_task) {
            self.tasks.remove(self.selected_task);
            crate::storage::save_state(&self)
        } else {
            unimplemented!();
        }
    }

    pub fn toggle_play_pause_selected_task(&mut self) -> Result<(), crate::Error> {
        if let Some(task) = self.tasks.get_mut(self.selected_task) {
            if task.timer.is_running() {
                task.timer.stop();
            } else {
                task.timer.start();
            }
            crate::storage::save_state(&self)
        } else {
            unimplemented!();
        }
    }

    pub fn complete_selected_task(&mut self) -> Result<(), crate::Error> {
        if let Some(task) = self.tasks.get_mut(self.selected_task) {
            task.complete().unwrap(); // todo: fix this unwrap
            crate::storage::save_state(&self)
        } else {
            unimplemented!();
        }
    }

    pub fn reset_selected_task(&mut self) -> Result<(), crate::Error> {
        if let Some(task) = self.tasks.get_mut(self.selected_task) {
            task.timer.reset();
            crate::storage::save_state(&self)
        } else {
            unimplemented!();
        }
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
