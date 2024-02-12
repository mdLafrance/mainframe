use std::error::Error;

use mainframe::app::MainFrameApp;
use mainframe::panic_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    panic_handler::init();

    let app = MainFrameApp::new()
        .with_poll_rate(10f32)
        .with_refresh_rate(30f32);

    app.run().await.unwrap();

    Ok(())
}
