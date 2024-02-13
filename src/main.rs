#![deny(unused_extern_crates)]

use std::error::Error;

use clap::Parser;

use mainframe::app::MainFrameApp;
use mainframe::cli;
use mainframe::panic_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    panic_handler::init();

    let opts = cli::MainframeOpts::parse();

    let app = MainFrameApp::new()
        .with_poll_rate(opts.poll_rate)
        .with_refresh_rate(opts.refresh_rate);

    app.run().await.unwrap();

    Ok(())
}
