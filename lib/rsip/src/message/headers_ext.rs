use crate::{
    error::Header as ErrorHeader,
    headers::{self, Header},
    Error, Request, Response, SipMessage,
};

pub trait HeadersExt {
    fn to_header(&self) -> Result<&headers::To, Error>;
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error>;

    fn from_header(&self) -> Result<&headers::From, Error>;
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error>;

    fn via_header(&self) -> Result<&headers::Via, Error>;
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error>;

    fn call_id_header(&self) -> Result<&headers::CallId, Error>;
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error>;

    fn cseq_header(&self) -> Result<&headers::CSeq, Error>;
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error>;

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error>;
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error>;

    fn contact_header(&self) -> Result<&headers::Contact, Error>;
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error>;

    fn user_agent_header(&self) -> Result<&headers::UserAgent, Error>;

    fn dialog_id(&self) -> Option<String>;
}

impl HeadersExt for Request {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(
            self.headers().iter(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(
            self.headers().iter(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(
            self.headers().iter(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(
            self.headers().iter(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(
            self.headers().iter(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(
            self.headers().iter(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(
            self.headers().iter(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }

    fn user_agent_header(&self) -> Result<&headers::UserAgent, Error> {
        header!(
            self.headers().iter(),
            Header::UserAgent,
            Error::MissingHeader(ErrorHeader::UserAgent)
        )
    }

    fn dialog_id(&self) -> Option<String> {
        let (call_id, from_tag, to_tag): (Option<String>, Option<String>, Option<String>) = (
            self.call_id_header().ok().cloned().map(Into::into),
            self.from_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
            self.to_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
        );

        match (call_id, from_tag, to_tag) {
            (Some(call_id), Some(from_tag), Some(to_tag)) => {
                Some(format!("{}-{}-{}", call_id, from_tag, to_tag))
            }
            _ => None,
        }
    }
}

impl HeadersExt for Response {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(
            self.headers().iter(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(
            self.headers().iter(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(
            self.headers().iter(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(
            self.headers().iter(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(
            self.headers().iter(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(
            self.headers().iter(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(
            self.headers().iter(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }

    fn user_agent_header(&self) -> Result<&headers::UserAgent, Error> {
        header!(
            self.headers().iter(),
            Header::UserAgent,
            Error::MissingHeader(ErrorHeader::UserAgent)
        )
    }
    fn dialog_id(&self) -> Option<String> {
        let (call_id, from_tag, to_tag): (Option<String>, Option<String>, Option<String>) = (
            self.call_id_header().ok().cloned().map(Into::into),
            self.from_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
            self.to_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
        );

        match (call_id, from_tag, to_tag) {
            (Some(call_id), Some(from_tag), Some(to_tag)) => {
                Some(format!("{}-{}-{}", call_id, from_tag, to_tag))
            }
            _ => None,
        }
    }
}

impl HeadersExt for SipMessage {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(
            self.headers().iter(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(
            self.headers().iter(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(
            self.headers().iter(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(
            self.headers().iter(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(
            self.headers().iter(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(
            self.headers().iter(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(
            self.headers().iter(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }

    fn user_agent_header(&self) -> Result<&headers::UserAgent, Error> {
        header!(
            self.headers().iter(),
            Header::UserAgent,
            Error::MissingHeader(ErrorHeader::UserAgent)
        )
    }
    fn dialog_id(&self) -> Option<String> {
        let (call_id, from_tag, to_tag): (Option<String>, Option<String>, Option<String>) = (
            self.call_id_header().ok().cloned().map(Into::into),
            self.from_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
            self.to_header()
                .ok()
                .map(|h| h.tag())
                .flatten()
                .cloned()
                .map(Into::into),
        );

        match (call_id, from_tag, to_tag) {
            (Some(call_id), Some(from_tag), Some(to_tag)) => {
                Some(format!("{}-{}-{}", call_id, from_tag, to_tag))
            }
            _ => None,
        }
    }
}