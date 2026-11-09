//! Real mechanical evaluation engine for transmission components.
//!
//! This module implements the core evaluation logic that transforms input 
//! mechanical states into output states according to physical laws and 
//! component-specific behavior.

use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity, Efficiency};
use crate::mechanical::transmission::types::{TransmissionId, TransmissionCategory};
use crate::mechanical::transmission::interfaces::{RotationalInput, RotationalOutput};
use crate::mechanical::transmission::errors::{TransmissionError, TransmissionResult};
use crate::mechanical::transmission::integration::{IntegratedTransmission, TransmissionState};
use crate::mechanical::transmission::shaft::Shaft;
use crate::mechanical::transmission::coupling::Coupling;
use crate::mechanical::transmission::gear::Gear;
use crate::mechanical::transmission::belt::Belt;
use crate::mechanical::transmission::gear::calculations as gear_calculations;
use crate::mechanical::transmission::belt::calculations as belt_calculations;

/// Evaluates a shaft component given input conditions.
///
/// For an ideal shaft:
/// - Output angular velocity equals input angular velocity
/// - Output torque magnitude equals input torque magnitude, subject to shaft capacity
/// - Output power equals input power
///
/// # Arguments
/// * `shaft` - The shaft component to evaluate
/// * `input_state` - The input state to evaluate against
///
/// # Returns
/// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
/// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
pub fn evaluate_shaft(
    shaft: &Shaft,
    input_state: TransmissionState,
) -> Result<TransmissionState, TransmissionError> {
    if !shaft.is_enabled() {
        return Err(TransmissionError::DisabledComponent);
    }

    // Validate input state against limits
    if input_state.torque > Torque::new(shaft.configuration().max_torque) {
        return Err(TransmissionError::ExceededTorque);
    }
    if input_state.rpm > RotationalSpeed::new(shaft.configuration().max_rpm) {
        return Err(TransmissionError::ExceededRPM);
    }
    if input_state.power > Power::new(shaft.configuration().max_power) {
        return Err(TransmissionError::ExceededPower);
    }

    // For a shaft, output state matches input state (ideal pass-through)
    Ok(input_state)
}

/// Evaluates a coupling component given input conditions.
///
/// A coupling passes through the input state unchanged as long as it's valid and enabled.
///
/// # Arguments
/// * `coupling` - The coupling component to evaluate
/// * `input_state` - The input state to evaluate against
///
/// # Returns
/// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
/// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
pub fn evaluate_coupling(
    coupling: &Coupling,
    input_state: TransmissionState,
) -> Result<TransmissionState, TransmissionError> {
    if !coupling.is_enabled() {
        return Err(TransmissionError::DisabledComponent);
    }

    // Validate that coupling has two components attached (this is a design constraint)
    if coupling.component_a().is_none() || coupling.component_b().is_none() {
        return Err(TransmissionError::InvalidCoupling);
    }

    // For a coupling, we pass the input state through unchanged
    Ok(input_state)
}

/// Evaluates a gear component given input conditions.
///
/// Applies gear ratio calculations and power conservation:
/// - ω_out = ω_in × (N_in / N_out) 
/// - |τ_out| = |τ_in| × (N_out / N_in) × η
/// - P_out = P_in × η
///
/// # Arguments
/// * `gear` - The gear component to evaluate
/// * `input_state` - The input state to evaluate against
///
/// # Returns
/// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
/// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
pub fn evaluate_gear(
    gear: &Gear,
    input_state: TransmissionState,
) -> Result<TransmissionState, TransmissionError> {
    if !gear.is_enabled() {
        return Err(TransmissionError::DisabledComponent);
    }

    // Validate input state against limits
    if input_state.torque > Torque::new(gear.configuration().max_torque) {
        return Err(TransmissionError::ExceededTorque);
    }
    if input_state.rpm > RotationalSpeed::new(gear.configuration().max_rpm) {
        return Err(TransmissionError::ExceededRPM);
    }
    if input_state.power > Power::new(gear.configuration().max_power) {
        return Err(TransmissionError::ExceededPower);
    }

    // Calculate output values based on gear ratio and efficiency
    let gear_ratio = gear.configuration().ratio;
    
    // Convert RPM to angular velocity for calculations
    let input_angular_velocity = input_state.rpm.to_rad_per_sec();
    
    // Calculate output angular velocity
    let output_angular_velocity = gear_calculations::calculate_output_angular_velocity(
        input_angular_velocity,
        &gear_ratio,
    );
    
    // Convert back to RPM for output state
    let output_rpm = output_angular_velocity.to_rpm();
    
    // Calculate ideal output torque (without efficiency)
    let ideal_output_torque = gear_calculations::calculate_output_torque_ideal(
        input_state.torque,
        &gear_ratio,
    );
    
    // Apply efficiency to get actual output torque
    let output_torque = gear_calculations::calculate_output_torque_with_efficiency(
        input_state.torque,
        &gear_ratio,
        gear.configuration().efficiency,
    );
    
    // Calculate output power with efficiency
    let output_power = gear_calculations::calculate_output_power(
        input_state.power,
        gear.configuration().efficiency,
    );
    
    // Check for valid physical conditions - ensure torque and RPM are not negative
    if output_torque.get() < 0.0 || output_rpm.get() < 0.0 {
        return Err(TransmissionError::InvalidConfiguration);
    }

    Ok(TransmissionState::with_values(
        output_torque,
        output_rpm,
        output_power,
        input_state.enabled,
    ))
}

