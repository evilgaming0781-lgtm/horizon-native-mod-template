//! Signal system for Horizon Mechanical Works.
//!
//! This module defines the core signal infrastructure for mechanical component interactions,
//! including signal identification, types, payloads, and transmission mechanisms.

use std::fmt;

/// A unique identifier for signals in the interaction system.
///
/// This type is used to distinguish between different signal types and ensure
/// type safety when working with signal operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalId(pub u32);

impl fmt::Display for SignalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signal{}", self.0)
    }
}

impl Default for SignalId {
    fn default() -> Self {
        SignalId(0)
    }
}

impl SignalId {
    /// Creates a new signal ID with the specified value.
    pub const fn new(id: u32) -> Self {
        SignalId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Represents different types of signals that can be transmitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignalType {
    /// A boolean signal (true/false)
    Boolean,
    
    /// An integer signal with a 32-bit value
    Integer,
    
    /// A floating-point signal with a 32-bit value
    Float,
    
    /// A string signal
    String,
    
    /// A custom payload signal
    Custom,
}

impl fmt::Display for SignalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalType::Boolean => write!(f, "Boolean"),
            SignalType::Integer => write!(f, "Integer"),
            SignalType::Float => write!(f, "Float"),
            SignalType::String => write!(f, "String"),
            SignalType::Custom => write!(f, "Custom"),
        }
    }
}

/// Represents the payload of a signal.
///
/// The payload contains the actual data being transmitted with the signal.
#[derive(Debug, Clone, PartialEq)]
pub enum SignalPayload {
    /// Boolean value
    Boolean(bool),
    
    /// Integer value (32-bit)
    Integer(i32),
    
    /// Floating-point value (32-bit)
    Float(f32),
    
    /// String value
    String(String),
    
    /// Custom binary payload
    Custom(Vec<u8>),
}

impl SignalPayload {
    /// Creates a new boolean signal payload.
    pub const fn boolean(value: bool) -> Self {
        SignalPayload::Boolean(value)
    }

    /// Creates a new integer signal payload.
    pub const fn integer(value: i32) -> Self {
        SignalPayload::Integer(value)
    }

    /// Creates a new float signal payload.
    pub const fn float(value: f32) -> Self {
        SignalPayload::Float(value)
    }

    /// Creates a new string signal payload.
    pub fn string(value: String) -> Self {
        SignalPayload::String(value)
    }

    /// Creates a new custom signal payload.
    pub fn custom(value: Vec<u8>) -> Self {
        SignalPayload::Custom(value)
    }

    /// Gets the type of this payload.
    pub const fn payload_type(&self) -> SignalType {
        match self {
            SignalPayload::Boolean(_) => SignalType::Boolean,
            SignalPayload::Integer(_) => SignalType::Integer,
            SignalPayload::Float(_) => SignalType::Float,
            SignalPayload::String(_) => SignalType::String,
            SignalPayload::Custom(_) => SignalType::Custom,
        }
    }

    /// Gets the boolean value if this is a boolean payload.
    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            SignalPayload::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Gets the integer value if this is an integer payload.
    pub const fn as_integer(&self) -> Option<i32> {
        match self {
            SignalPayload::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Gets the float value if this is a float payload.
    pub const fn as_float(&self) -> Option<f32> {
        match self {
            SignalPayload::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Gets the string value if this is a string payload.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SignalPayload::String(value) => Some(value),
            _ => None,
        }
    }

    /// Gets the custom payload if this is a custom payload.
    pub fn as_custom(&self) -> Option<&[u8]> {
        match self {
            SignalPayload::Custom(value) => Some(value),
            _ => None,
        }
    }
}

/// Represents a signal with its identifier, type, and payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    /// The unique identifier of this signal.
    pub id: SignalId,
    
    /// The type of this signal.
    pub signal_type: SignalType,
    
    /// The payload of this signal.
    pub payload: SignalPayload,
    
    /// The priority of this signal (higher values = higher priority).
    pub priority: u8,
}

impl Signal {
    /// Creates a new signal with the specified parameters.
    pub fn new(id: SignalId, signal_type: SignalType, payload: SignalPayload, priority: u8) -> Self {
        Signal {
            id,
            signal_type,
            payload,
            priority,
        }
    }

    /// Creates a boolean signal.
    pub fn boolean(id: SignalId, value: bool, priority: u8) -> Self {
        Signal::new(id, SignalType::Boolean, SignalPayload::boolean(value), priority)
    }

    /// Creates an integer signal.
    pub fn integer(id: SignalId, value: i32, priority: u8) -> Self {
        Signal::new(id, SignalType::Integer, SignalPayload::integer(value), priority)
    }

