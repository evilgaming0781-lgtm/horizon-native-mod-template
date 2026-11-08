//! Machine state management system for Horizon Mechanical Works.
//!
//! This module provides the foundational state management functionality for mechanical machines,
//! including valid operating states and controlled state transitions.

use crate::mechanical::machine::types::*;
use std::collections::VecDeque;

/// Represents the operational state of a machine.
///
/// Machines can exist in one of several defined states that determine their behavior
/// and capabilities within the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineState {
    /// Machine is idle and waiting for activation.
    Idle,
    
    /// Machine is actively processing or operating.
    Running,
    
    /// Machine is paused but can be resumed.
    Paused,
    
    /// Machine has encountered a fault condition.
    Faulted,
    
    /// Machine is disabled and cannot operate.
    Disabled,
}

impl MachineState {
    /// Gets the string representation of this machine state.
    pub fn as_str(&self) -> &'static str {
        match self {
            MachineState::Idle => "idle",
            MachineState::Running => "running",
            MachineState::Paused => "paused",
            MachineState::Faulted => "faulted",
            MachineState::Disabled => "disabled",
        }
    }
}

impl std::fmt::Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a machine state transition.
///
/// Contains information about the transition that occurred, including source and target states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateTransition {
    /// The previous state of the machine.
    pub from: MachineState,
    
    /// The new state of the machine.
    pub to: MachineState,
    
    /// Optional reason for the transition.
    pub reason: Option<String>,
}

/// Represents a fault condition that can occur in a machine.
///
/// Faults are recorded and can be cleared when the machine is recovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineFault {
    /// The type of fault that occurred.
    pub fault_type: String,
    
    /// A description of the fault condition.
    pub description: String,
    
    /// Timestamp or context information about when the fault occurred.
    pub timestamp: u64,
}

impl MachineFault {
    /// Creates a new machine fault.
    ///
    /// # Arguments
    /// * `fault_type` - Type identifier for the fault
    /// * `description` - Description of the fault condition
    /// * `timestamp` - Timestamp or context information
    ///
    /// # Returns
    /// A new MachineFault instance
    pub fn new(fault_type: String, description: String, timestamp: u64) -> Self {
        MachineFault {
            fault_type,
            description,
            timestamp,
        }
    }
}

/// Error type for machine state transition operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    /// Attempted an invalid state transition.
    InvalidTransition {
        from: MachineState,
        to: MachineState,
        reason: String,
    },
    
    /// Attempted to clear a fault when no fault exists.
    NoFaultToClear,
    
    /// Attempted to recover from a fault that cannot be recovered from.
    CannotRecoverFromFault {
        fault_type: String,
        reason: String,
    },
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::InvalidTransition { from, to, reason } => {
                write!(f, "Invalid transition from {} to {}: {}", from, to, reason)
            }
            StateError::NoFaultToClear => {
                write!(f, "No fault to clear")
            }
            StateError::CannotRecoverFromFault { fault_type, reason } => {
                write!(f, "Cannot recover from fault type {}: {}", fault_type, reason)
            }
        }
    }
}

impl std::error::Error for StateError {}

/// Manages the state of a machine instance.
///
/// This struct provides controlled access to machine state transitions and fault management
/// while maintaining the integrity of the machine's operational lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineStateTracker {
    /// The current state of the machine.
    current_state: MachineState,
    
    /// List of recent state transitions for tracking purposes.
    transition_history: VecDeque<StateTransition>,
    
    /// Current fault information, if any.
    current_fault: Option<MachineFault>,
    
    /// Maximum number of transitions to keep in history.
    max_history: usize,
}

impl MachineStateTracker {
    /// Creates a new machine state tracker with the specified initial state.
    ///
    /// # Arguments
    /// * `initial_state` - The initial state for the machine
    /// * `max_history` - Maximum number of transitions to track (default 10)
    ///
    /// # Returns
    /// A new MachineStateTracker instance
    pub fn new(initial_state: MachineState, max_history: usize) -> Self {
        MachineStateTracker {
            current_state: initial_state,
            transition_history: VecDeque::new(),
            current_fault: None,
            max_history,
        }
    }

    /// Gets the current state of the machine.
    pub fn current_state(&self) -> MachineState {
        self.current_state
    }

    /// Checks if the machine is currently idle.
    pub fn is_idle(&self) -> bool {
        matches!(self.current_state, MachineState::Idle)
    }

