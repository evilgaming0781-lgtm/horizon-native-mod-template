//! Shared state foundations for mechanical components.
//!
//! This module provides reusable state-related definitions that can be used 
//! across different subsystems while avoiding duplication of machine-specific states.

use std::fmt;

/// The operational state of a mechanical component or system.
///
/// This enum represents the various states a mechanical component can be in,
/// providing a standardized way to track and manage component behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationalState {
    /// The component is disabled and not operational.
    Disabled,
    
    /// The component is enabled but idle (not actively performing work).
    Idle,
    
    /// The component is enabled and actively operating.
    Active,
    
    /// The component is paused (temporarily stopped but ready to resume).
    Paused,
    
    /// The component has encountered an error or fault condition.
    Faulted,
}

impl fmt::Display for OperationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationalState::Disabled => write!(f, "Disabled"),
            OperationalState::Idle => write!(f, "Idle"),
            OperationalState::Active => write!(f, "Active"),
            OperationalState::Paused => write!(f, "Paused"),
            OperationalState::Faulted => write!(f, "Faulted"),
        }
    }
}

/// A state transition error.
///
/// This error represents situations where an invalid state transition was attempted.
#[derive(Debug, Clone, PartialEq)]
pub enum StateError {
    /// Indicates that a transition to the requested state is not allowed from the current state.
    InvalidTransition,
    
    /// Indicates that the component is in a faulted state and cannot be transitioned normally.
    ComponentFaulted,
    
    /// Indicates that the component is disabled and cannot be activated.
    ComponentDisabled,
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::InvalidTransition => write!(f, "Invalid state transition"),
            StateError::ComponentFaulted => write!(f, "Component is in faulted state"),
            StateError::ComponentDisabled => write!(f, "Component is disabled"),
        }
    }
}

impl std::error::Error for StateError {}

/// A trait for components that have operational states.
///
/// This trait defines the interface for components that can be in various operational states.
pub trait Stateful {
    /// Gets the current operational state of the component.
    fn state(&self) -> OperationalState;
    
    /// Checks if the component is currently active.
    fn is_active(&self) -> bool {
        matches!(self.state(), OperationalState::Active)
    }
    
    /// Checks if the component is currently idle.
    fn is_idle(&self) -> bool {
        matches!(self.state(), OperationalState::Idle)
    }
    
    /// Checks if the component is disabled.
    fn is_disabled(&self) -> bool {
        matches!(self.state(), OperationalState::Disabled)
    }
    
    /// Checks if the component is in a faulted state.
    fn is_faulted(&self) -> bool {
        matches!(self.state(), OperationalState::Faulted)
    }
    
    /// Attempts to transition the component to the requested state.
    ///
    /// # Arguments
    /// * `new_state` - The desired new state for the component
    ///
    /// # Returns
    /// Ok(()) if the transition was successful, or an error if it was not allowed
    fn transition_to(&mut self, new_state: OperationalState) -> Result<(), StateError>;
}

/// A simple implementation of a stateful component.
///
/// This struct provides a basic implementation that can be used as a foundation 
/// for more complex state management in specific subsystems.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleState {
    /// The current operational state.
    pub current_state: OperationalState,
}

impl SimpleState {
    /// Creates a new simple state with the specified initial state.
    pub fn new(initial_state: OperationalState) -> Self {
        SimpleState {
            current_state: initial_state,
        }
    }
    
    /// Gets the current operational state.
    pub fn state(&self) -> OperationalState {
        self.current_state
    }
    
    /// Sets the operational state directly (use with caution).
    ///
    /// # Arguments
    /// * `state` - The new state to set
    pub fn set_state(&mut self, state: OperationalState) {
        self.current_state = state;
    }
}

impl Stateful for SimpleState {
    fn state(&self) -> OperationalState {
        self.current_state
    }
    
