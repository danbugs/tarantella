use failure::Context;

use crate::utils;

pub fn login () -> Result<(), Context<String>> {
    utils::check_for_command("gh", "tapm depends on the GitHub CLI. To install it, see: https://github.com/cli/cli#installation")?;
    let mut child = utils::spawn_command("gh auth login --scopes delete_repo", "tapm login failed")?;
    child.wait().unwrap();
    Ok(())
}