use anyhow::Result;

mod app;
mod engine;

fn main() -> Result<()> {
    pretty_env_logger::try_init()?;

    log::debug!("Running application");
    app::run()?;

    Ok(())
}
