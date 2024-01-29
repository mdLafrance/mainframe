use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use futures::StreamExt;
use scopeguard::guard;

use ratatui::{
    widgets::{Block, Borders},
    Frame,
};
use systemstat::Duration;

use crate::{
    display::ui::{init_ui, shutdown_ui},
    errors::MFError,
};

use std::error::Error;

enum MFAMessage {
    Exit,
}

enum MFAAppEvent {
    KeyPress,
    Quit,
}

struct MFAUiState {}

pub struct MainFrameApp {
    frame_rate: usize,
    tick_rate: usize,
}

impl MainFrameApp {
    /// Set the ui refresh interval for the app instance.
    ///
    /// Interval is taken in hz - refreshes per second.
    /// An app with a 60 fps refresh rate would supply interval=60
    pub fn with_frame_rate(mut self, interval: usize) -> Self {
        self.frame_rate = interval;

        self
    }

    /// Set the event polling interval for the app instance.
    ///
    /// Interval is taken in hz - refreshes per second.
    pub fn with_tick_rate(mut self, interval: usize) -> Self {
        self.tick_rate = interval;

        self
    }

    /// Intstantiate a new app instance.
    ///
    /// A new instance of mainframe app has not acquired any resources, nor
    /// taken control of the terminal yet. New app instances should be modified
    /// before the call to `run()`, at which point event and render resources
    /// are acquired.
    pub fn new() -> Self {
        MainFrameApp {
            frame_rate: 20,
            tick_rate: 10,
        }
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
        // Message channel for the draw thread
        let (ui_tx, mut ui_rx) = tokio::sync::mpsc::unbounded_channel::<MFAMessage>();

        // Message channel for the event decoder thread
        let (evt_tx, mut evt_rt) = tokio::sync::mpsc::unbounded_channel::<MFAAppEvent>();

        let mut tick_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.tick_rate as f64));

        let mut redraw_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.frame_rate as f64));

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

                // Draw ui elements
                // terminal.draw(|f| draw(f));
                println!("Redraw");
            }
        });

        let mut events = EventStream::new();

        // Run main processing loop
        'mainloop: loop {
            match events.next().await {
                Some(Ok(Event::Key(evnt))) => match evnt.code {
                    KeyCode::Char('q') => {
                        break 'mainloop;
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

fn draw(f: &mut Frame) {
    f.render_widget(Block::new().title("asdf").borders(Borders::ALL), f.size());
}