    /// Checks if the machine is currently running.
    pub fn is_running(&self) -> bool {
        matches!(self.current_state, MachineState::Running)
    }

    /// Checks if the machine is currently paused.
    pub fn is_paused(&self) -> bool {
        matches!(self.current_state, MachineState::Paused)
    }

    /// Checks if the machine has a fault.
    pub fn has_fault(&self) -> bool {
        self.current_fault.is_some()
    }

    /// Gets the current fault information, if any.
    pub fn current_fault(&self) -> Option<&MachineFault> {
        self.current_fault.as_ref()
    }

    /// Checks if the machine is disabled.
    pub fn is_disabled(&self) -> bool {
        matches!(self.current_state, MachineState::Disabled)
    }

    /// Attempts to transition the machine to a new state.
    ///
    /// # Arguments
    /// * `new_state` - The target state for the transition
    /// * `reason` - Optional reason for the transition
    ///
    /// # Returns
    /// `Ok(())` if the transition was successful, or `StateError` if it was invalid
    pub fn transition_to(&mut self, new_state: MachineState, reason: Option<String>) -> Result<(), StateError> {
        let from = self.current_state;
        
        // Validate the transition
        if !Self::is_valid_transition(from, new_state) {
            return Err(StateError::InvalidTransition {
                from,
                to: new_state,
                reason: format!("Invalid transition from {} to {}", from, new_state),
            });
        }
        
        // Handle special case: transitioning to Faulted state
        if new_state == MachineState::Faulted && from != MachineState::Faulted {
            // If we're transitioning to faulted and there's no existing fault,
            // we need to record a default fault (this would be extended in real implementation)
        }
        
        // Handle special case: transitioning from Faulted state
        if from == MachineState::Faulted && new_state != MachineState::Faulted {
            // Clear the fault when leaving faulted state
            self.current_fault = None;
        }
        
        // Update current state
        self.current_state = new_state;
        
        // Record the transition
        let transition = StateTransition {
            from,
            to: new_state,
            reason,
        };
        
        // Maintain history size limit
        if self.transition_history.len() >= self.max_history {
            self.transition_history.pop_front();
        }
        self.transition_history.push_back(transition);
        
        Ok(())
    }

    /// Validates whether a state transition is allowed.
    ///
    /// # Arguments
    /// * `from` - The source state
    /// * `to` - The target state
    ///
    /// # Returns
    /// `true` if the transition is valid, `false` otherwise
    fn is_valid_transition(from: MachineState, to: MachineState) -> bool {
        match (from, to) {
            // From Idle
            (MachineState::Idle, MachineState::Running) => true,
            (MachineState::Idle, MachineState::Paused) => true,
            (MachineState::Idle, MachineState::Disabled) => true,
            
            // From Running
            (MachineState::Running, MachineState::Paused) => true,
            (MachineState::Running, MachineState::Faulted) => true,
            (MachineState::Running, MachineState::Disabled) => true,
            
            // From Paused
            (MachineState::Paused, MachineState::Running) => true,
            (MachineState::Paused, MachineState::Faulted) => true,
            (MachineState::Paused, MachineState::Disabled) => true,
            
            // From Faulted
            (MachineState::Faulted, MachineState::Idle) => true,
            (MachineState::Faulted, MachineState::Running) => true,
            (MachineState::Faulted, MachineState::Paused) => true,
            (MachineState::Faulted, MachineState::Disabled) => true,
            
            // From Disabled
            (MachineState::Disabled, MachineState::Idle) => true,
            (MachineState::Disabled, MachineState::Running) => true,
            (MachineState::Disabled, MachineState::Paused) => true,
            
            // Same state transitions are always valid
            (from_state, to_state) if from_state == to_state => true,
            
            // All other transitions are invalid
            _ => false,
        }
    }

    /// Records a fault condition for the machine.
    ///
    /// # Arguments
    /// * `fault_type` - Type identifier for the fault
    /// * `description` - Description of the fault condition
    /// * `timestamp` - Timestamp or context information
    ///
    /// # Returns
    /// `Ok(())` if the fault was recorded successfully, or `StateError` if it couldn't be recorded
    pub fn record_fault(&mut self, fault_type: String, description: String, timestamp: u64) -> Result<(), StateError> {
        // If there's already a fault, we might want to handle this differently
        // For now, we'll just replace it with the new one
        self.current_fault = Some(MachineFault::new(fault_type, description, timestamp));
        
        // Transition to Faulted state automatically
        if self.current_state != MachineState::Faulted {
            self.transition_to(MachineState::Faulted, Some("Fault recorded".to_string()))
                .map_err(|e| StateError::InvalidTransition {
                    from: self.current_state,
                    to: MachineState::Faulted,
                    reason: format!("Failed to transition to faulted state: {}", e),
                })?;
        }
        
        Ok(())
    }

