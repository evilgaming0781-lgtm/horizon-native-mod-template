//! Communication protocols for Horizon Mechanical Works.
//!
//! This module defines structured communication protocols for mechanical component interactions,
//! including request/response messaging, message correlation, and protocol versioning.

use std::fmt;

/// A unique identifier for communication protocols.
///
/// This type is used to distinguish between different protocol types and ensure
/// proper handling of messages according to their intended protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolId(pub u32);

impl fmt::Display for ProtocolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Protocol{}", self.0)
    }
}

impl Default for ProtocolId {
    fn default() -> Self {
        ProtocolId(0)
    }
}

impl ProtocolId {
    /// Creates a new protocol ID with the specified value.
    pub const fn new(id: u32) -> Self {
        ProtocolId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Represents a message correlation identifier.
///
/// Used to match requests with their corresponding responses in communication protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(pub u64);

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Msg{}", self.0)
    }
}

impl Default for MessageId {
    fn default() -> Self {
        MessageId(0)
    }
}

impl MessageId {
    /// Creates a new message ID with the specified value.
    pub const fn new(id: u64) -> Self {
        MessageId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// Represents the version of a communication protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolVersion {
    /// Major version number.
    pub major: u16,
    
    /// Minor version number.
    pub minor: u16,
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion { major: 1, minor: 0 }
    }
}

impl ProtocolVersion {
    /// Creates a new protocol version with specified major and minor numbers.
    pub const fn new(major: u16, minor: u16) -> Self {
        ProtocolVersion { major, minor }
    }

    /// Checks if this version is compatible with another version.
    ///
    /// Two versions are compatible if they have the same major version.
    pub const fn is_compatible_with(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major
    }
}

/// Represents a structured message in a communication protocol.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolMessage {
    /// The protocol identifier this message belongs to.
    pub protocol_id: ProtocolId,
    
    /// The version of the protocol this message uses.
    pub version: ProtocolVersion,
    
    /// The correlation identifier for this message (for requests/responses).
    pub message_id: MessageId,
    
    /// The type of message (request, response, etc.).
    pub message_type: MessageType,
    
    /// The payload content of the message.
    pub payload: SignalPayload,
}

impl ProtocolMessage {
    /// Creates a new protocol message with specified parameters.
    pub fn new(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        message_type: MessageType,
        payload: SignalPayload,
    ) -> Self {
        ProtocolMessage {
            protocol_id,
            version,
            message_id,
            message_type,
            payload,
        }
    }

    /// Creates a request message.
    pub fn request(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        payload: SignalPayload,
    ) -> Self {
        ProtocolMessage::new(protocol_id, version, message_id, MessageType::Request, payload)
    }

    /// Creates a response message.
    pub fn response(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        payload: SignalPayload,
    ) -> Self {
        ProtocolMessage::new(protocol_id, version, message_id, MessageType::Response, payload)
    }

    /// Creates an error response message.
    pub fn error(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        error_code: u16,
        error_message: String,
    ) -> Self {
        let payload = SignalPayload::string(format!("{}: {}", error_code, error_message));
        ProtocolMessage::new(protocol_id, version, message_id, MessageType::ErrorResponse, payload)
    }

    /// Creates a timeout response message.
    pub fn timeout(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
    ) -> Self {
        let payload = SignalPayload::string("Request timed out".to_string());
        ProtocolMessage::new(protocol_id, version, message_id, MessageType::TimeoutResponse, payload)
    }

    /// Checks if this is a request message.
    pub fn is_request(&self) -> bool {
        self.message_type == MessageType::Request
    }

    /// Checks if this is a response message.
    pub fn is_response(&self) -> bool {
        self.message_type == MessageType::Response
    }

    /// Checks if this is an error response message.
    pub fn is_error_response(&self) -> bool {
        self.message_type == MessageType::ErrorResponse
    }

    /// Checks if this is a timeout response message.
    pub fn is_timeout_response(&self) -> bool {
        self.message_type == MessageType::TimeoutResponse
    }
}

/// Represents the type of a protocol message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageType {
    /// A request message.
    Request,
    
    /// A successful response message.
    Response,
    
    /// An error response message.
    ErrorResponse,
    
    /// A timeout response message.
    TimeoutResponse,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Request => write!(f, "Request"),
            MessageType::Response => write!(f, "Response"),
            MessageType::ErrorResponse => write!(f, "Error"),
            MessageType::TimeoutResponse => write!(f, "Timeout"),
        }
    }
}

