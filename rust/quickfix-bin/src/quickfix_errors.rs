use std::error::Error;
use std::fmt::{self, Formatter};


#[derive(Debug)]
pub struct FieldNotPresentError;

impl fmt::Display for FieldNotPresentError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Mandatory field not present")
    }
}

impl Error for FieldNotPresentError {
    fn description(&self) -> &str {
        "Mandatory field not present"
    }
}

#[derive(Debug)]
pub struct SessionLevelRejectErr {
    pub kind: SessionLevelRejectReason,
    pub source: Option<Box<dyn Error>>
}

impl SessionLevelRejectErr {
    pub fn invalid_value_for_tag_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::InvalidValueForTag,
            source: None
        }
    }

    pub fn invalid_tag_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::InvalidTag,
            source: None
        }
    }

    pub fn required_tag_missing_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::RequiredTagMissing,
            source: None
        }
    }

    pub fn undefined_tag_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::UndefinedTag,
            source: None
        }
    }

    pub fn tag_without_value_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::TagSpecifiedWithoutValue,
            source: None
        }
    }

    pub fn value_out_of_range_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::ValueOutOfRange,
            source: None
        }
    }

    pub fn incorrect_data_format_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::IncorrectDataFormat,
            source: None
        }
    }

    pub fn decryption_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::DecryptionProblem,
            source: None
        }
    }

    pub fn signature_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::SignatureProblem,
            source: None
        }
    }

    pub fn comp_id_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::CompIdProblem,
            source: None
        }
    }

    pub fn sending_time_accuracy_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::SendingTimeAccuracyProblem,
            source: None
        }
    }

    pub fn invalid_msg_type_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::InvalidMessageType,
            source: None
        }
    }

    pub fn invalid_body_len_err() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::InvalidBodyLength,
            source: None
        }
    }

    pub fn invalid_checksum() -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::InvalidChecksum,
            source: None
        }
    }

    pub fn parse_err(err: Option<Box<dyn Error>>) -> Self {
        SessionLevelRejectErr {
            kind: SessionLevelRejectReason::ParseError,
            source: err
        }
    }
}

#[derive(Debug)]
pub enum SessionLevelRejectReason {
    InvalidValueForTag,
    InvalidTag,
    RequiredTagMissing,
    UndefinedTag,
    TagSpecifiedWithoutValue,
    ValueOutOfRange,
    IncorrectDataFormat,
    DecryptionProblem,
    SignatureProblem,
    CompIdProblem,
    SendingTimeAccuracyProblem,
    InvalidMessageType,
    ParseError,
    InvalidBodyLength,
    InvalidChecksum,
}

impl fmt::Display for SessionLevelRejectErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Fix Error {:?}", self.kind)
    }
}

impl Error for SessionLevelRejectErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if self.source.is_some() {
            return self.source.as_deref();
        }
        return None
    }
}
