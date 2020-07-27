use common::{
    chrono::{DateTime, Duration, Utc},
    ipnetwork::{IpNetwork, Ipv4Network},
};
use std::{convert::TryFrom, net::Ipv4Addr};

#[derive(Debug, Clone)]
pub struct Registration {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub username: String,
    pub domain: Option<String>,
    pub contact: String,
    pub expires: DateTime<Utc>,
    pub call_id: String,
    pub cseq: i32,
    pub user_agent: String,
    pub instance: Option<String>,
    pub reg_id: i32,
    pub ip_address: IpNetwork,
    pub port: i16,
    pub transport: crate::TransportType,
}

pub struct UpdateRegistration {
    pub username: String,
    pub domain: Option<String>,
    pub contact: String,
    pub expires: Option<DateTime<Utc>>,
    pub call_id: String,
    pub cseq: i32,
    pub user_agent: String,
    pub instance: Option<String>,
    pub reg_id: Option<i32>,
    pub ip_address: IpNetwork,
    pub port: i16,
    pub transport: crate::TransportType,
}

impl TryFrom<crate::Request> for UpdateRegistration {
    type Error = String;

    fn try_from(request: crate::Request) -> Result<Self, Self::Error> {
        Ok(Self {
            username: request
                .from_header_username()
                .map_err(|e| format!("{:?}", e))?
                .clone(),
            domain: Some(
                request
                    .from_header_domain()
                    .map_err(|e| format!("{:?}", e))?
                    .clone()
                    .to_string(),
            ),
            contact: request
                .contact_header()
                .map_err(|e| format!("{:?}", e))?
                .clone()
                .to_string(),
            expires: Some(
                Utc::now()
                    + Duration::seconds(
                        request
                            .contact_header_expires()
                            .unwrap_or(request.expires_header().map_err(|e| format!("{:?}", e))?)
                            as i64,
                    ),
            ),
            call_id: request.call_id().map_err(|e| format!("{:?}", e))?.clone(),
            cseq: request.cseq().map_err(|e| format!("{:?}", e))?.0 as i32,
            user_agent: request
                .user_agent()
                .map_err(|e| format!("{:?}", e))?
                .clone(),
            instance: Some(
                request
                    .contact_header_instance()
                    .map_err(|e| format!("{:?}", e))?
                    .to_string(),
            ),
            ip_address: IpNetwork::V4(
                Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 3), 32).map_err(|e| e.to_string())?,
            ),
            port: 5066,
            transport: crate::TransportType::Udp,
            reg_id: None,
        })
    }
}
