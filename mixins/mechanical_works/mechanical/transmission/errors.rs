//! Transmission-specific error handling.
//!
//! This module defines all error variants that can occur during transmission operations,
//! providing meaningful feedback for failed transmission management tasks.

use std::fmt;

/// A result type alias for transmission operations.
pub type TransmissionResult<T> = Result<T, TransmissionError>;

/// Errors that can occur when managing transmission components.
#[derive(Debug, Clone, PartialEq)]
pub enum TransmissionError {
    /// Indicates that a transmission configuration is invalid.
    InvalidConfiguration,
    
    /// Indicates that a shaft parameter exceeds its maximum limits.
    ExceededTorque,
    
    /// Indicates that a shaft parameter exceeds its maximum limits.
    ExceededRPM,
    
    /// Indicates that a shaft parameter exceeds its maximum limits.
    ExceededPower,
    
    /// Indicates that an attachment to a coupling is invalid.
    InvalidAttachment,
    
    /// Indicates that a coupling has too many attachments.
    TooManyAttachments,
    
    /// Indicates that a coupling is attempting to attach to itself.
    SelfConnection,
    
    /// Indicates that a coupling already has this attachment.
    DuplicateAttachment,
    
    /// Indicates that an attachment is missing from a coupling.
    MissingAttachment,
    
    /// Indicates that a coupling is invalid (missing components).
    InvalidCoupling,
    
    /// Indicates that a transmission component is disabled.
    DisabledComponent,
    
    /// Indicates that a shaft has not been properly initialized.
    UninitializedShaft,
}

impl fmt::Display for TransmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransmissionError::InvalidConfiguration => write!(f, "Transmission configuration is invalid"),
            TransmissionError::ExceededTorque => write!(f, "Torque exceeds maximum limit"),
            TransmissionError::ExceededRPM => write!(f, "Rotational speed exceeds maximum limit"),
            TransmissionError::ExceededPower => write!(f, "Power exceeds maximum limit"),
            TransmissionError::InvalidAttachment => write!(f, "Invalid attachment to coupling"),
            TransmissionError::TooManyAttachments => write!(f, "Coupling has too many attachments"),
            TransmissionError::SelfConnection => write!(f, "Cannot connect a component to itself"),
            TransmissionError::DuplicateAttachment => write!(f, "Component already attached to coupling"),
            TransmissionError::MissingAttachment => write!(f, "Component is not attached to coupling"),
            TransmissionError::InvalidCoupling => write!(f, "Coupling has invalid configuration"),
            TransmissionError::DisabledComponent => write!(f, "Transmission component is disabled"),
            TransmissionError::UninitializedShaft => write!(f, "Shaft has not been properly initialized"),
        }
    }
}

impl std::error::Error for TransmissionError {}