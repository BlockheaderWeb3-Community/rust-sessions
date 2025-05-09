use log::{debug, error, info, trace, warn};
// use tracing::{warn, info, debug, error, info_span, trace};
// use tracing_subscriber;

pub fn log() {
    env_logger::init();

    info!("Application starting up");

    if let Err(e) = do_something() {
        error!("Operation failed: {}", e);
    }

    debug!("Detailed state information: {:?}", get_state());

    fn do_something() -> Result<String, String> {
        Ok("this is a string".to_string())
    }

    fn get_state() -> String {
        "hello world".to_string()
    }

    warn!("Resource usage is high");
    trace!("this is trace");

    // tracing();
}

// fn tracing() {
//     tracing_subscriber::fmt::init();

//     let span = info_span!("processing", user_id = 42);
//     let _guard = span.enter();

//     info!(task = "initialization", "Application starting up");

//     if let Err(e) = do_something() {
//         error!("Operation failed {}", e);
//     }

//     fn do_something() -> Result<String, String> {
//         Ok("this is a string".to_string())
//     }
// }
