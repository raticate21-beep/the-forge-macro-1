use active_win_pos_rs::{self, get_active_window};
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use rand::Rng;
use std::sync::{self, Arc, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

pub fn clicker(is_running: Arc<AtomicBool>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";
    let mut rng = rand::rng();
    let random_delay = rng.random_range(600..900);
    loop {
        if is_running.load(sync::atomic::Ordering::Relaxed) {
            if let Ok(active_window) = get_active_window() {
                let title = active_window.title.to_lowercase();

                if title.contains(target_window_name) {
                    let _ = enigo.button(Button::Left, Click);
                    thread::sleep(Duration::from_millis(random_delay));
                } else {
                    thread::sleep(Duration::from_millis(200));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
pub fn luck(is_luck: Arc<AtomicBool>) {}
pub fn sell(is_sell: Arc<AtomicBool>) {}
