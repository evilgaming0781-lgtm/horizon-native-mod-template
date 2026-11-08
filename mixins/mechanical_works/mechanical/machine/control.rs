//! Machine control system for Horizon Mechanical Works.
//!
//! This module provides the control layer for machine operations, including
//! start, stop, pause, resume, and reset commands with proper validation.

use crate::mechanical::machine::{state::*, types::*};
use std::collections::HashMap;

/// Represents the result of a machine control command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlResult {
    /// Whether the command was successful.
    pub success: bool,
    
    /// The new state of the machine after command execution (if applicable).
    pub new_state: Option<MachineState>,
    
    /// Optional message describing the result or reason for failure.
    pub message: String,
}

impl ControlResult {
    /// Creates a new successful control result.
    pub fn success(new_state: MachineState, message: String) -> Self {
        ControlResult {
            success: true,
            new_state: Some(new_state),
            message,
        }
    }

    /// Creates a new failed control result.
    pub fn failure(message: String) -> Self {
        ControlResult {
            success: false,
            new_state: None,
            message,
        }
    }
}

/// Error type for machine control operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlError {
    /// The command was invalid for the current machine state.
    InvalidCommand {
        command: String,
        current_state: MachineState,
        reason: String,
    },
    
    /// A fault occurred that prevents the command from being executed.
    FaultedMachine {
        fault_type: String,
        description: String,
    },
}

impl std::fmt::Display for ControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlError::InvalidCommand { command, current_state, reason } => {
                write!(f, "Invalid command '{}' for state '{}': {}", command, current_state, reason)
            }
            ControlError::FaultedMachine { fault_type, description } => {
                write!(f, "Cannot execute command on faulted machine ({}): {}", fault_type, description)
            }
        }
    }
}

impl std::error::Error for ControlError {}

/// Represents a machine control command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCommand {
    /// Start the machine operation.
    Start,
    
    /// Stop the machine operation.
    Stop,
    
    /// Pause the machine operation.
    Pause,
    
    /// Resume the machine operation.
    Resume,
    
    /// Reset the machine to its initial state.
    Reset,
}

impl MachineCommand {
    /// Gets the string representation of this command.
    pub fn as_str(&self) -> &'static str {
        match self {
            MachineCommand::Start => "start",
            MachineCommand::Stop => "stop",
            MachineCommand::Pause => "pause",
            MachineCommand::Resume => "resume",
            MachineCommand::Reset => "reset",
        }
    }
}

impl std::fmt::Display for MachineCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Controls machine operations through state transitions.
///
/// This struct provides methods to execute control commands on machines
/// while validating that the commands are appropriate for the current state.
pub struct MachineController;

impl MachineController {
    /// Executes a control command on a machine.
    ///
    /// # Arguments
    /// * `command` - The command to execute
    /// * `machine` - Reference to the machine to control
    /// * `state_tracker` - Reference to the machine's state tracker
    ///
    /// # Returns
    /// A ControlResult indicating success or failure of the command
    pub fn execute_command(
        command: MachineCommand,
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        // Check if machine is faulted first
        if let Some(fault) = state_tracker.current_fault() {
            return Err(ControlError::FaultedMachine {
                fault_type: fault.fault_type.clone(),
                description: fault.description.clone(),
            });
        }

        // Validate command against current state
        match command {
            MachineCommand::Start => Self::validate_and_execute_start(machine, state_tracker),
            MachineCommand::Stop => Self::validate_and_execute_stop(machine, state_tracker),
            MachineCommand::Pause => Self::validate_and_execute_pause(machine, state_tracker),
            MachineCommand::Resume => Self::validate_and_execute_resume(machine, state_tracker),
            MachineCommand::Reset => Self::validate_and_execute_reset(machine, state_tracker),
        }
    }

    /// Validates and executes the Start command.
    fn validate_and_execute_start(
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        let current_state = state_tracker.current_state();
        
        match current_state {
            MachineState::Idle => {
                // Can start from idle
                state_tracker.transition_to(MachineState::Running, Some("Started by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "start".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Running,
                    "Machine started successfully".to_string(),
                ))
            }
            MachineState::Paused => {
                // Can resume from paused state
                state_tracker.transition_to(MachineState::Running, Some("Resumed by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "start".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Running,
                    "Machine resumed from paused state".to_string(),
                ))
            }
            MachineState::Running => {
                // Already running, no change needed
                Ok(ControlResult::success(
                    MachineState::Running,
                    "Machine is already running".to_string(),
                ))
            }
            MachineState::Disabled => {
                // Cannot start a disabled machine
                Err(ControlError::InvalidCommand {
                    command: "start".to_string(),
                    current_state,
                    reason: "Cannot start a disabled machine".to_string(),
                })
            }
            MachineState::Faulted => {
                // Cannot start a faulted machine
                Err(ControlError::InvalidCommand {
                    command: "start".to_string(),
                    current_state,
                    reason: "Cannot start a faulted machine".to_string(),
                })
            }
        }
    }

