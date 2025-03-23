#[macro_export]
macro_rules! config {
    ($p:literal) => {{
        let volume = Config::load($p.into());
        volume
    }};
}
