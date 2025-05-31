pub fn init_logging() {
    #[cfg(debug_assertions)]
    match log4rs::init_file("./logging/debug.yml", Default::default()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    }

    #[cfg(not(debug_assertions))]
    match log4rs::init_file("./logging/release.yml", Default::default()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