    fn transition_to(&mut self, new_state: OperationalState) -> Result<(), StateError> {
        // Basic validation of state transitions
        match (self.current_state, new_state) {
            // Can always go to disabled from any state
            (_, OperationalState::Disabled) => {
                self.current_state = new_state;
                Ok(())
            }
            
            // Cannot transition from faulted state
            (OperationalState::Faulted, _) => Err(StateError::ComponentFaulted),
            
            // Cannot activate a disabled component
            (OperationalState::Disabled, OperationalState::Active) => {
                Err(StateError::ComponentDisabled)
            }
            
            // Valid transitions
            (OperationalState::Idle, OperationalState::Active) |
            (OperationalState::Idle, OperationalState::Paused) |
            (OperationalState::Active, OperationalState::Paused) |
            (OperationalState::Active, OperationalState::Idle) |
            (OperationalState::Paused, OperationalState::Active) |
            (OperationalState::Paused, OperationalState::Idle) => {
                self.current_state = new_state;
                Ok(())
            }
            
            // No-op transitions
            (current, target) if current == target => Ok(()),
            
            // All other transitions are invalid
            _ => Err(StateError::InvalidTransition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_state_display() {
        assert_eq!(format!("{}", OperationalState::Disabled), "Disabled");
        assert_eq!(format!("{}", OperationalState::Idle), "Idle");
        assert_eq!(format!("{}", OperationalState::Active), "Active");
        assert_eq!(format!("{}", OperationalState::Paused), "Paused");
        assert_eq!(format!("{}", OperationalState::Faulted), "Faulted");
    }

    #[test]
    fn test_simple_state_initialization() {
        let state = SimpleState::new(OperationalState::Idle);
        assert_eq!(state.state(), OperationalState::Idle);
    }

    #[test]
    fn test_simple_state_transitions() {
        let mut state = SimpleState::new(OperationalState::Idle);
        
        // Valid transitions
        assert!(state.transition_to(OperationalState::Active).is_ok());
        assert_eq!(state.state(), OperationalState::Active);
        
        assert!(state.transition_to(OperationalState::Paused).is_ok());
        assert_eq!(state.state(), OperationalState::Paused);
        
        assert!(state.transition_to(OperationalState::Idle).is_ok());
        assert_eq!(state.state(), OperationalState::Idle);
        
        // Invalid transitions
        assert!(state.transition_to(OperationalState::Active).is_ok());
        assert!(state.transition_to(OperationalState::Faulted).is_ok()); // This should work, but then faulted state prevents further transitions
        
        // After entering faulted state, other transitions should fail
        assert!(state.transition_to(OperationalState::Active).is_err());
    }

    #[test]
    fn test_simple_state_disabled_transitions() {
        let mut state = SimpleState::new(OperationalState::Disabled);
        
        // Can always go to disabled from any state
        assert!(state.transition_to(OperationalState::Idle).is_ok());
        assert_eq!(state.state(), OperationalState::Idle);
        
        // Cannot activate a disabled component
        assert!(state.transition_to(OperationalState::Active).is_err());
    }

    #[test]
    fn test_simple_state_faulted_transitions() {
        let mut state = SimpleState::new(OperationalState::Active);
        assert!(state.transition_to(OperationalState::Faulted).is_ok());
        
        // After faulted, no normal transitions should work
        assert!(state.transition_to(OperationalState::Idle).is_err());
        assert!(state.transition_to(OperationalState::Active).is_err());
    }

    #[test]
    fn test_stateful_trait_methods() {
        let mut state = SimpleState::new(OperationalState::Active);
        
        assert!(state.is_active());
        assert!(!state.is_idle());
        assert!(!state.is_disabled());
        assert!(!state.is_faulted());
        
        state.set_state(OperationalState::Idle);
        assert!(!state.is_active());
        assert!(state.is_idle());
    }

    #[test]
    fn test_state_error_display() {
        let error = StateError::InvalidTransition;
        assert_eq!(format!("{}", error), "Invalid state transition");
        
        let error = StateError::ComponentFaulted;
        assert_eq!(format!("{}", error), "Component is in faulted state");
        
        let error = StateError::ComponentDisabled;
        assert_eq!(format!("{}", error), "Component is disabled");
    }
}