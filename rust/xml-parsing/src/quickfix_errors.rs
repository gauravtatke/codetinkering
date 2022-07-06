use std::error::Error;
use std::fmt::{self, Formatter};
use std::num::ParseIntError;
use thiserror;

#[derive(Debug, thiserror::Error)]
#[error("Session Level Reject Reason - {:?}", .kind)]
pub struct SessionRejectError {
    kind: SessionRejectReason,
    // tag: Option<String>,
    // value: Option<String>,
    // pub source: Option<Box<dyn Error>>,
}

impl SessionRejectError {
    // pub fn invalid_value_for_tag_err(value: &str) -> Self {
    //     SessionRejectError {
    //         kind: SessionRejectReason::InvalidValueForTag(value),
    //     }
    // }

    pub fn invalid_tag_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidTag,
        }
    }

    pub fn required_tag_missing_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::RequiredTagMissing,
        }
    }

    pub fn undefined_tag_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::UndefinedTag,
        }
    }

    pub fn tag_without_value_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::TagSpecifiedWithoutValue,
        }
    }

    pub fn value_out_of_range_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::ValueOutOfRange,
        }
    }

    pub fn incorrect_data_format_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::IncorrectDataFormat,
        }
    }

    pub fn decryption_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::DecryptionProblem,
        }
    }

    pub fn signature_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::SignatureProblem,
        }
    }

    pub fn comp_id_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::CompIdProblem,
        }
    }

    pub fn sending_time_accuracy_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::SendingTimeAccuracyProblem,
        }
    }

    pub fn invalid_msg_type_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidMessageType,
        }
    }

    pub fn invalid_body_len_err() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidBodyLength,
        }
    }

    pub fn invalid_checksum() -> Self {
        SessionRejectError {
            kind: SessionRejectReason::InvalidChecksum,
        }
    }

    // pub fn parse_err(err: Box<dyn Error>) -> Self {
    //     SessionRejectError {
    //         kind: SessionRejectReason::ParseError(err),
    //     }
    // }
}

#[derive(Debug)]
enum SessionRejectReason {
    // #[error("Invalid tag")]
    InvalidTag,
    // #[error("Required tag missing")]
    RequiredTagMissing,
    // #[error("Undefined tag")]
    UndefinedTag,
    // #[error("Tag not defined for message, tag")]
    TagNotDefinedForMsgType,
    // #[error("No value for tag")]
    TagSpecifiedWithoutValue,
    // #[error("Value out of range for tag")]
    ValueOutOfRange,
    // #[error("Incorrect data format")]
    IncorrectDataFormat,
    // #[error("Decryption problem")]
    DecryptionProblem,
    // #[error("Signature problem")]
    SignatureProblem,
    // #[error("Compid problem")]
    CompIdProblem,
    // #[error("Sending time accuracy problem")]
    SendingTimeAccuracyProblem,
    // #[error("Invalid message type")]
    InvalidMessageType,
    // #[error("Invalid body length")]
    InvalidBodyLength,
    // #[error("Invalid checksum")]
    InvalidChecksum,
}

#[derive(Debug, thiserror::Error)]
pub enum XmlError {
    #[error("Could not parse the document")]
    DocumentNotParsed(#[from] roxmltree::Error),
    #[error("Node {} not found", .0)]
    XmlNodeNotFound(String),
    #[error("Could not parse field {field} into u32: {:?}", .source)]
    FieldNotParsed {
        source: ParseIntError,
        field: String,
    },
    #[error("Duplicate field {}", .0)]
    DuplicateField(String),
    #[error("Duplicate message {}", .0)]
    DuplicateMessage(String),
    #[error("Attribute {} not found", .0)]
    AttributeNotFound(String),
    #[error("Unknown xml tag {}", .0)]
    UnknownXmlTag(String),
}
