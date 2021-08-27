use exitfailure::ExitFailure;
use structopt::StructOpt;

pub mod add;
pub mod build;
pub mod constants;
pub mod new;
pub mod tapm;
pub mod utils;

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