    /// Clears the current fault condition.
    ///
    /// # Returns
    /// `Ok(())` if the fault was cleared successfully, or `StateError` if there was no fault to clear
    pub fn clear_fault(&mut self) -> Result<(), StateError> {
        if self.current_fault.is_none() {
            return Err(StateError::NoFaultToClear);
        }
        
        self.current_fault = None;
        
        // If we were in faulted state, transition back to idle or previous non-fault state
        // For simplicity, we'll transition to idle when clearing a fault
        if self.current_state == MachineState::Faulted {
            self.transition_to(MachineState::Idle, Some("Fault cleared".to_string()))
                .map_err(|e| StateError::InvalidTransition {
                    from: MachineState::Faulted,
                    to: MachineState::Idle,
                    reason: format!("Failed to transition after clearing fault: {}", e),
                })?;
        }
        
        Ok(())
    }

    /// Attempts to recover the machine from a fault.
    ///
    /// # Arguments
    /// * `fault_type` - The type of fault to attempt recovery from
    /// * `reason` - Reason for the recovery attempt
    ///
    /// # Returns
    /// `Ok(())` if recovery was successful, or `StateError` if it failed
    pub fn recover_from_fault(&mut self, fault_type: &str, reason: &str) -> Result<(), StateError> {
        // Check if there's a fault to recover from
        if let Some(fault) = &self.current_fault {
            if fault.fault_type != fault_type {
                return Err(StateError::CannotRecoverFromFault {
                    fault_type: fault_type.to_string(),
                    reason: "Fault type mismatch".to_string(),
                });
            }
            
            // In a real implementation, we might have different recovery rules
            // For now, we'll just clear the fault and transition to idle
            self.clear_fault()?;
            
            Ok(())
        } else {
            Err(StateError::NoFaultToClear)
        }
    }

    /// Gets the recent state transition history.
    ///
    /// # Returns
    /// An iterator over recent state transitions (most recent first)
    pub fn transition_history(&self) -> impl Iterator<Item = &StateTransition> {
        self.transition_history.iter().rev()
    }

    /// Gets the maximum history size.
    pub fn max_history(&self) -> usize {
        self.max_history
    }
}

