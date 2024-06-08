fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    #[cfg(feature = "enable")]
    tracy_client::Client::start();
    panic!("Hello, world!");
}