/// Represents a structured request message.
#[derive(Debug, Clone, PartialEq)]
pub struct RequestMessage {
    /// The protocol identifier this request belongs to.
    pub protocol_id: ProtocolId,
    
    /// The version of the protocol this request uses.
    pub version: ProtocolVersion,
    
    /// The correlation identifier for this request.
    pub message_id: MessageId,
    
    /// The payload content of the request.
    pub payload: SignalPayload,
}

impl RequestMessage {
    /// Creates a new request message.
    pub fn new(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        payload: SignalPayload,
    ) -> Self {
        RequestMessage {
            protocol_id,
            version,
            message_id,
            payload,
        }
    }

    /// Creates a request from a protocol message.
    pub fn from_message(message: &ProtocolMessage) -> Option<Self> {
        if message.is_request() {
            Some(RequestMessage {
                protocol_id: message.protocol_id,
                version: message.version,
                message_id: message.message_id,
                payload: message.payload.clone(),
            })
        } else {
            None
        }
    }

    /// Converts this request to a protocol message.
    pub fn to_message(&self) -> ProtocolMessage {
        ProtocolMessage::request(
            self.protocol_id,
            self.version,
            self.message_id,
            self.payload.clone(),
        )
    }
}

/// Represents a structured response message.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseMessage {
    /// The protocol identifier this response belongs to.
    pub protocol_id: ProtocolId,
    
    /// The version of the protocol this response uses.
    pub version: ProtocolVersion,
    
    /// The correlation identifier for this response.
    pub message_id: MessageId,
    
    /// The payload content of the response.
    pub payload: SignalPayload,
}

impl ResponseMessage {
    /// Creates a new response message.
    pub fn new(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        payload: SignalPayload,
    ) -> Self {
        ResponseMessage {
            protocol_id,
            version,
            message_id,
            payload,
        }
    }

    /// Creates a response from a protocol message.
    pub fn from_message(message: &ProtocolMessage) -> Option<Self> {
        if message.is_response() {
            Some(ResponseMessage {
                protocol_id: message.protocol_id,
                version: message.version,
                message_id: message.message_id,
                payload: message.payload.clone(),
            })
        } else {
            None
        }
    }

    /// Converts this response to a protocol message.
    pub fn to_message(&self) -> ProtocolMessage {
        ProtocolMessage::response(
            self.protocol_id,
            self.version,
            self.message_id,
            self.payload.clone(),
        )
    }
}

/// Represents an error response message.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMessage {
    /// The protocol identifier this error belongs to.
    pub protocol_id: ProtocolId,
    
    /// The version of the protocol this error uses.
    pub version: ProtocolVersion,
    
    /// The correlation identifier for this error.
    pub message_id: MessageId,
    
    /// The error code.
    pub error_code: u16,
    
    /// The error message text.
    pub error_message: String,
}

impl ErrorMessage {
    /// Creates a new error message.
    pub fn new(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
        error_code: u16,
        error_message: String,
    ) -> Self {
        ErrorMessage {
            protocol_id,
            version,
            message_id,
            error_code,
            error_message,
        }
    }

    /// Creates an error from a protocol message.
    pub fn from_message(message: &ProtocolMessage) -> Option<Self> {
        if message.is_error_response() {
            // Extract error code and message from payload
            if let Some(payload_str) = message.payload.as_string() {
                // Parse the format "code: message"
                if let Some(colon_pos) = payload_str.find(':') {
                    if let Ok(code) = payload_str[..colon_pos].parse::<u16>() {
                        let message_text = payload_str[colon_pos + 2..].to_string();
                        return Some(ErrorMessage {
                            protocol_id: message.protocol_id,
                            version: message.version,
                            message_id: message.message_id,
                            error_code: code,
                            error_message: message_text,
                        });
                    }
                }
            }
            // If parsing fails, create a generic error
            Some(ErrorMessage {
                protocol_id: message.protocol_id,
                version: message.version,
                message_id: message.message_id,
                error_code: 500,
                error_message: "Unknown error".to_string(),
            })
        } else {
            None
        }
    }

