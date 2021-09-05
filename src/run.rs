use crate::utils;
use failure::Context;

pub fn run(port: i32) -> Result<(), Context<String>> {
    let module_type = utils::check_for_toml_field("module_type")?;
    let server = utils::check_for_toml_field("server");
    let err_msg = "tapm run failed";

    if module_type.eq("main_module") && server.is_err() {
        utils::check_for_command(
            "basic-http-server",
            "tapm depends on basic-http-server. To install it, run: `cargo install basic-http-server`",
        )?;
        let mut child = utils::spawn_command(&format!("basic-http-server -a 127.0.0.1:{}", port), err_msg)?;

        child.wait().unwrap();
    } else if module_type.eq("main_module") && !server.is_err() {
        utils::check_for_command(
            "node",
            "tapm depends on node. To install it, see: https://nodejs.org/en/download/current/",
        )?;
        let mut child = utils::spawn_command(&format!("node {}", server.unwrap()), err_msg)?;

        child.wait().unwrap();
    } else {
        return Err(Context::from(
            "tapm run is a command meant solely for main modules.".to_string(),
        ));
    }

    Ok(())
}
