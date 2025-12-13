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
    pub fn smooth_move(
        &mut self,
        target_x: i32,
        target_y: i32,
        steps: i32,
        delay_ms: u64,
    ) -> Result<(), Box<dyn Error>> {
        let window = match get_active_window() {
            Ok(win) => win,
            Err(_) => return Err("Brak aktywnego okna".into()),
        };
        let mut rng = rand::rng();

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

        let (start_x, start_y) = self.enigo.location()?;

        let dx = final_target_x - start_x as f64;
        let dy = final_target_y - start_y as f64;

        let step_x_f = dx / steps as f64;
        let step_y_f = dy / steps as f64;

        for i in 1..=steps {
            let current_x = start_x as f64 + step_x_f * i as f64;
            let current_y = start_y as f64 + step_y_f * i as f64;

            self.enigo.move_mouse(
                current_x.round() as i32,
                current_y.round() as i32,
                Coordinate::Abs,
            )?;

            thread::sleep(Duration::from_millis(delay_ms));
        }

        let wiggle_pixels = rng.random_range(10..80) as f64;
        let wiggle_steps = 10;

        for i in 1..=wiggle_steps {
            let offset = (wiggle_pixels / wiggle_steps as f64) * i as f64;
            self.enigo.move_mouse(
                (final_target_x + offset).round() as i32,
                final_target_y.round() as i32,
                Coordinate::Abs,
            )?;
            thread::sleep(Duration::from_millis(10));
        }

        for i in 1..=wiggle_steps {
            let offset = wiggle_pixels - ((wiggle_pixels / wiggle_steps as f64) * i as f64);
            self.enigo.move_mouse(
                (final_target_x + offset).round() as i32,
                final_target_y.round() as i32,
                Coordinate::Abs,
            )?;
            thread::sleep(Duration::from_millis(10));
        }

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
    let mut bot = MacroBot::new();
    let mut last_potion_time = Instant::now() - Duration::from_secs(300);

    loop {
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

        let _ = bot.smooth_move(730, 1050, 30, 20);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1770, 490, 60, 40);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1280, 900, 30, 20);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1280, 1000, 30, 20);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1050, 750, 30, 20);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.smooth_move(1850, 300, 30, 20);
        thread::sleep(Duration::from_millis(500));
        let _ = bot.click_release();

        let _ = bot.key_click('1');

        last_sell_time = Instant::now();
        is_busy.store(false, sync::atomic::Ordering::Relaxed);
    }
}
