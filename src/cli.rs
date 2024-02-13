use clap::{arg, Parser};

static ABOUT_STRING: &str = "
A fast and lightweight visual system monitor.
Reports live data about cpu and gpu usage, temperature, memory consumption, 
and more.

To configure poll rate, and refresh rate, see the available option flags below.";

#[derive(Parser, Debug)]
#[command(author = "mlafrance")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = ABOUT_STRING)]
pub struct MainframeOpts {
    #[arg(short, long, default_value = "3", help = "Set the poll rate in hz.")]
    pub poll_rate: f32,

    #[arg(
        short,
        long,
        default_value = "20",
        help = "Set the refresh rate in hz."
    )]
    pub refresh_rate: f32,
}
