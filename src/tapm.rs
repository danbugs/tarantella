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

    /// Add a new dependency to your wasm app (e.g., `--add "dcw-0.1.0"`)
    Add { dependency_name_and_version: String },

    /// Build your wasmp app
    Build {},

    // /// Publish your side module to Tarantella
    // Publish {},
}
