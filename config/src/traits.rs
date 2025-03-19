pub trait ConfigStruct {
    fn load(path: String) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;

    fn healthcheck(&self) -> bool;
}
