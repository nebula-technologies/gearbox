use crate::service::discovery::discovery_type::broadcaster::Broadcaster;
use crate::service::discovery::entity::Advertisement;
use bytes::Bytes;

pub trait AdvertisementTransformer<A>
where
    A: Clone,
    Self: Sized,
{
    fn with_advertisement(self, a: Option<A>) -> Self;
    fn set_advertisement(&mut self, a: A) -> &mut Self;
    fn advertisement(&self) -> Option<&A>;

    fn advert_from_bytes(&mut self, a: Bytes) -> &Self;
    fn advert_into_bytes(&self) -> Bytes;
}

impl AdvertisementTransformer<Advertisement> for Broadcaster<Advertisement> {
    fn with_advertisement(mut self, a: Option<Advertisement>) -> Self {
        self.advertisement = a.map(|t| t.into());
        self
    }
    fn set_advertisement(&mut self, a: Advertisement) -> &mut Self {
        self.advertisement = Some(a);
        self
    }
    fn advertisement(&self) -> Option<&Advertisement> {
        self.advertisement.as_ref()
    }
    fn advert_from_bytes(&mut self, a: Bytes) -> &Self {
        self.advertisement = Advertisement::try_from(a).ok();
        self
    }
    fn advert_into_bytes(&self) -> Bytes {
        self.advertisement
            .as_ref()
            .map(|t| t.to_string().into())
            .unwrap_or_default()
    }
}
impl AdvertisementTransformer<Bytes> for Broadcaster<Bytes> {
    fn with_advertisement(mut self, a: Option<Bytes>) -> Self {
        self.advertisement = a.map(|t| t.into());
        self
    }
    fn set_advertisement(&mut self, a: Bytes) -> &mut Self {
        self.advertisement = Some(a);
        self
    }
    fn advertisement(&self) -> Option<&Bytes> {
        self.advertisement.as_ref()
    }
    fn advert_from_bytes(&mut self, a: Bytes) -> &Self {
        self.advertisement = Some(a);
        self
    }
    fn advert_into_bytes(&self) -> Bytes {
        self.advertisement.clone().unwrap_or_default()
    }
}
