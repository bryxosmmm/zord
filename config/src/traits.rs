pub trait ConfigStruct {
    fn load(_: String) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized + Default,
    {
        Ok(Self::default())
    }

    fn healthcheck(&self) -> bool {
        true
    }
}
