use crate::utils;
use failure::{Context, ResultExt};
use std::process::Command;

pub fn build() -> Result<(), Context<String>> {
    utils::check_for_command("emmake make", "We couldn't run the `emmake` command — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_command("emcc", "We couldn't run the `emmake` command — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_path("Makefile", "Makefile is missing")?;
    let build_dir = &utils::toml_to_struct("Tarantella.toml")
        .unwrap()
        .package
        .build_dir;
    utils::check_for_path(build_dir, &format!("{} folder is missing", &build_dir))?;
    let err_msg = "tapm build failed".to_string();
    if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["/C", "emmake make"])
            .output()
            .context(err_msg)?;
    } else {
        Command::new("sh")
            .args(&["-c", "emmake make"])
            .output()
            .context(err_msg)?;
    }

    Ok(())
}