pub trait Collector {
    type Inner;
    fn inner(&self) -> Self::Inner;
    fn collect(ft: &str) -> Self;
    fn mutate(&self, other: &Self) -> Self;
}
