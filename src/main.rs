use anyhow::Result;

mod app;

fn main() -> Result<()> {
    pretty_env_logger::try_init()?;

    log::debug!("Running application");
    app::run()?;

    Ok(())
}
