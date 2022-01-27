use structopt::StructOpt;
use crate::Error;

pub(crate) fn run() -> Result<(), Error> {
    println!("NI Extractor v0.0.1\n");

    match Command::from_args() {
        Command::Add => {},
        Command::List => {},
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
enum Command {
    Add,
    List,
}