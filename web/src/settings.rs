use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const KEY: &str = "mate.settings";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    kiosk_mode: bool,
}

pub fn get_all() -> Settings {
    LocalStorage::get(KEY).unwrap_or_default()
}

fn save(settings: Settings) {
    LocalStorage::set(KEY, settings).unwrap()
}

pub fn is_kiosk_mode() -> bool {
    get_all().kiosk_mode
}

pub fn set_kiosk_mode(active: bool) {
    let mut settings = get_all();
    settings.kiosk_mode = active;
    save(settings);
}
