use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tapm")]
struct Opt {
    #[structopt(subcommand)]
    sub_command: Tapm,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tapm subcommands")]
enum Tapm {
    /// Create a new wasm app (e.g., `--new "dancing_web" `)
    New {
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

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
