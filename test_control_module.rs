// Simple test file to verify control module functionality
use horizon_mechanical_works::mechanical::machine::{control, state, types};
use std::collections::HashMap;

fn main() {
    // Test basic functionality
    println!("Testing control module...");
    
    // Create a machine for testing
    let specs = types::MachineSpecs::new(
        types::MachineTypeId::new("test_machine".to_string()),
        "Test Machine".to_string(),
        "A test machine".to_string(),
        types::Watts::new(1000),
        types::Watts::new(200),
        4,
        true,
    ).unwrap();
    
    let config = types::MachineConfig::new(
        types::MachineId::new("test_001".to_string()),
        types::MachineTypeId::new("test_machine".to_string()),
        types::Watts::new(500),
        HashMap::new(),
        true,
    ).unwrap();
    
    let machine = types::Machine::new(specs, config);
    
    // Test start command
    let mut tracker = state::MachineStateTracker::new(state::MachineState::Idle, 10);
    let result = control::MachineController::execute_command(
        control::MachineCommand::Start,
        &machine,
        &mut tracker
    );
    
    match result {
        Ok(res) => {
            println!("Start command successful: {:?}", res);
        }
        Err(e) => {
            println!("Start command failed: {}", e);
        }
    }
    
    // Test stop command
    let result = control::MachineController::execute_command(
        control::MachineCommand::Stop,
        &machine,
        &mut tracker
    );
    
    match result {
        Ok(res) => {
            println!("Stop command successful: {:?}", res);
        }
        Err(e) => {
            println!("Stop command failed: {}", e);
        }
    }
    
    println!("Control module test completed.");
}