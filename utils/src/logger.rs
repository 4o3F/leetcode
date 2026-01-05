use tracing::Level;
use tracing_unwrap::ResultExt;

pub fn init_logger() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect_or_log("Init tracing failed");
}