    /// Converts this error to a protocol message.
    pub fn to_message(&self) -> ProtocolMessage {
        ProtocolMessage::error(
            self.protocol_id,
            self.version,
            self.message_id,
            self.error_code,
            self.error_message.clone(),
        )
    }
}

/// Represents a timeout response message.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeoutMessage {
    /// The protocol identifier this timeout belongs to.
    pub protocol_id: ProtocolId,
    
    /// The version of the protocol this timeout uses.
    pub version: ProtocolVersion,
    
    /// The correlation identifier for this timeout.
    pub message_id: MessageId,
}

impl TimeoutMessage {
    /// Creates a new timeout message.
    pub fn new(
        protocol_id: ProtocolId,
        version: ProtocolVersion,
        message_id: MessageId,
    ) -> Self {
        TimeoutMessage {
            protocol_id,
            version,
            message_id,
        }
    }

    /// Creates a timeout from a protocol message.
    pub fn from_message(message: &ProtocolMessage) -> Option<Self> {
        if message.is_timeout_response() {
            Some(TimeoutMessage {
                protocol_id: message.protocol_id,
                version: message.version,
                message_id: message.message_id,
            })
        } else {
            None
        }
    }

    /// Converts this timeout to a protocol message.
    pub fn to_message(&self) -> ProtocolMessage {
        ProtocolMessage::timeout(
            self.protocol_id,
            self.version,
            self.message_id,
        )
    }
}

/// Represents a communication protocol error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    /// Attempted to use an unsupported protocol version.
    UnsupportedVersion,
    
    /// Attempted to create or process an invalid message.
    InvalidMessage,
    
    /// Attempted to match request and response messages with mismatched IDs.
    MismatchedCorrelation,
    
    /// Attempted to process a message with an unknown protocol ID.
    UnknownProtocol,
    
    /// Protocol validation failed.
    ValidationFailed,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::UnsupportedVersion => write!(f, "Unsupported protocol version"),
            ProtocolError::InvalidMessage => write!(f, "Invalid message format"),
            ProtocolError::MismatchedCorrelation => write!(f, "Mismatched message correlation"),
            ProtocolError::UnknownProtocol => write!(f, "Unknown or unsupported protocol"),
            ProtocolError::ValidationFailed => write!(f, "Protocol validation failed"),
        }
    }
}

impl std::error::Error for ProtocolError {}

/// Represents a protocol result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProtocolResult {
    /// Operation completed successfully.
    Success,
    
    /// Operation failed with an error.
    Error(ProtocolError),
}

impl fmt::Display for ProtocolResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolResult::Success => write!(f, "Operation completed successfully"),
            ProtocolResult::Error(error) => write!(f, "Operation failed: {}", error),
        }
    }
}

/// Defines the interface for protocol message handling.
pub trait ProtocolHandler {
    /// Handles a request message and returns an appropriate response.
    fn handle_request(&mut self, request: &RequestMessage) -> ResponseMessage;
    
    /// Handles a response message.
    fn handle_response(&mut self, response: &ResponseMessage);
    
    /// Handles an error response message.
    fn handle_error(&mut self, error: &ErrorMessage);
    
    /// Handles a timeout response message.
    fn handle_timeout(&mut self, timeout: &TimeoutMessage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_id_creation() {
        let protocol_id = ProtocolId::new(42);
        assert_eq!(protocol_id.get(), 42);
        assert_eq!(format!("{}", protocol_id), "Protocol42");
        
        let default_id = ProtocolId::default();
        assert_eq!(default_id.get(), 0);
    }

    #[test]
    fn test_message_id_creation() {
        let message_id = MessageId::new(123456789);
        assert_eq!(message_id.get(), 123456789);
        assert_eq!(format!("{}", message_id), "Msg123456789");
        
        let default_id = MessageId::default();
        assert_eq!(default_id.get(), 0);
    }

    #[test]
    fn test_protocol_version_creation() {
        let version = ProtocolVersion::new(2, 1);
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 1);
        assert_eq!(format!("{}", version), "2.1");
        
        let default_version = ProtocolVersion::default();
        assert_eq!(default_version.major, 1);
        assert_eq!(default_version.minor, 0);
    }

