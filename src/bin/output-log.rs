use log::{info, warn};

fn main() {
    // use env RUST_LOG=info cargo run --bin output-log

    // Warnings and info are very, very, very helpful for your future self
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented");
}