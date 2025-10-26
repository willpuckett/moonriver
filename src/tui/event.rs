use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::Duration;
use tokio::time::interval;

/// Events that can occur in the TUI
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal tick for regular updates
    Tick,
    /// Key press event
    Key(KeyEvent),
    /// Mouse event (click, scroll, etc.)
    Mouse(MouseEvent),
    /// Terminal resize event
    Resize(u16, u16),
}

/// Event handler that polls for terminal events
pub struct EventHandler {
    tick_interval: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        EventHandler {
            tick_interval: tick_rate,
        }
    }

    /// Get the next event, blocking until one is available
    pub async fn next(&mut self) -> Option<Event> {
        let mut ticker = interval(self.tick_interval);
        
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    return Some(Event::Tick);
                }
                result = tokio::task::spawn_blocking(|| {
                    crossterm::event::poll(Duration::from_millis(0))
                }) => {
                    if let Ok(Ok(true)) = result {
                        if let Ok(event) = crossterm::event::read() {
                            match event {
                                CrosstermEvent::Key(key) => return Some(Event::Key(key)),
                                CrosstermEvent::Mouse(mouse) => return Some(Event::Mouse(mouse)),
                                CrosstermEvent::Resize(w, h) => return Some(Event::Resize(w, h)),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
