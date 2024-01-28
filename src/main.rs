use std::error::Error;

use mainframe::app;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(app::run()?)
}
