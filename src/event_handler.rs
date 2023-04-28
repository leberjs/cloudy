use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CtEvent, KeyCode, KeyEvent};

use crate::app::{App, AppResult, InputMode};
use crate::widgets::profile_selection;

pub enum Event {
    Tick,
    KeyPress(KeyEvent),
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    tx: mpsc::Sender<Event>,
    tick_rate: u64,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            rx,
            tx,
            tick_rate: 200,
        }
    }

    pub fn init(&self) {
        let tick_rate = Duration::from_millis(self.tick_rate);
        let tx = self.tx.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events") {
                    match event::read().expect("unable to read event") {
                        CtEvent::Key(e) => tx.send(Event::KeyPress(e)),
                        _ => Ok(()),
                    }
                    .expect("failed to send event")
                }

                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).ok();
                    last_tick = Instant::now();
                }
            }
        });
    }

    pub fn next(&self) -> AppResult<Event> {
        Ok(self.rx.recv()?)
    }
}

pub async fn on_key_press_event(key_press_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Help => match key_press_event.code {
            KeyCode::Esc => app.show_help(),
            _ => {}
        },
        InputMode::Normal => match key_press_event.code {
            // Quit app
            KeyCode::Esc => app.quit(),
            // Show help
            KeyCode::Char('?') => app.show_help(),
            // Show profile selection
            KeyCode::Char('p') => app.show_profile_selection(),
            // Log Block navigation
            KeyCode::Down => app.current_log_display.next(),
            KeyCode::Up => app.current_log_display.previous(),
            _ => {}
        },
        InputMode::ProfileSelection => match key_press_event.code {
            KeyCode::Enter => profile_selection::select_profile(app).await,
            KeyCode::Esc => app.show_profile_selection(),
            KeyCode::Down => app.profile_list.next(),
            // KeyCode::Left => app.profile_list.unselect(),
            KeyCode::Up => app.profile_list.previous(),
            _ => {}
        },
    }

    Ok(())
}
