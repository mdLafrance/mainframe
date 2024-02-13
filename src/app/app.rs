use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;

use systemstat::Duration;

use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::display::state::UIState;
use crate::display::ui::{draw, init_ui, shutdown_ui};

use crate::monitoring::polling::{SystemPollResult, SystemPoller, SystemPollerTarget};
use crate::monitoring::system::SystemData;
use crate::ringbuffer::RingBuffer;

enum MFAMessage {
    Exit,
}

pub struct MainFrameApp {
    refresh_rate: f32,
    poll_rate: f32,
}

impl MainFrameApp {
    /// Set the ui refresh interval for the app instance.
    ///
    /// Interval is taken in hz - refreshes per second.
    /// An app with a 60 fps refresh rate would supply interval=60
    pub fn with_refresh_rate(mut self, hz: f32) -> Self {
        self.refresh_rate = hz;

        self
    }

    /// Set the data poll interval for the app instance.
    ///
    /// Interval is taken in hz - refreshes per second.
    /// An app with a 60 fps refresh rate would supply interval=60
    pub fn with_poll_rate(mut self, hz: f32) -> Self {
        self.poll_rate = hz;

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
            refresh_rate: 20.0,
            poll_rate: 1.0,
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
        // ui state
        let app_state = UIState::new_shared();
        let app_data = Arc::new(Mutex::new(SystemData::new_from_poll()));

        let poll_results = Arc::new(Mutex::new(RingBuffer::<SystemPollResult>::new(1)));

        let mut system_poller = SystemPoller::new().with_poll_targets(vec![
            SystemPollerTarget::CpuUsage,
            SystemPollerTarget::CpuTemperature,
            SystemPollerTarget::Gpu,
            SystemPollerTarget::Memory,
        ]);

        poll_results.lock().unwrap().add(system_poller.poll());

        let _app_state_handle = app_state.clone();
        let _app_data_handle = app_data.clone();
        let _poll_result_handle_poll_thread = poll_results.clone();
        let _poll_result_handle_draw_thread = poll_results.clone();

        // Message channel for the draw thread
        let (ui_tx, mut ui_rx) = tokio::sync::mpsc::unbounded_channel::<MFAMessage>();

        let mut redraw_interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / self.refresh_rate));

        let mut polling_interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / self.poll_rate));

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
                    let mut s = _app_state_handle.lock().unwrap();
                    let d = _app_data_handle.lock().unwrap();
                    let r = _poll_result_handle_draw_thread.lock().unwrap();

                    // Draw ui elements
                    terminal.draw(|f| draw(&mut s, &d, &r, f)).unwrap();
                }
            }
        });

        // Launch polling thread
        let polling_thread = tokio::spawn(async move {
            loop {
                polling_interval.tick().await;

                let poll_result = system_poller.poll();

                {
                    let mut p = _poll_result_handle_poll_thread.lock().unwrap();

                    p.add(poll_result);
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
                        ui_tx.send(MFAMessage::Exit).unwrap();
                        break 'mainloop;
                    }
                    // Tab selection keys
                    // KeyCode::Char('h') => {
                    //     app_state.lock().unwrap().current_tab = 0;
                    // }
                    // KeyCode::Char('u') => {
                    //     app_state.lock().unwrap().current_tab = 1;
                    // }
                    // KeyCode::Char('d') => {
                    //     app_state.lock().unwrap().current_tab = 2;
                    // }
                    _ => (),
                },
                Some(Err(e)) => return Err(Box::new(e)),
                None => break 'mainloop,
                _ => (),
            };
        }

        ui_thread.abort();
        polling_thread.abort();

        Ok(())
    }
}
