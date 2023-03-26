use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use cloudy::app::{App, AppResult};
use cloudy::event_handler::{on_key_press_event, Event, EventHandler};
use cloudy::ui::Ui;

fn main() -> AppResult<()> {
    // Create application
    let mut app = App::new();

    if app.profile_set.profiles.len() == 0 {
        eprintln!("[Error] need at least one AWS profile to work");
        std::process::exit(1)
    }

    // for profile in app.profile_set.profiles {
    //     println!("{}", profile.name)
    // }

    // Initialize terminal ui
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut ui = Ui::new(terminal);
    let event_handler = EventHandler::new();

    ui.init()?;
    event_handler.init();

    app.init();

    // Start loop
    while app.is_running() {
        ui.draw(&mut app)?;
        match event_handler.next()? {
            Event::KeyPress(e) => on_key_press_event(e, &mut app)?,
            Event::Tick => {}
        }
    }

    ui.quit()?;

    Ok(())
}
