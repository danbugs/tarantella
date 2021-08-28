use std::process::Command;

use crate::utils;
use failure::{Context, ResultExt};

pub fn run(port: i32) -> Result<(), Context<String>> {
    let toml = utils::toml_to_struct("Tarantella.toml");
    let module_type = toml.unwrap().package.module_type;
    if module_type.eq("main_module") {
        utils::check_for_command(
            "basic-http-server",
            "`tapm` depends on basic-http-server. To install it, run: `cargo install basic-http-server`",
        )?;
        let err_msg = "tapm run failed".to_string();
        let mut child = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(&["/C", "basic-http-server", "-a", &format!("127.0.0.1:{}", port)])
                .spawn()
                .context(err_msg)?
        } else {
            Command::new("sh")
                .args(&["-c", "basic-http-server", "-a", &format!("127.0.0.1:{}", port)])
                .spawn()
                .context(err_msg)?
        };

        child.wait().unwrap();

    } else {
        return Err(Context::from(
            "`tapm run` is a command meant solely for main modules.".to_string(),
        ));
    }

    Ok(())
}