use exitfailure::ExitFailure;
use structopt::StructOpt;
#[macro_use]
extern crate log;
use env_logger::{Builder, Env};

pub mod add;
pub mod build;
pub mod constants;
pub mod login;
pub mod new;
pub mod publish;
pub mod register;
pub mod run;
pub mod tapm;
pub mod utils;

use tapm::{Tapm, TapmSubcommands};

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Tapm::from_args();
    let env = Env::new().default_filter_or("tapm=info,warn,error");
    Builder::from_env(env)
        .default_format_module_path(false)
        .default_format_timestamp(false)
        .init();

    match opt.sub_command {
        TapmSubcommands::New {
            app_name,
            side_module,
            no_git,
            server,
        } => new::new(app_name, side_module, no_git, server)?,
        TapmSubcommands::Run { port } => match run::run(port) {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
        TapmSubcommands::Build {} => match build::build() {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
        TapmSubcommands::Login {} => match login::login() {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
        TapmSubcommands::Register {} => match register::register() {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
        TapmSubcommands::Publish {} => match publish::publish().await {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
        TapmSubcommands::Add {
            owner_and_depname,
            version,
        } => match add::add(owner_and_depname, version).await {
            Ok(_) => (),
            Err(err_msg) => error!("{}", err_msg),
        },
    };
    Ok(())
}