    /// Validates and executes the Stop command.
    fn validate_and_execute_stop(
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        let current_state = state_tracker.current_state();
        
        match current_state {
            MachineState::Running => {
                // Can stop from running
                state_tracker.transition_to(MachineState::Idle, Some("Stopped by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "stop".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Idle,
                    "Machine stopped successfully".to_string(),
                ))
            }
            MachineState::Paused => {
                // Can stop from paused
                state_tracker.transition_to(MachineState::Idle, Some("Stopped by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "stop".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Idle,
                    "Machine stopped from paused state".to_string(),
                ))
            }
            MachineState::Idle => {
                // Already idle, no change needed
                Ok(ControlResult::success(
                    MachineState::Idle,
                    "Machine is already idle".to_string(),
                ))
            }
            MachineState::Disabled => {
                // Already disabled, no change needed
                Ok(ControlResult::success(
                    MachineState::Disabled,
                    "Machine is disabled (already stopped)".to_string(),
                ))
            }
            MachineState::Faulted => {
                // Can stop from faulted state
                state_tracker.transition_to(MachineState::Idle, Some("Stopped due to fault".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "stop".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Idle,
                    "Machine stopped due to fault condition".to_string(),
                ))
            }
        }
    }

    /// Validates and executes the Pause command.
    fn validate_and_execute_pause(
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        let current_state = state_tracker.current_state();
        
        match current_state {
            MachineState::Running => {
                // Can pause from running
                state_tracker.transition_to(MachineState::Paused, Some("Paused by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "pause".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Paused,
                    "Machine paused successfully".to_string(),
                ))
            }
            MachineState::Idle => {
                // Cannot pause from idle
                Err(ControlError::InvalidCommand {
                    command: "pause".to_string(),
                    current_state,
                    reason: "Cannot pause an idle machine".to_string(),
                })
            }
            MachineState::Paused => {
                // Already paused, no change needed
                Ok(ControlResult::success(
                    MachineState::Paused,
                    "Machine is already paused".to_string(),
                ))
            }
            MachineState::Disabled => {
                // Cannot pause a disabled machine
                Err(ControlError::InvalidCommand {
                    command: "pause".to_string(),
                    current_state,
                    reason: "Cannot pause a disabled machine".to_string(),
                })
            }
            MachineState::Faulted => {
                // Cannot pause a faulted machine
                Err(ControlError::InvalidCommand {
                    command: "pause".to_string(),
                    current_state,
                    reason: "Cannot pause a faulted machine".to_string(),
                })
            }
        }
    }

    /// Validates and executes the Resume command.
    fn validate_and_execute_resume(
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        let current_state = state_tracker.current_state();
        
        match current_state {
            MachineState::Paused => {
                // Can resume from paused
                state_tracker.transition_to(MachineState::Running, Some("Resumed by command".to_string()))
                    .map_err(|e| ControlError::InvalidCommand {
                        command: "resume".to_string(),
                        current_state,
                        reason: format!("Failed to transition: {}", e),
                    })?;
                
                Ok(ControlResult::success(
                    MachineState::Running,
                    "Machine resumed successfully".to_string(),
                ))
            }
            MachineState::Idle => {
                // Cannot resume from idle
                Err(ControlError::InvalidCommand {
                    command: "resume".to_string(),
                    current_state,
                    reason: "Cannot resume an idle machine".to_string(),
                })
            }
            MachineState::Running => {
                // Already running, no change needed
                Ok(ControlResult::success(
                    MachineState::Running,
                    "Machine is already running".to_string(),
                ))
            }
            MachineState::Disabled => {
                // Cannot resume a disabled machine
                Err(ControlError::InvalidCommand {
                    command: "resume".to_string(),
                    current_state,
                    reason: "Cannot resume a disabled machine".to_string(),
                })
            }
            MachineState::Faulted => {
                // Cannot resume a faulted machine
                Err(ControlError::InvalidCommand {
                    command: "resume".to_string(),
                    current_state,
                    reason: "Cannot resume a faulted machine".to_string(),
                })
            }
        }
    }

