use active_win_pos_rs::{self, get_active_window};
use eframe::egui::mutex::Mutex;
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use rand::Rng;
use std::thread;
use std::{error::Error, time::Duration};
use std::{
    sync::{self, Arc, atomic::AtomicBool},
    time::Instant,
};

pub fn clicker(is_running: Arc<AtomicBool>, is_busy: Arc<AtomicBool>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";
    let mut rng = rand::rng();
    loop {
        if is_running.load(sync::atomic::Ordering::Relaxed) {
            if !is_busy.load(sync::atomic::Ordering::Relaxed) {
                if let Ok(active_window) = get_active_window() {
                    let title = active_window.title.to_lowercase();
                    if title.contains(target_window_name) {
                        let _ = enigo.button(Button::Left, Click);
                        let random_delay = rng.random_range(600..900);

                        thread::sleep(Duration::from_millis(random_delay));
                    } else {
                        thread::sleep(Duration::from_millis(1000));
                    }
                } else {
                    thread::sleep(Duration::from_millis(200));
                }
            } else {
                thread::sleep(Duration::from_millis(500));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
pub fn luck(is_luck: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, potion_key: Arc<Mutex<String>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";

    let mut last_potion_time = Instant::now() - Duration::from_secs(300);
    loop {
        if is_luck.load(sync::atomic::Ordering::Relaxed) {
            if let Ok(active_window) = get_active_window() {
                let title = active_window.title.to_lowercase();
                if title.contains(target_window_name) {
                    if last_potion_time.elapsed() >= Duration::from_secs(300) {
                        if is_busy
                            .compare_exchange(
                                false,
                                true,
                                sync::atomic::Ordering::Acquire,
                                sync::atomic::Ordering::Relaxed,
                            )
                            .is_ok()
                        {
                            let _ = enigo.key(
                                Key::Unicode(potion_key.lock().chars().next().unwrap_or('3')),
                                Click,
                            );
                            thread::sleep(Duration::from_millis(100));
                            let _ = enigo.button(Button::Left, Press);
                            thread::sleep(Duration::from_millis(2500));
                            let _ = enigo.button(Button::Left, Release);
                            let _ = enigo.key(Key::Unicode('1'), Click);

                            last_potion_time = Instant::now();

                            is_busy.store(false, sync::atomic::Ordering::Relaxed);
                        } else {
                            thread::sleep(Duration::from_millis(100));
                        }
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                } else {
                    thread::sleep(Duration::from_millis(1000));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}

pub fn smooth_move_mouse(
    enigo: &mut Enigo,
    target_x: i32,
    target_y: i32,
    steps: i32,
    delay_ms: u64,
) -> Result<(), Box<dyn Error>> {
    // 1. Pobieramy pozycję okna
    let window = match get_active_window() {
        Ok(win) => win,
        Err(_) => return Err("Not found active window!".into()),
    };

    let win_x = window.position.x as f64;
    let win_y = window.position.y as f64;
    let win_w = window.position.width as f64;
    let win_h = window.position.height as f64;

    let base_width: f64 = 2560.0;
    let base_height: f64 = 1440.0;

    let scale_x = win_w / base_width;
    let scale_y = win_h / base_height;

    let final_target_x = win_x + (target_x as f64 * scale_x);
    let final_target_y = win_y + (target_y as f64 * scale_y);

    let (start_x, start_y) = enigo.location()?;

    let dx = final_target_x - start_x as f64;
    let dy = final_target_y - start_y as f64;

    let step_x_f = dx / steps as f64;
    let step_y_f = dy / steps as f64;

    for i in 1..=steps {
        let current_x = start_x as f64 + step_x_f * i as f64;
        let current_y = start_y as f64 + step_y_f * i as f64;

        enigo
            .move_mouse(
                current_x.round() as i32,
                current_y.round() as i32,
                Coordinate::Abs,
            )
            .unwrap();

        thread::sleep(Duration::from_millis(delay_ms));
    }

    Ok(())
}
pub fn mouse_click_release() {
    thread::sleep(Duration::from_millis(1000));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.button(Button::Left, Press);
    thread::sleep(Duration::from_millis(1000));
    let _ = enigo.button(Button::Left, Release);
    thread::sleep(Duration::from_millis(1000));
}
pub fn sell(is_sell: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, time_key: Arc<Mutex<u8>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let target_window_name = "roblox";

    let mut last_sell_time = Instant::now() - Duration::from_mins(*time_key.lock() as u64);
    loop {
        if is_sell.load(sync::atomic::Ordering::Relaxed) {
            if let Ok(active_win) = get_active_window() {
                let title = active_win.title.to_lowercase();
                if title.contains(target_window_name) {
                    if last_sell_time.elapsed() >= Duration::from_mins(*time_key.lock() as u64) {
                        if is_busy
                            .compare_exchange(
                                false,
                                true,
                                sync::atomic::Ordering::Acquire,
                                sync::atomic::Ordering::Relaxed,
                            )
                            .is_ok()
                        {
                            thread::sleep(Duration::from_millis(1000));
                            let _ = enigo.key(Key::Unicode('1'), Click);
                            thread::sleep(Duration::from_millis(1000));
                            let _ = enigo.key(Key::Unicode('t'), Click);
                            thread::sleep(Duration::from_millis(1000));
                            let _ = smooth_move_mouse(&mut enigo, 730, 1050, 30, 20);
                            mouse_click_release();
                            let _ = smooth_move_mouse(&mut enigo, 1770, 490, 60, 40);
                            mouse_click_release();
                            let _ = smooth_move_mouse(&mut enigo, 1280, 900, 30, 20);
                            mouse_click_release();
                            let _ = smooth_move_mouse(&mut enigo, 1280, 1000, 30, 20);
                            mouse_click_release();
                            let _ = smooth_move_mouse(&mut enigo, 1050, 750, 30, 20);
                            mouse_click_release();
                            let _ = smooth_move_mouse(&mut enigo, 1850, 300, 30, 20);
                            mouse_click_release();
                            let _ = enigo.key(Key::Unicode('1'), Click);
                            last_sell_time = Instant::now();
                            is_busy.store(false, sync::atomic::Ordering::Relaxed);
                        } else {
                            thread::sleep(Duration::from_millis(100));
                        }
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                } else {
                    thread::sleep(Duration::from_millis(1000));
                }
            } else {
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
