pub trait YamlExt {
    type Error;
    fn get_yaml<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Self::Error>;
    fn set_yaml<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error>;
}
