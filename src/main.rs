#[macro_use]
extern crate log;

mod state;
mod storage;
mod ui;
mod timer;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // set up a logger with default level 'info'
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    match run() {
        Ok(_) => info!("done"),
        Err(e) => error!("{}", e), // todo: print error properly
    }
}

fn run() -> Result<()> {
    // load app state from disk
    let app = storage::load_state()?;

    ui::run(app)
}