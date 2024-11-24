use crate::net::socket_addr::SocketAddr;

pub struct Bindable<S> {
    bind: SocketAddr,
    data: S,
}

impl<S> Bindable<S> {
    pub fn bind(&self) -> &SocketAddr {
        &self.bind
    }

    pub fn bind_owned(&self) -> SocketAddr {
        (&self.bind).to_owned()
    }

    pub fn data(&self) -> &S {
        &self.data
    }

    pub fn into_data(self) -> S {
        self.data
    }
}

impl<S: Clone> Bindable<S> {
    pub fn data_owned(&self) -> S {
        self.data.clone()
    }
}

impl<S> TryFrom<(SocketAddr, S)> for Bindable<S> {
    type Error = BindableError;
    fn try_from((bind, data): (SocketAddr, S)) -> Result<Self, Self::Error> {
        if bind.has_valid_nonglobal_binding() {
            return Err(BindableError::InvalidSocketBindAddr);
        }
        Ok(Bindable { bind, data })
    }
}

pub enum BindableError {
    InvalidSocketBindAddr,
}
