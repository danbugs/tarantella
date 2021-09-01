use crate::utils;
use failure::Context;
use std::str;

pub fn build() -> Result<(), Context<String>> {
    utils::check_for_command("emmake make", "tapm depends on emmake — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_command("emcc", "tapm depends on emcc — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html")?;
    utils::check_for_path("Makefile", "Makefile is missing")?;
    let build_dir = utils::check_for_toml_field("build_dir")?;
    utils::check_for_path(&build_dir, &format!("{} folder is missing", &build_dir))?;
    let output = utils::run_command("emmake make", "tapm build failed")?;
    let stderr = str::from_utf8(&output.stderr).unwrap();
    if stderr.contains("emcc") { // we only care about an error from `emcc` — make prints to stderr even on success
        return Err(Context::from(stderr.to_string()));
    }
    info!("{}", &format!("Created new build at {}/", &build_dir));
    Ok(())
}
