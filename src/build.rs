use crate::utils;
use failure::{Context, ResultExt};
use std::process::Command;

pub fn build() -> Result<(), Context<String>> {
    utils::check_for_command("emmake make", "tapm depends on emmake — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_command("emcc", "tapm depends on emcc — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_path("Makefile", "Makefile is missing")?;
    let build_dir = &utils::toml_to_struct("Tarantella.toml")
        .unwrap()
        .package
        .build_dir;
    utils::check_for_path(build_dir, &format!("{} folder is missing", &build_dir))?;
    utils::run_command("emmake make","tapm build failed" )?;
    Ok(())
}