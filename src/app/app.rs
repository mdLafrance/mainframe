use ratatui::{
    widgets::{Block, Borders},
    Frame,
};
use systemstat::Duration;

use crate::display::ui::{init_ui, shutdown_ui};

use std::error::Error;

enum MFAMessage {
    Exit,
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

    pub fn new() -> Self {
        MainFrameApp {
            frame_rate: 30,
            tick_rate: 10,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        // Setup
        let mut terminal = init_ui()?;

        // --- Init sync primitives --- //
        // Sender and receiver provide a message passing service to pass flags
        // to the render thread.
        let (ui_tx, mut ui_rx) = tokio::sync::mpsc::unbounded_channel::<MFAMessage>();

        let mut tick_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.tick_rate as f64));

        let mut redraw_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / self.frame_rate as f64));

        // Launch ui thread
        let ui_thread = tokio::spawn(async move {
            // Sleep until next redraw timer
            redraw_interval.tick().await;

            // See if there are pending messages from host thread
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
            terminal.draw(|f| draw(f));
        });

        let mut x = 0;
        // Run main processing loop
        loop {
            tick_interval.tick().await;

            x += 1;

            if x > 20 {
                ui_tx.send(MFAMessage::Exit)?;
                break;
            }
        }

        ui_thread.await?;

        // Teardown
        shutdown_ui()?;
        Ok(())
    }
}

fn draw(f: &mut Frame) {
    f.render_widget(Block::new().title("asdf").borders(Borders::ALL), f.size());
}
