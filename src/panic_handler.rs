pub fn init() {
    std::panic::set_hook(Box::new(|panic_info| {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .create_panic_handler()(panic_info);
        std::process::exit(-1);
    }));
}
