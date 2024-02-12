use std::error::Error;

use mainframe::app::MainFrameApp;
use mainframe::panic_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    panic_handler::init();

    let app = MainFrameApp::new()
        .with_poll_rate(2.0)
        .with_refresh_rate(20.0);

    app.run().await.unwrap();

    Ok(())
}
