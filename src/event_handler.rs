use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CtEvent, KeyCode, KeyEvent};

use crate::app::{App, AppResult, InputMode};

pub enum Event {
    Tick,
    KeyPress(KeyEvent),
}

pub struct EventHandler {
    receiver: mpsc::Receiver<Event>,
    sender: mpsc::Sender<Event>,
    tick_rate: u64,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            receiver: rx,
            sender: tx,
            tick_rate: 100,
        }
    }

    pub fn init(&self) {
        let tick_rate = Duration::from_millis(self.tick_rate);
        let sender = self.sender.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events") {
                    match event::read().expect("cannot read event") {
                        CtEvent::Key(e) => sender.send(Event::KeyPress(e)),
                        _ => Ok(()),
                    }
                    .expect("failed to send event")
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).ok();
                    last_tick = Instant::now();
                }
            }
        });
    }

    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}

pub fn on_key_press_event(key_press_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_press_event.code {
            // Quit app
            KeyCode::Esc => app.quit(),
            // Show profile selection
            KeyCode::Char('p') => app.show_profile_selection(),
            _ => {}
        },
        InputMode::ProfileSelection => match key_press_event.code {
            KeyCode::Enter => app.select_profile(),
            KeyCode::Esc => app.show_profile_selection(),
            KeyCode::Down => app.profile_list.next(),
            KeyCode::Left => app.profile_list.unselect(),
            KeyCode::Up => app.profile_list.previous(),
            _ => {}
        },
    }

    Ok(())
}
