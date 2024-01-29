use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;

use systemstat::Duration;

use std::error::Error;

use crate::display::state::UIState;
use crate::display::ui::{draw, init_ui, shutdown_ui};

enum MFAMessage {
    Exit,
}

enum MFAAppEvent {
    KeyPress,
    Quit,
}

pub struct MainFrameApp {
    refresh_rate: usize,
}

impl MainFrameApp {
    /// Set the ui refresh interval for the app instance.
    ///
    /// Interval is taken in hz - refreshes per second.
    /// An app with a 60 fps refresh rate would supply interval=60
    pub fn with_refresh_rage(mut self, hz: usize) -> Self {
        self.refresh_rate = hz;

        self
    }

    /// Intstantiate a new app instance.
    ///
    /// A new instance of mainframe app has not acquired any resources, nor
    /// taken control of the terminal yet. New app instances should be modified
    /// before the call to `run()`, at which point event and render resources
    /// are acquired.
    pub fn new() -> Self {
        MainFrameApp { refresh_rate: 20 }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        // Setup
        let mut terminal = init_ui()?;

        defer! {
            // Teardown
            // NOTE: UI shutdown must happen after every other event and
            // terminal call.
            shutdown_ui().unwrap();
        }

        // --- Init sync primitives --- //
        // ui state
        let app_state = UIState::new_shared();
        let _app_state_handle = app_state.clone();

        // Message channel for the draw thread
        let (ui_tx, mut ui_rx) = tokio::sync::mpsc::unbounded_channel::<MFAMessage>();

        let mut redraw_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.refresh_rate as f64));

        // Launch ui thread
        let ui_thread = tokio::spawn(async move {
            loop {
                // Suspend until next redraw timer
                redraw_interval.tick().await;

                // Consume messages from host thread
                let msg = match ui_rx.try_recv() {
                    Ok(x) => Some(x),
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => None,
                    Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                        return;
                    }
                };

                match msg {
                    Some(MFAMessage::Exit) => {
                        return;
                    }
                    _ => (),
                };

                {
                    let s = _app_state_handle.lock().unwrap();

                    // Draw ui elements
                    terminal.draw(|f| draw(&s, f)).unwrap();
                }
            }
        });

        let mut events = EventStream::new();

        // Run main processing loop
        'mainloop: loop {
            // Consume pending events
            match events.next().await {
                Some(Ok(Event::Key(evnt))) => match evnt.code {
                    // Quit key
                    KeyCode::Char('q') => {
                        break 'mainloop;
                    }
                    // Tab selection keys
                    KeyCode::Char('h') => {
                        app_state.lock().unwrap().current_tab = 0;
                    }
                    KeyCode::Char('u') => {
                        app_state.lock().unwrap().current_tab = 1;
                    }
                    KeyCode::Char('d') => {
                        app_state.lock().unwrap().current_tab = 2;
                    }
                    _ => (),
                },
                Some(Err(e)) => return Err(Box::new(e)),
                None => break 'mainloop,
                _ => (),
            };
        }

        ui_thread.abort();

        Ok(())
    }
}
