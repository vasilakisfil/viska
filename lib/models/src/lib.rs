macro_rules! header {
    ($iter:expr, $header:path, $error:expr) => {
        $iter
            .find_map(|header| {
                if let $header(header) = header {
                    Some(header)
                } else {
                    None
                }
            })
            .ok_or($error)
    };
}

macro_rules! named_header_param {
    ($header:expr, $param:expr, $error:expr) => {
        if let Ok(header) = $header {
            if let Some(Some(param)) = header.parameters.get($param) {
                Ok(param)
            } else {
                Err($error)
            }
        } else {
            Err($error)
        }
    };
}

macro_rules! named_header_username {
    ($header:expr, $error:expr) => {
        if let Ok(header) = $header {
            if let Some(auth) = &header.uri.auth {
                Ok(&auth.username)
            } else {
                Err($error)
            }
        } else {
            Err($error)
        }
    };
}

mod dialog;
mod error;
mod registration;
mod request;
mod response;
pub mod transactions;

pub use dialog::{Dialog, DialogFlow};
pub use error::Error;
pub use registration::{Registration, UpdateRegistration};
pub use request::Request;
pub use response::Response;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TransportType {
    Tcp,
    Udp,
}
