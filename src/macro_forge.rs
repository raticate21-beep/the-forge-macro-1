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

pub const TARGET_WINDOW_NAME: &str = "roblox";

pub struct MacroBot {
    enigo: Enigo,
}

impl MacroBot {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).expect("Not found a pointer!");
        Self { enigo }
    }
    pub fn click_release(&mut self) -> Result<(), Box<dyn Error>> {
        thread::sleep(Duration::from_millis(500));
        self.enigo.button(Button::Left, Press)?;
        thread::sleep(Duration::from_millis(500));
        self.enigo.button(Button::Left, Release)?;
        thread::sleep(Duration::from_millis(500));
        Ok(())
    }

    pub fn key_click(&mut self, key: char) -> Result<(), Box<dyn Error>> {
        self.enigo.key(Key::Unicode(key), Click)?;
        Ok(())
    }
    pub fn button_click(&mut self) -> Result<(), Box<dyn Error>> {
        self.enigo.button(Button::Left, Click)?;
        Ok(())
    }
    pub fn smooth_move(&mut self, target_x: i32, target_y: i32) -> Result<(), Box<dyn Error>> {
        let window = match get_active_window() {
            Ok(win) => win,
            Err(_) => return Err("Brak aktywnego okna".into()),
        };
        let mut rng = rand::rng();

        let delay_ms: u64 = rng.random_range(10..18);
        let steps: i32 = rng.random_range(15..25);
        let shake_parament = rng.random_range(4.0..10.0);

        let win_x = window.position.x as f64;
        let win_y = window.position.y as f64;
        let win_w = window.position.width as f64;
        let win_h = window.position.height as f64;

        let base_width: f64 = 2560.0;
        let base_height: f64 = 1440.0;

        let scale_x = win_w / base_width;
        let scale_y = win_h / base_height;

        let (start_x_i32, start_y_i32) = self.enigo.location()?;
        let start_x = start_x_i32 as f64;
        let start_y = start_y_i32 as f64;

        let final_target_x = win_x + (target_x as f64 * scale_x);
        let final_target_y = win_y + (target_y as f64 * scale_y);

        let dx = final_target_x - start_x;
        let dy = final_target_y - start_y;

        let control_x = start_x + (dx / 2.0) + rng.random_range(100..300) as f64;
        let control_y = start_y + (dy / 2.0) + rng.random_range(100..300) as f64;

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let one_minus_t = 1.0 - t;

            let next_x = (one_minus_t.powi(2) * start_x)
                + (2.0 * one_minus_t * t * control_x)
                + (t.powi(2) * final_target_x);

            let next_y = (one_minus_t.powi(2) * start_y)
                + (2.0 * one_minus_t * t * control_y)
                + (t.powi(2) * final_target_y);

            let jitter_x = rng.random_range(-shake_parament..shake_parament);
            let jitter_y = rng.random_range(-shake_parament..shake_parament);

            self.enigo.move_mouse(
                (next_x + jitter_x).round() as i32,
                (next_y + jitter_y).round() as i32,
                Coordinate::Abs,
            )?;

            thread::sleep(Duration::from_millis(delay_ms));
        }
        self.enigo.move_mouse(
            final_target_x.round() as i32,
            final_target_y.round() as i32,
            Coordinate::Abs,
        )?;
        thread::sleep(Duration::from_millis(delay_ms));
        Ok(())
    }
}
pub fn clicker(is_running: Arc<AtomicBool>, is_busy: Arc<AtomicBool>) {
    let mut bot = MacroBot::new();
    let mut rng = rand::rng();

    loop {
        if !is_running.load(sync::atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        if is_busy.load(sync::atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(500));
            continue;
        }

        let Ok(title) = get_active_window().map(|w| w.title.to_lowercase()) else {
            thread::sleep(Duration::from_millis(200));
            continue;
        };
        if !title.contains(TARGET_WINDOW_NAME) {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        let _ = bot.button_click();

        let random_delay = rng.random_range(300..450);
        thread::sleep(Duration::from_millis(random_delay));
    }
}

pub fn luck(is_luck: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, potion_key: Arc<Mutex<String>>) {
    const MAX_LUCK_ACTIONS: u8 = 10;

    let mut bot = MacroBot::new();
    let mut last_potion_time = Instant::now() - Duration::from_secs(300);
    let mut luck_actions = 0;

    loop {
        if !is_luck.load(sync::atomic::Ordering::Relaxed) {
            luck_actions = 0;
            last_potion_time = Instant::now() - Duration::from_secs(300);
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        if luck_actions >= MAX_LUCK_ACTIONS {
            is_luck.store(false, sync::atomic::Ordering::Relaxed);
            continue;
        }
        if !is_luck.load(sync::atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        let Ok(title) = get_active_window().map(|w| w.title.to_lowercase()) else {
            thread::sleep(Duration::from_millis(200));
            continue;
        };
        if !title.contains(TARGET_WINDOW_NAME) {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        if last_potion_time.elapsed() <= Duration::from_secs(300) {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        if is_busy
            .compare_exchange(
                false,
                true,
                sync::atomic::Ordering::Acquire,
                sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        let key_char = potion_key.lock().chars().next().unwrap_or('3');

        let _ = bot.key_click(key_char);
        thread::sleep(Duration::from_millis(100));

        let _ = bot.enigo.button(Button::Left, Press);
        thread::sleep(Duration::from_millis(2500));
        let _ = bot.enigo.button(Button::Left, Release);

        let _ = bot.key_click('1');

        last_potion_time = Instant::now();
        is_busy.store(false, sync::atomic::Ordering::Relaxed);
        luck_actions += 1;
    }
}

pub fn sell(is_sell: Arc<AtomicBool>, is_busy: Arc<AtomicBool>, time_key: Arc<Mutex<u8>>) {
    let mut bot = MacroBot::new();
    let mut last_sell_time = Instant::now() - Duration::from_mins(*time_key.lock() as u64);

    loop {
        if !is_sell.load(sync::atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        let Ok(title) = get_active_window().map(|w| w.title.to_lowercase()) else {
            thread::sleep(Duration::from_millis(200));
            continue;
        };
        if !title.contains(TARGET_WINDOW_NAME) {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        if last_sell_time.elapsed() <= Duration::from_mins(*time_key.lock() as u64) {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        if is_busy
            .compare_exchange(
                false,
                true,
                sync::atomic::Ordering::Acquire,
                sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        thread::sleep(Duration::from_millis(1000));

        let _ = bot.key_click('1');
        thread::sleep(Duration::from_millis(1000));

        let _ = bot.key_click('t');
        thread::sleep(Duration::from_millis(1000));

        let _ = bot.smooth_move(730, 1050);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1770, 490);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1280, 900);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1280, 1000);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1050, 750);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1850, 300);
        thread::sleep(Duration::from_millis(200));
        let _ = bot.click_release();

        let _ = bot.key_click('1');

        last_sell_time = Instant::now();
        is_busy.store(false, sync::atomic::Ordering::Relaxed);
    }
}
