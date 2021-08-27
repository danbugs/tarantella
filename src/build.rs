use failure::{Context, ResultExt};
use std::process::Command;

pub fn build() -> Result<(), Context<String>> {
    check_for_emmake()?;
    check_for_emcc()?;
    let err_msg = "tapm build failed".to_string();
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(&["/C", "emmake make"]).output().context(err_msg)?;
    } else {
        Command::new("sh").args(&["-c", "emmake make"]).output().context(err_msg)?;
    }

    Ok(())
}

fn check_for_emmake() -> Result<(), Context<String>> {
    let err_msg = "We couldn't run the `emmake` command — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html".to_string();
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(&["/C", "emmake make", "-v"]).output().context(err_msg)?;
    } else {
        Command::new("sh").args(&["-c", "emmake make", "-v"]).output().context(err_msg)?;
    }
    Ok(())
}

fn check_for_emcc() -> Result<(), Context<String>> {
    let err_msg = "We couldn't run the `emcc` command — make sure you have got Emscripten installed. For instructions, visit: https://emscripten.org/docs/getting_started/downloads.html".to_string();
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(&["/C", "emcc", "-v"]).output().context(err_msg)?;
    } else {
        Command::new("sh").args(&["-c", "emcc", "-v"]).output().context(err_msg)?;
    }

    Ok(())
}