impl Default for MachineStateTracker {
    /// Creates a new machine state tracker with Idle as the initial state and default history size.
    ///
    /// This is primarily for testing and should not be used in production.
    fn default() -> Self {
        MachineStateTracker::new(MachineState::Idle, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_state_display() {
        assert_eq!(format!("{}", MachineState::Idle), "idle");
        assert_eq!(format!("{}", MachineState::Running), "running");
        assert_eq!(format!("{}", MachineState::Paused), "paused");
        assert_eq!(format!("{}", MachineState::Faulted), "faulted");
        assert_eq!(format!("{}", MachineState::Disabled), "disabled");
    }

    #[test]
    fn test_state_transition_creation() {
        let transition = StateTransition {
            from: MachineState::Idle,
            to: MachineState::Running,
            reason: Some("User initiated".to_string()),
        };
        
        assert_eq!(transition.from, MachineState::Idle);
        assert_eq!(transition.to, MachineState::Running);
        assert_eq!(transition.reason, Some("User initiated".to_string()));
    }

    #[test]
    fn test_machine_fault_creation() {
        let fault = MachineFault::new(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        
        assert_eq!(fault.fault_type, "overheating");
        assert_eq!(fault.description, "Temperature exceeded safe limits");
        assert_eq!(fault.timestamp, 1234567890);
    }

    #[test]
    fn test_state_error_display() {
        let error = StateError::InvalidTransition {
            from: MachineState::Idle,
            to: MachineState::Faulted,
            reason: "Test reason".to_string(),
        };
        assert_eq!(format!("{}", error), "Invalid transition from idle to faulted: Test reason");
        
        let error2 = StateError::NoFaultToClear;
        assert_eq!(format!("{}", error2), "No fault to clear");
        
        let error3 = StateError::CannotRecoverFromFault {
            fault_type: "overheating".to_string(),
            reason: "Test recovery reason".to_string(),
        };
        assert_eq!(format!("{}", error3), "Cannot recover from fault type overheating: Test recovery reason");
    }

    #[test]
    fn test_machine_state_tracker_creation() {
        let tracker = MachineStateTracker::new(MachineState::Running, 5);
        assert_eq!(tracker.current_state(), MachineState::Running);
        assert_eq!(tracker.max_history(), 5);
        assert!(!tracker.has_fault());
        assert!(!tracker.is_running());
        assert!(tracker.is_running());
    }

    #[test]
    fn test_machine_state_tracker_default() {
        let tracker = MachineStateTracker::default();
        assert_eq!(tracker.current_state(), MachineState::Idle);
        assert_eq!(tracker.max_history(), 10);
        assert!(!tracker.has_fault());
        assert!(tracker.is_idle());
    }

    #[test]
    fn test_valid_transitions() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Valid transitions from Idle
        assert!(tracker.transition_to(MachineState::Running, None).is_ok());
        assert!(tracker.transition_to(MachineState::Paused, None).is_ok());
        assert!(tracker.transition_to(MachineState::Disabled, None).is_ok());
        
        // Valid transitions from Running
        tracker = MachineStateTracker::new(MachineState::Running, 10);
        assert!(tracker.transition_to(MachineState::Paused, None).is_ok());
        assert!(tracker.transition_to(MachineState::Faulted, None).is_ok());
        assert!(tracker.transition_to(MachineState::Disabled, None).is_ok());
        
        // Valid transitions from Paused
        tracker = MachineStateTracker::new(MachineState::Paused, 10);
        assert!(tracker.transition_to(MachineState::Running, None).is_ok());
        assert!(tracker.transition_to(MachineState::Faulted, None).is_ok());
        assert!(tracker.transition_to(MachineState::Disabled, None).is_ok());
        
        // Valid transitions from Faulted
        tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        assert!(tracker.transition_to(MachineState::Idle, None).is_ok());
        assert!(tracker.transition_to(MachineState::Running, None).is_ok());
        assert!(tracker.transition_to(MachineState::Paused, None).is_ok());
        assert!(tracker.transition_to(MachineState::Disabled, None).is_ok());
        
        // Valid transitions from Disabled
        tracker = MachineStateTracker::new(MachineState::Disabled, 10);
        assert!(tracker.transition_to(MachineState::Idle, None).is_ok());
        assert!(tracker.transition_to(MachineState::Running, None).is_ok());
        assert!(tracker.transition_to(MachineState::Paused, None).is_ok());
    }

    #[test]
    fn test_invalid_transitions() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Invalid transitions from Idle
        assert!(matches!(
            tracker.transition_to(MachineState::Faulted, None),
            Err(StateError::InvalidTransition { .. })
        ));
        assert!(matches!(
            tracker.transition_to(MachineState::Disabled, None),
            Err(StateError::InvalidTransition { .. })
        ));
        
        // Invalid transitions from Running
        tracker = MachineStateTracker::new(MachineState::Running, 10);
        assert!(matches!(
            tracker.transition_to(MachineState::Idle, None),
            Err(StateError::InvalidTransition { .. })
        ));
        assert!(matches!(
            tracker.transition_to(MachineState::Faulted, None),
            Err(StateError::InvalidTransition { .. })
        ));
        
        // Invalid transitions from Paused
        tracker = MachineStateTracker::new(MachineState::Paused, 10);
        assert!(matches!(
            tracker.transition_to(MachineState::Running, None),
            Err(StateError::InvalidTransition { .. })
        ));
        assert!(matches!(
            tracker.transition_to(MachineState::Idle, None),
            Err(StateError::InvalidTransition { .. })
        ));
        
        // Invalid transitions from Faulted
        tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        assert!(matches!(
            tracker.transition_to(MachineState::Running, None),
            Err(StateError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn test_same_state_transitions() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Same state transitions should be valid
        assert!(tracker.transition_to(MachineState::Idle, None).is_ok());
        assert_eq!(tracker.current_state(), MachineState::Idle);
        
        tracker = MachineStateTracker::new(MachineState::Running, 10);
        assert!(tracker.transition_to(MachineState::Running, None).is_ok());
        assert_eq!(tracker.current_state(), MachineState::Running);
    }

    #[test]
    fn test_fault_recording() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Record a fault
        let result = tracker.record_fault(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        assert!(result.is_ok());
        assert!(tracker.has_fault());
        assert_eq!(tracker.current_state(), MachineState::Faulted);
        
        // Check fault details
        let fault = tracker.current_fault().unwrap();
        assert_eq!(fault.fault_type, "overheating");
        assert_eq!(fault.description, "Temperature exceeded safe limits");
        assert_eq!(fault.timestamp, 1234567890);
    }

    #[test]
    fn test_fault_clearing() {
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        
        // Record a fault first
        let _ = tracker.record_fault(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        
        assert!(tracker.has_fault());
        assert_eq!(tracker.current_state(), MachineState::Faulted);
        
        // Clear the fault
        let result = tracker.clear_fault();
        assert!(result.is_ok());
        assert!(!tracker.has_fault());
        assert_eq!(tracker.current_state(), MachineState::Idle); // Should transition to idle
    }

    #[test]
    fn test_cannot_clear_no_fault() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Try to clear a fault when none exists
        let result = tracker.clear_fault();
        assert!(matches!(result, Err(StateError::NoFaultToClear)));
    }

    #[test]
    fn test_recover_from_fault() {
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        
        // Record a fault first
        let _ = tracker.record_fault(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        
        assert!(tracker.has_fault());
        
        // Recover from the fault
        let result = tracker.recover_from_fault("overheating", "System cooled down");
        assert!(result.is_ok());
        assert!(!tracker.has_fault());
        assert_eq!(tracker.current_state(), MachineState::Idle);
    }

    #[test]
    fn test_cannot_recover_wrong_fault_type() {
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        
        // Record a fault first
        let _ = tracker.record_fault(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        
        // Try to recover from wrong fault type
        let result = tracker.recover_from_fault("other_error", "System cooled down");
        assert!(matches!(result, Err(StateError::CannotRecoverFromFault { .. })));
    }

    #[test]
    fn test_cannot_recover_no_fault() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Try to recover from fault when none exists
        let result = tracker.recover_from_fault("overheating", "System cooled down");
        assert!(matches!(result, Err(StateError::NoFaultToClear)));
    }

    #[test]
    fn test_transition_history() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 3);
        
        // Perform several transitions
        let _ = tracker.transition_to(MachineState::Running, Some("Started".to_string()));
        let _ = tracker.transition_to(MachineState::Paused, Some("Paused".to_string()));
        let _ = tracker.transition_to(MachineState::Running, Some("Resumed".to_string()));
        
        // Check history
        let history: Vec<_> = tracker.transition_history().collect();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].from, MachineState::Paused);
        assert_eq!(history[0].to, MachineState::Running);
        assert_eq!(history[1].from, MachineState::Running);
        assert_eq!(history[1].to, MachineState::Paused);
        assert_eq!(history[2].from, MachineState::Idle);
        assert_eq!(history[2].to, MachineState::Running);
    }

    #[test]
    fn test_transition_history_limit() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 2);
        
