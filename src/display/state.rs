use std::sync::{Arc, Mutex};

use ratatui::widgets::ScrollbarState;

/// Contains the current ui state of the application.
///
/// To create a shareable reference to an instance of this struct, use
/// `new_shared()`, which will create an arcmutex around a new struct instance.
pub struct UIState {
    pub(crate) current_tab: usize,
    pub(crate) cpu_scroll: u16,
}

impl UIState {
    /// Instantiate a new instance of this struct with default values.
    pub(crate) fn new() -> Self {
        UIState {
            current_tab: 0,
            cpu_scroll: 0,
        }
    }

    /// Instantiate a new instance of this struct, and wrap it in an
    /// arcmutex.
    pub(crate) fn new_shared() -> Arc<Mutex<UIState>> {
        Arc::new(Mutex::new(Self::new()))
    }
}
