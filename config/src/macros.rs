#[macro_export]
macro_rules! config {
    ($p:expr) => {{
        let volume = Config::load($p.into());
        volume
    }};
}
