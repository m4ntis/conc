mod dasm;
mod dbg;
mod run;

use clap::Parser;

#[derive(Parser)]
#[command(name = "conc")]
#[command(bin_name = "conc")]
pub enum Command {
    Dasm(dasm::Args),
    Run(run::Args),
    Dbg(dbg::Args),
}

impl Command {
    // TODO: Return err and stuff
    pub fn run(self) {
        // TODO: Perhaps Args structs can be some trait and the match can be
        // avoided?
        match self {
            Command::Dasm(args) => dasm::run(args),
            Command::Run(args) => run::run(args),
            Command::Dbg(args) => dbg::run(args),
        }
    }
}