    /// Creates a float signal.
    pub fn float(id: SignalId, value: f32, priority: u8) -> Self {
        Signal::new(id, SignalType::Float, SignalPayload::float(value), priority)
    }

    /// Creates a string signal.
    pub fn string(id: SignalId, value: String, priority: u8) -> Self {
        Signal::new(id, SignalType::String, SignalPayload::string(value), priority)
    }

    /// Creates a custom signal.
    pub fn custom(id: SignalId, value: Vec<u8>, priority: u8) -> Self {
        Signal::new(id, SignalType::Custom, SignalPayload::custom(value), priority)
    }

    /// Gets the type of this signal's payload.
    pub const fn payload_type(&self) -> SignalType {
        self.signal_type
    }

    /// Checks if this signal matches a specific signal type.
    pub fn is_type(&self, signal_type: SignalType) -> bool {
        self.signal_type == signal_type
    }
}

/// Represents a signal filter for filtering signals by various criteria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalFilter {
    /// The signal ID to match (None means any ID).
    pub id: Option<SignalId>,
    
    /// The signal type to match (None means any type).
    pub signal_type: Option<SignalType>,
    
    /// The minimum priority threshold (None means no threshold).
    pub min_priority: Option<u8>,
}

impl SignalFilter {
    /// Creates a new filter with all criteria set to None.
    pub const fn new() -> Self {
        SignalFilter {
            id: None,
            signal_type: None,
            min_priority: None,
        }
    }

    /// Creates a filter that matches only the specified signal ID.
    pub const fn by_id(id: SignalId) -> Self {
        SignalFilter {
            id: Some(id),
            signal_type: None,
            min_priority: None,
        }
    }

    /// Creates a filter that matches only the specified signal type.
    pub const fn by_type(signal_type: SignalType) -> Self {
        SignalFilter {
            id: None,
            signal_type: Some(signal_type),
            min_priority: None,
        }
    }

    /// Creates a filter that matches signals with priority at least the specified value.
    pub const fn by_priority(min_priority: u8) -> Self {
        SignalFilter {
            id: None,
            signal_type: None,
            min_priority: Some(min_priority),
        }
    }

    /// Checks if a signal matches this filter.
    pub fn matches(&self, signal: &Signal) -> bool {
        // Check ID filter
        if let Some(filter_id) = self.id {
            if signal.id != filter_id {
                return false;
            }
        }
        
        // Check signal type filter
        if let Some(filter_type) = self.signal_type {
            if signal.signal_type != filter_type {
                return false;
            }
        }
        
        // Check priority filter
        if let Some(min_priority) = self.min_priority {
            if signal.priority < min_priority {
                return false;
            }
        }
        
        true
    }
}

/// Represents a signal error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalError {
    /// Attempted to create a signal with an invalid payload type.
    InvalidPayload,
    
    /// Attempted to use an unknown or unsupported signal ID.
    UnknownSignal,
    
    /// Attempted to send a signal with invalid priority.
    InvalidPriority,
    
    /// Attempted to filter signals with invalid criteria.
    InvalidFilter,
}

impl fmt::Display for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalError::InvalidPayload => write!(f, "Invalid signal payload"),
            SignalError::UnknownSignal => write!(f, "Unknown or unsupported signal"),
            SignalError::InvalidPriority => write!(f, "Invalid signal priority"),
            SignalError::InvalidFilter => write!(f, "Invalid signal filter criteria"),
        }
    }
}

impl std::error::Error for SignalError {}

/// Represents a signal transmission result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignalResult {
    /// Signal was transmitted successfully.
    Success,
    
    /// Signal transmission failed with an error.
    Error(SignalError),
}