    /// Validates and executes the Reset command.
    fn validate_and_execute_reset(
        machine: &Machine,
        state_tracker: &mut MachineStateTracker,
    ) -> Result<ControlResult, ControlError> {
        let current_state = state_tracker.current_state();
        
        // Reset always transitions to idle state
        state_tracker.transition_to(MachineState::Idle, Some("Reset by command".to_string()))
            .map_err(|e| ControlError::InvalidCommand {
                command: "reset".to_string(),
                current_state,
                reason: format!("Failed to transition: {}", e),
            })?;
        
        Ok(ControlResult::success(
            MachineState::Idle,
            "Machine reset successfully".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_result_creation() {
        let success_result = ControlResult::success(MachineState::Running, "Test message".to_string());
        assert!(success_result.success);
        assert_eq!(success_result.new_state, Some(MachineState::Running));
        assert_eq!(success_result.message, "Test message");

        let failure_result = ControlResult::failure("Error message".to_string());
        assert!(!failure_result.success);
        assert_eq!(failure_result.new_state, None);
        assert_eq!(failure_result.message, "Error message");
    }

    #[test]
    fn test_control_error_display() {
        let error = ControlError::InvalidCommand {
            command: "start".to_string(),
            current_state: MachineState::Running,
            reason: "Test reason".to_string(),
        };
        assert_eq!(format!("{}", error), "Invalid command 'start' for state 'running': Test reason");

        let error2 = ControlError::FaultedMachine {
            fault_type: "overheating".to_string(),
            description: "Temperature exceeded limits".to_string(),
        };
        assert_eq!(format!("{}", error2), "Cannot execute command on faulted machine (overheating): Temperature exceeded limits");
    }

    #[test]
    fn test_machine_command_display() {
        assert_eq!(format!("{}", MachineCommand::Start), "start");
        assert_eq!(format!("{}", MachineCommand::Stop), "stop");
        assert_eq!(format!("{}", MachineCommand::Pause), "pause");
        assert_eq!(format!("{}", MachineCommand::Resume), "resume");
        assert_eq!(format!("{}", MachineCommand::Reset), "reset");
    }

    #[test]
    fn test_start_command_valid_transitions() {
        // Test starting from idle
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Running));
        assert_eq!(result.message, "Machine started successfully");
        assert_eq!(tracker.current_state(), MachineState::Running);

        // Test starting from paused
        let mut tracker = MachineStateTracker::new(MachineState::Paused, 10);
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Running));
        assert_eq!(result.message, "Machine resumed from paused state");
        assert_eq!(tracker.current_state(), MachineState::Running);