    #[test]
    fn test_protocol_version_compatibility() {
        let v1 = ProtocolVersion::new(1, 0);
        let v2 = ProtocolVersion::new(1, 5);
        let v3 = ProtocolVersion::new(2, 0);
        
        assert!(v1.is_compatible_with(&v2));
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_message_type_display() {
        assert_eq!(format!("{}", MessageType::Request), "Request");
        assert_eq!(format!("{}", MessageType::Response), "Response");
        assert_eq!(format!("{}", MessageType::ErrorResponse), "Error");
        assert_eq!(format!("{}", MessageType::TimeoutResponse), "Timeout");
    }

    #[test]
    fn test_protocol_message_creation() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let request = ProtocolMessage::request(protocol_id, version, message_id, SignalPayload::boolean(true));
        assert_eq!(request.protocol_id, protocol_id);
        assert_eq!(request.version, version);
        assert_eq!(request.message_id, message_id);
        assert_eq!(request.message_type, MessageType::Request);
        assert_eq!(request.payload.as_boolean(), Some(true));
        
        let response = ProtocolMessage::response(protocol_id, version, message_id, SignalPayload::integer(42));
        assert_eq!(response.message_type, MessageType::Response);
        assert_eq!(response.payload.as_integer(), Some(42));
    }

    #[test]
    fn test_protocol_message_error() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let error = ProtocolMessage::error(protocol_id, version, message_id, 404, "Not found".to_string());
        assert_eq!(error.message_type, MessageType::ErrorResponse);
        assert_eq!(error.payload.as_string(), Some("404: Not found"));
    }

    #[test]
    fn test_protocol_message_timeout() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let timeout = ProtocolMessage::timeout(protocol_id, version, message_id);
        assert_eq!(timeout.message_type, MessageType::TimeoutResponse);
        assert_eq!(timeout.payload.as_string(), Some("Request timed out"));
    }

    #[test]
    fn test_protocol_message_type_checks() {
        let request = ProtocolMessage::request(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::boolean(true)
        );
        
        assert!(request.is_request());
        assert!(!request.is_response());
        assert!(!request.is_error_response());
        assert!(!request.is_timeout_response());
    }

    #[test]
    fn test_request_message_creation() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let request = RequestMessage::new(protocol_id, version, message_id, SignalPayload::integer(42));
        assert_eq!(request.protocol_id, protocol_id);
        assert_eq!(request.version, version);
        assert_eq!(request.message_id, message_id);
        assert_eq!(request.payload.as_integer(), Some(42));
    }

    #[test]
    fn test_request_message_from_protocol_message() {
        let protocol_msg = ProtocolMessage::request(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        let request = RequestMessage::from_message(&protocol_msg).unwrap();
        assert_eq!(request.protocol_id, protocol_msg.protocol_id);
        assert_eq!(request.version, protocol_msg.version);
        assert_eq!(request.message_id, protocol_msg.message_id);
        assert_eq!(request.payload.as_integer(), Some(42));
        
        // Test with non-request message
        let response = ProtocolMessage::response(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        assert!(RequestMessage::from_message(&response).is_none());
    }

    #[test]
    fn test_request_message_to_protocol_message() {
        let request = RequestMessage::new(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        let protocol_msg = request.to_message();
        assert_eq!(protocol_msg.message_type, MessageType::Request);
        assert_eq!(protocol_msg.payload.as_integer(), Some(42));
    }

    #[test]
    fn test_response_message_creation() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let response = ResponseMessage::new(protocol_id, version, message_id, SignalPayload::string("success".to_string()));
        assert_eq!(response.protocol_id, protocol_id);
        assert_eq!(response.version, version);
        assert_eq!(response.message_id, message_id);
        assert_eq!(response.payload.as_string(), Some("success"));
    }

    #[test]
    fn test_response_message_from_protocol_message() {
        let protocol_msg = ProtocolMessage::response(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::string("success".to_string())
        );
        
        let response = ResponseMessage::from_message(&protocol_msg).unwrap();
        assert_eq!(response.protocol_id, protocol_msg.protocol_id);
        assert_eq!(response.version, protocol_msg.version);
        assert_eq!(response.message_id, protocol_msg.message_id);
        assert_eq!(response.payload.as_string(), Some("success"));
        
        // Test with non-response message
        let request = ProtocolMessage::request(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        assert!(ResponseMessage::from_message(&request).is_none());
    }

    #[test]
    fn test_response_message_to_protocol_message() {
        let response = ResponseMessage::new(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::string("success".to_string())
        );
        
        let protocol_msg = response.to_message();
        assert_eq!(protocol_msg.message_type, MessageType::Response);
        assert_eq!(protocol_msg.payload.as_string(), Some("success"));
    }

    #[test]
    fn test_error_message_creation() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let error = ErrorMessage::new(protocol_id, version, message_id, 500, "Internal error".to_string());
        assert_eq!(error.protocol_id, protocol_id);
        assert_eq!(error.version, version);
        assert_eq!(error.message_id, message_id);
        assert_eq!(error.error_code, 500);
        assert_eq!(error.error_message, "Internal error");
    }

    #[test]
    fn test_error_message_from_protocol_message() {
        let protocol_msg = ProtocolMessage::error(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            404,
            "Not found".to_string()
        );
        
        let error = ErrorMessage::from_message(&protocol_msg).unwrap();
        assert_eq!(error.protocol_id, protocol_msg.protocol_id);
        assert_eq!(error.version, protocol_msg.version);
        assert_eq!(error.message_id, protocol_msg.message_id);
        assert_eq!(error.error_code, 404);
        assert_eq!(error.error_message, "Not found");
        
        // Test with non-error message
        let response = ProtocolMessage::response(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        assert!(ErrorMessage::from_message(&response).is_none());
    }

    #[test]
    fn test_error_message_to_protocol_message() {
        let error = ErrorMessage::new(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            500,
            "Internal error".to_string()
        );
        
        let protocol_msg = error.to_message();
        assert_eq!(protocol_msg.message_type, MessageType::ErrorResponse);
        assert_eq!(protocol_msg.payload.as_string(), Some("500: Internal error"));
    }

    #[test]
    fn test_timeout_message_creation() {
        let protocol_id = ProtocolId::new(1);
        let version = ProtocolVersion::new(1, 0);
        let message_id = MessageId::new(100);
        
        let timeout = TimeoutMessage::new(protocol_id, version, message_id);
        assert_eq!(timeout.protocol_id, protocol_id);
        assert_eq!(timeout.version, version);
        assert_eq!(timeout.message_id, message_id);
    }

    #[test]
    fn test_timeout_message_from_protocol_message() {
        let protocol_msg = ProtocolMessage::timeout(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100)
        );
        
        let timeout = TimeoutMessage::from_message(&protocol_msg).unwrap();
        assert_eq!(timeout.protocol_id, protocol_msg.protocol_id);
        assert_eq!(timeout.version, protocol_msg.version);
        assert_eq!(timeout.message_id, protocol_msg.message_id);
        
        // Test with non-timeout message
        let request = ProtocolMessage::request(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100),
            SignalPayload::integer(42)
        );
        
        assert!(TimeoutMessage::from_message(&request).is_none());
    }

    #[test]
    fn test_timeout_message_to_protocol_message() {
        let timeout = TimeoutMessage::new(
            ProtocolId::new(1),
            ProtocolVersion::new(1, 0),
            MessageId::new(100)
        );
        
        let protocol_msg = timeout.to_message();
        assert_eq!(protocol_msg.message_type, MessageType::TimeoutResponse);
        assert_eq!(protocol_msg.payload.as_string(), Some("Request timed out"));
    }

    #[test]
    fn test_protocol_error_display() {
        let error = ProtocolError::UnsupportedVersion;
        assert_eq!(format!("{}", error), "Unsupported protocol version");
        
        let error2 = ProtocolError::InvalidMessage;
        assert_eq!(format!("{}", error2), "Invalid message format");
        
        let error3 = ProtocolError::MismatchedCorrelation;
        assert_eq!(format!("{}", error3), "Mismatched message correlation");
        
        let error4 = ProtocolError::UnknownProtocol;
        assert_eq!(format!("{}", error4), "Unknown or unsupported protocol");
        
        let error5 = ProtocolError::ValidationFailed;
        assert_eq!(format!("{}", error5), "Protocol validation failed");
    }

    #[test]
    fn test_protocol_result_display() {
        let result = ProtocolResult::Success;
        assert_eq!(format!("{}", result), "Operation completed successfully");
        
        let result2 = ProtocolResult::Error(ProtocolError::UnsupportedVersion);
        assert_eq!(format!("{}", result2), "Operation failed: Unsupported protocol version");
    }
}