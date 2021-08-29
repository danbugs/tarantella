use failure::Context;

pub fn register() -> Result<(), Context<String>> {
    println!("Info: You must be registered on GitHub to publish apps with Tarantella. To register, see: https://github.com/join");
    Ok(())
}