        // Test starting from running (should be no-op)
        let mut tracker = MachineStateTracker::new(MachineState::Running, 10);
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Running));
        assert_eq!(result.message, "Machine is already running");
        assert_eq!(tracker.current_state(), MachineState::Running);
    }

    #[test]
    fn test_start_command_invalid_transitions() {
        // Test starting from disabled
        let mut tracker = MachineStateTracker::new(MachineState::Disabled, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "start");
                assert_eq!(current_state, MachineState::Disabled);
                assert_eq!(reason, "Cannot start a disabled machine");
            }
        }

        // Test starting from faulted
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "start");
                assert_eq!(current_state, MachineState::Faulted);
                assert_eq!(reason, "Cannot start a faulted machine");
            }
        }
    }

    #[test]
    fn test_stop_command_valid_transitions() {
        // Test stopping from running
        let mut tracker = MachineStateTracker::new(MachineState::Running, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Stop, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Idle));
        assert_eq!(result.message, "Machine stopped successfully");
        assert_eq!(tracker.current_state(), MachineState::Idle);

        // Test stopping from paused
        let mut tracker = MachineStateTracker::new(MachineState::Paused, 10);
        let result = MachineController::execute_command(MachineCommand::Stop, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Idle));
        assert_eq!(result.message, "Machine stopped from paused state");
        assert_eq!(tracker.current_state(), MachineState::Idle);

        // Test stopping from idle (should be no-op)
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        let result = MachineController::execute_command(MachineCommand::Stop, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Idle));
        assert_eq!(result.message, "Machine is already idle");
        assert_eq!(tracker.current_state(), MachineState::Idle);

        // Test stopping from disabled (should be no-op)
        let mut tracker = MachineStateTracker::new(MachineState::Disabled, 10);
        let result = MachineController::execute_command(MachineCommand::Stop, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Disabled));
        assert_eq!(result.message, "Machine is disabled (already stopped)");
        assert_eq!(tracker.current_state(), MachineState::Disabled);

        // Test stopping from faulted
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        let result = MachineController::execute_command(MachineCommand::Stop, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Idle));
        assert_eq!(result.message, "Machine stopped due to fault condition");
        assert_eq!(tracker.current_state(), MachineState::Idle);
    }

    #[test]
    fn test_pause_command_valid_transitions() {
        // Test pausing from running
        let mut tracker = MachineStateTracker::new(MachineState::Running, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Pause, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Paused));
        assert_eq!(result.message, "Machine paused successfully");
        assert_eq!(tracker.current_state(), MachineState::Paused);

        // Test pausing from paused (should be no-op)
        let mut tracker = MachineStateTracker::new(MachineState::Paused, 10);
        let result = MachineController::execute_command(MachineCommand::Pause, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Paused));
        assert_eq!(result.message, "Machine is already paused");
        assert_eq!(tracker.current_state(), MachineState::Paused);
    }

    #[test]
    fn test_pause_command_invalid_transitions() {
        // Test pausing from idle
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Pause, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "pause");
                assert_eq!(current_state, MachineState::Idle);
                assert_eq!(reason, "Cannot pause an idle machine");
            }
        }

        // Test pausing from disabled
        let mut tracker = MachineStateTracker::new(MachineState::Disabled, 10);
        let result = MachineController::execute_command(MachineCommand::Pause, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "pause");
                assert_eq!(current_state, MachineState::Disabled);
                assert_eq!(reason, "Cannot pause a disabled machine");
            }
        }

        // Test pausing from faulted
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        let result = MachineController::execute_command(MachineCommand::Pause, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "pause");
                assert_eq!(current_state, MachineState::Faulted);
                assert_eq!(reason, "Cannot pause a faulted machine");
            }
        }
    }

    #[test]
    fn test_resume_command_valid_transitions() {
        // Test resuming from paused
        let mut tracker = MachineStateTracker::new(MachineState::Paused, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Resume, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Running));
        assert_eq!(result.message, "Machine resumed successfully");
        assert_eq!(tracker.current_state(), MachineState::Running);

        // Test resuming from running (should be no-op)
        let mut tracker = MachineStateTracker::new(MachineState::Running, 10);
        let result = MachineController::execute_command(MachineCommand::Resume, &machine, &mut tracker).unwrap();
        assert!(result.success);
        assert_eq!(result.new_state, Some(MachineState::Running));
        assert_eq!(result.message, "Machine is already running");
        assert_eq!(tracker.current_state(), MachineState::Running);
    }

    #[test]
    fn test_resume_command_invalid_transitions() {
        // Test resuming from idle
        let mut tracker = MachineStateTracker::new(MachineState::Idle, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        let result = MachineController::execute_command(MachineCommand::Resume, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "resume");
                assert_eq!(current_state, MachineState::Idle);
                assert_eq!(reason, "Cannot resume an idle machine");
            }
        }

        // Test resuming from disabled
        let mut tracker = MachineStateTracker::new(MachineState::Disabled, 10);
        let result = MachineController::execute_command(MachineCommand::Resume, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "resume");
                assert_eq!(current_state, MachineState::Disabled);
                assert_eq!(reason, "Cannot resume a disabled machine");
            }
        }

        // Test resuming from faulted
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        let result = MachineController::execute_command(MachineCommand::Resume, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::InvalidCommand { command, current_state, reason } => {
                assert_eq!(command, "resume");
                assert_eq!(current_state, MachineState::Faulted);
                assert_eq!(reason, "Cannot resume a faulted machine");
            }
        }
    }

    #[test]
    fn test_reset_command() {
        // Test reset from any state
        let states = vec![
            MachineState::Idle,
            MachineState::Running,
            MachineState::Paused,
            MachineState::Disabled,
            MachineState::Faulted,
        ];
        
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        for state in states {
            let mut tracker = MachineStateTracker::new(state, 10);
            let result = MachineController::execute_command(MachineCommand::Reset, &machine, &mut tracker).unwrap();
            assert!(result.success);
            assert_eq!(result.new_state, Some(MachineState::Idle));
            assert_eq!(result.message, "Machine reset successfully");
            assert_eq!(tracker.current_state(), MachineState::Idle);
        }
    }

    #[test]
    fn test_faulted_machine_prevents_commands() {
        let mut tracker = MachineStateTracker::new(MachineState::Faulted, 10);
        let specs = MachineSpecs::new(
            MachineTypeId::new("test_machine".to_string()),
            "Test Machine".to_string(),
            "A test machine".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let config = MachineConfig::new(
            MachineId::new("test_001".to_string()),
            MachineTypeId::new("test_machine".to_string()),
            Watts::new(500),
            HashMap::new(),
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        // Record a fault
        let _ = tracker.record_fault(
            "overheating".to_string(),
            "Temperature exceeded safe limits".to_string(),
            1234567890,
        );
        
        // Try to execute any command - should fail due to fault
        let result = MachineController::execute_command(MachineCommand::Start, &machine, &mut tracker);
        assert!(result.is_err());
        match result.unwrap_err() {
            ControlError::FaultedMachine { fault_type, description } => {
                assert_eq!(fault_type, "overheating");
                assert_eq!(description, "Temperature exceeded safe limits");
            }
        }
    }
}