/// Evaluates a belt component given input conditions.
///
/// Applies belt ratio calculations and power conservation:
/// - ω_out = ω_in × (r_in / r_out) 
/// - |τ_out| = |τ_in| × (r_out / r_in) × η
/// - P_out = P_in × η
///
/// # Arguments
/// * `belt` - The belt component to evaluate
/// * `input_state` - The input state to evaluate against
///
/// # Returns
/// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
/// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
pub fn evaluate_belt(
    belt: &Belt,
    input_state: TransmissionState,
) -> Result<TransmissionState, TransmissionError> {
    if !belt.is_enabled() {
        return Err(TransmissionError::DisabledComponent);
    }

    // Validate input state against limits
    if input_state.torque > Torque::new(belt.configuration.max_torque) {
        return Err(TransmissionError::ExceededTorque);
    }
    if input_state.rpm > RotationalSpeed::new(belt.configuration.max_rpm) {
        return Err(TransmissionError::ExceededRPM);
    }
    if input_state.power > Power::new(belt.configuration.max_power) {
        return Err(TransmissionError::ExceededPower);
    }

    // Calculate output values based on belt ratio and efficiency
    let input_radius = belt.pair.input_pulley.radius;
    let output_radius = belt.pair.output_pulley.radius;
    
    // Convert RPM to angular velocity for calculations
    let input_angular_velocity = input_state.rpm.to_rad_per_sec();
    
    // Calculate output angular velocity
    let output_angular_velocity = belt_calculations::calculate_output_angular_velocity(
        input_angular_velocity,
        input_radius,
        output_radius,
        belt.configuration.category,
    );
    
    // Convert back to RPM for output state
    let output_rpm = output_angular_velocity.to_rpm();
    
    // Calculate ideal output torque (without efficiency)
    let ideal_output_torque = belt_calculations::calculate_ideal_output_torque(
        input_state.torque,
        input_radius,
        output_radius,
    );
    
    // Apply efficiency to get actual output torque
    let output_torque = belt_calculations::calculate_output_torque(
        input_state.torque,
        input_radius,
        output_radius,
        belt.configuration.efficiency,
    );
    
    // Calculate output power with efficiency
    let output_power = belt_calculations::calculate_output_power(
        input_state.power,
        belt.configuration.efficiency,
    );
    
    // Check for valid physical conditions - ensure torque and RPM are not negative
    if output_torque.get() < 0.0 || output_rpm.get() < 0.0 {
        return Err(TransmissionError::InvalidConfiguration);
    }

    Ok(TransmissionState::with_values(
        output_torque,
        output_rpm,
        output_power,
        input_state.enabled,
    ))
}

/// Evaluates any transmission component given its type and input conditions.
///
/// This is the main entry point for evaluating transmission components.
///
/// # Arguments
/// * `transmission_id` - The ID of the transmission to evaluate
/// * `category` - The category of the transmission
/// * `input_state` - The input state to evaluate against
/// * `component_data` - The actual component data (as a trait object)
///
/// # Returns
/// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
/// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
pub fn evaluate_transmission(
    transmission_id: TransmissionId,
    category: TransmissionCategory,
    input_state: TransmissionState,
    component_data: &dyn IntegratedTransmission,
) -> Result<TransmissionState, TransmissionError> {
    match category {
        TransmissionCategory::Shaft => {
            // We need to downcast to get the actual shaft
            if let Some(shaft) = component_data.as_any().downcast_ref::<Shaft>() {
                evaluate_shaft(shaft, input_state)
            } else {
                Err(TransmissionError::InvalidConfiguration)
            }
        },
        TransmissionCategory::Coupling => {
            // We need to downcast to get the actual coupling
            if let Some(coupling) = component_data.as_any().downcast_ref::<Coupling>() {
                evaluate_coupling(coupling, input_state)
            } else {
                Err(TransmissionError::InvalidConfiguration)
            }
        },
        TransmissionCategory::Gear => {
            // We need to downcast to get the actual gear
            if let Some(gear) = component_data.as_any().downcast_ref::<Gear>() {
                evaluate_gear(gear, input_state)
            } else {
                Err(TransmissionError::InvalidConfiguration)
            }
        },
        TransmissionCategory::Belt => {
            // We need to downcast to get the actual belt
            if let Some(belt) = component_data.as_any().downcast_ref::<Belt>() {
                evaluate_belt(belt, input_state)
            } else {
                Err(TransmissionError::InvalidConfiguration)
            }
        },
        _ => {
            // For other categories or unknown types, we fall back to basic evaluation
            Ok(input_state)
        }
    }
}