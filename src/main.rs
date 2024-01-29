use std::error::Error;

use mainframe::app::MainFrameApp;
use mainframe::panic_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    panic_handler::init();

    let app = MainFrameApp::new();

    app.run().await.unwrap();

    Ok(())
}
