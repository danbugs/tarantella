use exitfailure::ExitFailure;
use structopt::StructOpt;

pub mod build;
pub mod constants;
pub mod new;
pub mod run;
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
        TapmSubcommands::Run {port} => run::run(port)?,
        TapmSubcommands::Build {} => build::build()?,
    };
    Ok(())
}
