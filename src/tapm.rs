use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tapm")]
pub struct Tapm {
    #[structopt(subcommand)]
    pub sub_command: TapmSubcommands,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tapm subcommands")]
pub enum TapmSubcommands {
    /// Create a new wasm app (e.g., `--new "dancing_web" [-s] `)
    New {
        /// Name your new app
        app_name: String,
        /// Create a side module
        #[structopt(short, long)]
        side_module: bool,
    },

    /// Start an HTTP server to serve your main module
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

    /// Publish your wasm app with Tarantella
    Publish {}, 
}
