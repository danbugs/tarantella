use exitfailure::ExitFailure;
use structopt::StructOpt;
#[macro_use] extern crate log;
use env_logger::{Builder, Env};

pub mod build;
pub mod constants;
pub mod new;
pub mod run;
pub mod tapm;
pub mod utils;
pub mod login;
pub mod register;
pub mod publish;

use tapm::{Tapm, TapmSubcommands};

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Tapm::from_args();
    let env = Env::new().default_filter_or("tapm=info");
    Builder::from_env(env)
        .default_format_module_path(false)
        .default_format_timestamp(false)
        .init();

    match opt.sub_command {
        TapmSubcommands::New {
            app_name,
            side_module,
        } => new::new(app_name, side_module)?,
        TapmSubcommands::Run {port} => run::run(port)?,
        TapmSubcommands::Build {} => build::build()?,
        TapmSubcommands::Login {} => login::login()?,
        TapmSubcommands::Register {} => register::register()?,
        TapmSubcommands::Publish {} => publish::publish().await?,
    };
    Ok(())
}