impl fmt::Display for SignalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalResult::Success => write!(f, "Signal transmitted successfully"),
            SignalResult::Error(error) => write!(f, "Signal transmission failed: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_id_creation() {
        let signal_id = SignalId::new(42);
        assert_eq!(signal_id.get(), 42);
        assert_eq!(format!("{}", signal_id), "Signal42");
        
        let default_id = SignalId::default();
        assert_eq!(default_id.get(), 0);
    }

    #[test]
    fn test_signal_type_display() {
        assert_eq!(format!("{}", SignalType::Boolean), "Boolean");
        assert_eq!(format!("{}", SignalType::Integer), "Integer");
        assert_eq!(format!("{}", SignalType::Float), "Float");
        assert_eq!(format!("{}", SignalType::String), "String");
        assert_eq!(format!("{}", SignalType::Custom), "Custom");
    }

    #[test]
    fn test_signal_payload_creation() {
        let bool_payload = SignalPayload::boolean(true);
        assert_eq!(bool_payload.payload_type(), SignalType::Boolean);
        assert_eq!(bool_payload.as_boolean(), Some(true));
        
        let int_payload = SignalPayload::integer(42);
        assert_eq!(int_payload.payload_type(), SignalType::Integer);
        assert_eq!(int_payload.as_integer(), Some(42));
        
        let float_payload = SignalPayload::float(3.14);
        assert_eq!(float_payload.payload_type(), SignalType::Float);
        assert_eq!(float_payload.as_float(), Some(3.14));
        
        let string_payload = SignalPayload::string("test".to_string());
        assert_eq!(string_payload.payload_type(), SignalType::String);
        assert_eq!(string_payload.as_string(), Some("test"));
        
        let custom_payload = SignalPayload::custom(vec![1, 2, 3]);
        assert_eq!(custom_payload.payload_type(), SignalType::Custom);
        assert_eq!(custom_payload.as_custom(), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn test_signal_creation() {
        let signal = Signal::boolean(SignalId::new(1), true, 5);
        assert_eq!(signal.id.get(), 1);
        assert_eq!(signal.signal_type, SignalType::Boolean);
        assert_eq!(signal.priority, 5);
        
        // Test payload access
        assert_eq!(signal.payload.as_boolean(), Some(true));
    }

    #[test]
    fn test_signal_type_checking() {
        let signal = Signal::integer(SignalId::new(1), 42, 5);
        assert!(signal.is_type(SignalType::Integer));
        assert!(!signal.is_type(SignalType::Boolean));
    }

    #[test]
    fn test_signal_filter_creation() {
        let filter1 = SignalFilter::new();
        assert_eq!(filter1.id, None);
        assert_eq!(filter1.signal_type, None);
        assert_eq!(filter1.min_priority, None);
        
        let filter2 = SignalFilter::by_id(SignalId::new(10));
        assert_eq!(filter2.id, Some(SignalId::new(10)));
        assert_eq!(filter2.signal_type, None);
        assert_eq!(filter2.min_priority, None);
        
        let filter3 = SignalFilter::by_type(SignalType::Boolean);
        assert_eq!(filter3.id, None);
        assert_eq!(filter3.signal_type, Some(SignalType::Boolean));
        assert_eq!(filter3.min_priority, None);
        
        let filter4 = SignalFilter::by_priority(10);
        assert_eq!(filter4.id, None);
        assert_eq!(filter4.signal_type, None);
        assert_eq!(filter4.min_priority, Some(10));
    }

    #[test]
    fn test_signal_filter_matching() {
        let signal = Signal::boolean(SignalId::new(1), true, 5);
        
        // Test matching by ID
        let filter = SignalFilter::by_id(SignalId::new(1));
        assert!(filter.matches(&signal));
        
        let filter2 = SignalFilter::by_id(SignalId::new(2));
        assert!(!filter2.matches(&signal));
        
        // Test matching by type
        let filter3 = SignalFilter::by_type(SignalType::Boolean);
        assert!(filter3.matches(&signal));
        
        let filter4 = SignalFilter::by_type(SignalType::Integer);
        assert!(!filter4.matches(&signal));
        
        // Test matching by priority
        let filter5 = SignalFilter::by_priority(3);
        assert!(filter5.matches(&signal));
        
        let filter6 = SignalFilter::by_priority(10);
        assert!(!filter6.matches(&signal));
        
        // Test combined filters
        let filter7 = SignalFilter {
            id: Some(SignalId::new(1)),
            signal_type: Some(SignalType::Boolean),
            min_priority: Some(3),
        };
        assert!(filter7.matches(&signal));
        
        let filter8 = SignalFilter {
            id: Some(SignalId::new(1)),
            signal_type: Some(SignalType::Integer), // Wrong type
            min_priority: Some(3),
        };
        assert!(!filter8.matches(&signal));
    }

    #[test]
    fn test_signal_error_display() {
        let error = SignalError::InvalidPayload;
        assert_eq!(format!("{}", error), "Invalid signal payload");
        
        let error2 = SignalError::UnknownSignal;
        assert_eq!(format!("{}", error2), "Unknown or unsupported signal");
        
        let error3 = SignalError::InvalidPriority;
        assert_eq!(format!("{}", error3), "Invalid signal priority");
        
        let error4 = SignalError::InvalidFilter;
        assert_eq!(format!("{}", error4), "Invalid signal filter criteria");
    }

    #[test]
    fn test_signal_result_display() {
        let result = SignalResult::Success;
        assert_eq!(format!("{}", result), "Signal transmitted successfully");
        
        let result2 = SignalResult::Error(SignalError::UnknownSignal);
        assert_eq!(format!("{}", result2), "Signal transmission failed: Unknown or unsupported signal");
    }
}