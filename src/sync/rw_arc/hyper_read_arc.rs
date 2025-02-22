use super::RwArcInner;
use crate::prelude::{ops::Deref, sync::Arc};

pub struct HyperReadArc<T: ?Sized + Clone, R> {
    #[allow(unused)]
    pub(super) inner: Arc<RwArcInner<T, R>>,
    pub(super) data: T,
}

impl<T: ?Sized + Clone, R> Deref for HyperReadArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        &self.data
    }
}
