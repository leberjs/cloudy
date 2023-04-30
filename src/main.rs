use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use cloudy::app::{App, AppResult};
use cloudy::error::{self, ErrorType};
use cloudy::event::{Event, EventHandler};
use cloudy::handler::handle_key_events;
use cloudy::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create application
    let mut app = App::new();

    if app.profile_set.profiles.len() == 0 {
        let err = "Need at least one AWS profile to work";
        error::handle(err, ErrorType::Custom)
    }

    // Initialize terminal ui
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;
    app.init();

    // Start loop
    while app.state.is_running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit ui
    tui.exit()?;

    Ok(())
}
