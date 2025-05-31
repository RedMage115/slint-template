use log::{error, info};
use velopack::{sources, UpdateCheck, UpdateManager};

pub fn init_velopack() {
    check_for_updates();
}

fn check_for_updates() {
    let source = sources::HttpSource::new("https://the.place/you-host/updates");
    let um = match UpdateManager::new(source, None, None) {
        Ok(u) => u,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let update_status = match um.check_for_updates() {
        Ok(u) => u,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let updates = match update_status {
        UpdateCheck::RemoteIsEmpty => {
            info!("remote was empty");
            return;
        }
        UpdateCheck::NoUpdateAvailable => {
            info!("no new updates");
            return;
        }
        UpdateCheck::UpdateAvailable(u) => u,
    };

    match um.download_updates(&updates, None) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
            return;
        }
    }
    match um.apply_updates_and_restart(&updates) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
            return;
        }
    }
}
