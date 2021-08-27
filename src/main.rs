use exitfailure::ExitFailure;
use failure::Context;
use std::process::Command;
use structopt::StructOpt;

pub mod add;
pub mod build;
pub mod constants;
pub mod new;
pub mod tapm;

use tapm::{Tapm, TapmSubcommands};

fn main() -> Result<(), ExitFailure> {
    let opt = Tapm::from_args();

    match opt.sub_command {
        TapmSubcommands::New {
            app_name,
            side_module,
        } => new::new(app_name, side_module)?,
        TapmSubcommands::Add {
            dependency_name_and_version,
        } => add::add(dependency_name_and_version)?,
        TapmSubcommands::Build {} => build::build()?,
    };
    Ok(())
}

fn check_for_emcc() -> Result<(), Context<String>> {
    match Command::new("emcc -v").spawn() {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Context::from("Emscripten is not installed in your system. To install it, visit: https://emscripten.org/docs/getting_started/downloads.html".to_string()));
        }
    }
}