        // Perform more transitions than history limit
        let _ = tracker.transition_to(MachineState::Running, None);
        let _ = tracker.transition_to(MachineState::Paused, None);
        let _ = tracker.transition_to(MachineState::Running, None);
        let _ = tracker.transition_to(MachineState::Paused, None);
        
        // History should only contain the last 2 transitions
        let history: Vec<_> = tracker.transition_history().collect();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].from, MachineState::Running);
        assert_eq!(history[0].to, MachineState::Paused);
        assert_eq!(history[1].from, MachineState::Paused);
        assert_eq!(history[1].to, MachineState::Running);
    }

    #[test]
    fn test_state_checking_methods() {
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        
        // Test idle state
        assert!(tracker.is_idle());
        assert!(!tracker.is_running());
        assert!(!tracker.is_paused());
        assert!(!tracker.is_disabled());
        
        // Test running state
        let _ = tracker.transition_to(MachineState::Running, None);
        assert!(!tracker.is_idle());
        assert!(tracker.is_running());
        assert!(!tracker.is_paused());
        assert!(!tracker.is_disabled());
        
        // Test paused state
        let _ = tracker.transition_to(MachineState::Paused, None);
        assert!(!tracker.is_idle());
        assert!(!tracker.is_running());
        assert!(tracker.is_paused());
        assert!(!tracker.is_disabled());
        
        // Test disabled state
        let _ = tracker.transition_to(MachineState::Disabled, None);
        assert!(!tracker.is_idle());
        assert!(!tracker.is_running());
        assert!(!tracker.is_paused());
        assert!(tracker.is_disabled());
    }
}