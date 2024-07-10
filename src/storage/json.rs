pub trait JsonExt {
    type Error;
    fn get_json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Self::Error>;
    fn set_json<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error>;
}
