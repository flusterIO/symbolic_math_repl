mod convert;

use anyhow::Result;

use crate::cli::Command;

pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Convert(args) => convert::run(args),
    }
}
