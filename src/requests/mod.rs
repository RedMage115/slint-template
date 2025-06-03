use log::{debug};
use slint::ComponentHandle;
use crate::{workers::BackgroundWorker, AppWindow};

pub fn register_callbacks(ui: &AppWindow, background_worker: &BackgroundWorker) {
    /* 
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            debug!("Counter: {}", ui.get_counter());
        }
    });
*/
}




