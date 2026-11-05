#![windows_subsystem = "windows"]

use color_eyre::Result;

use crate::ui::App;

mod modes;
mod ui;
pub mod lr_globalstate;

fn main() -> Result<()> {
    App::run()?;
    Ok(())
}
