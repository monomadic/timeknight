#[macro_use]
extern crate log;

mod state;
mod storage;
mod ui;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    // set up a logger with default level 'info'
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    // load app state from disk
    let app = storage::load_state();

    match ui::run(app) {
        Ok(_) => info!("done"),
        Err(e) => error!("{}", e), // todo: print error properly
    }
}
