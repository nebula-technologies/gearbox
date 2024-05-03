pub trait TryDefault {
    type Error;
    fn try_default() -> Result<Self, Self::Error>
    where
        Self: Sized;
}
