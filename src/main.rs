fn main() {
    #[cfg(feature = "enable")]
    tracy_client::Client::start();
    panic!("Hello, world!");
}
