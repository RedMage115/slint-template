// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Standard Modules
mod workers;
mod requests;
// Feature Modules
#[cfg(feature = "installer")]
mod installer;
#[cfg(feature = "logging")]
mod logging;

#[cfg(feature = "dialogs")]
mod dialogs;

// Standard Usings
use log::debug;
use slint::ToSharedString;
use std::error::Error;
// Feature Usings
#[cfg(feature = "installer")]
use velopack::VelopackApp;

use crate::workers::BackgroundWorker;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "installer")]
    VelopackApp::build().run();

    init_features();
    let ui = AppWindow::new()?;
    debug!("initializing globals");
    init_globals(&ui);
    debug!("initializing background worker");
    let background_worker = BackgroundWorker::new(&ui);
    debug!("registering callbacks");
    requests::register_callbacks(&ui, &background_worker);
    debug!("starting app");
    ui.run()?;
    debug!("closing app");
    _ = background_worker.join();
    Ok(())
}

fn init_features() {
    #[cfg(feature = "logging")]
    logging::init_logging();
    #[cfg(feature = "installer")]
    installer::init_velopack();
}

fn init_globals(ui: &AppWindow) {
    ui.global::<AppConstants>().set_app_version_text(format!("Version: {}", env!("CARGO_PKG_VERSION")).to_shared_string());
}
