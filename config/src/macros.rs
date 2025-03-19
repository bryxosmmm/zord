#[macro_export]
macro_rules! config {
    ($p:literal) => {{
        let volume = Volumes::load($p.into());
        volume
    }};
}
