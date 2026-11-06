#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use color_eyre::Result;

use crate::ui::App;

pub mod lr_globalstate;
mod modes;
mod ui;

fn main() -> Result<()> {
    App::run()?;
    Ok(())
}
