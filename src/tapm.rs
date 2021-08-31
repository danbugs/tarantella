use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tapm",
    about = "tapm is a modern package manager for C/C++ WASM apps."
)]
pub struct Tapm {
    #[structopt(subcommand)]
    pub sub_command: TapmSubcommands,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tapm subcommands")]
pub enum TapmSubcommands {
    /// Create a new wasm app (e.g., tapm new "dancing_web" [-s])
    New {
        /// Name your new wasm app
        app_name: String,
        /// Make your new wasm app a side module
        #[structopt(short, long)]
        side_module: bool,
    },

    /// Start an HTTP server to test your main module (e.g., tapm run [-p 8000])
    Run {
        /// Specify a port
        #[structopt(short, long, default_value = "4000")]
        port: i32,
    },

    /// Build your wasm app
    Build {},

    /// Login to GitHub to publish your wasm app with Tarantella
    Login {},

    /// Register to GitHub to publish your wasm app with Tarantella
    Register {},

    /// Publish a new release of your wasm app to GitHub with Tarantella (if your code is private, your release will be published to a separate repo)
    Publish {},

    /// Add a new dependency to your wasm app (e.g., tapm add "danbugs/dancing_web" ["0.2.0"])
    Add {
        /// Owner and dependency's name (e.g., "danbugs/dancing_web")
        owner_and_depname: String,
        /// Dependency's version (e.g., "0.2.0"). If no value is provided, tapm defaults to the latest version
        version: Option<String>,
    },
}
