use std::sync::Mutex;
//mod constant;
use crate::constants::constant::PhysicsConstantsMutable;
use crate::constants::constant::PhysicsConstants;

pub const PHYSICS: PhysicsConstants = PhysicsConstants {
    speed_of_light: 299_792_458.0,          // c
    planck_constant: 6.62607015e-34,        // h (точне визначення)
    gravity_constant: 6.67428e-11,          // G (наближено)
    absolute_zero: -273.15,                // °C (або 0K як окремий тип)
    elementary_charge: 1.602176634e-19,     // e
};

pub static PHYSICS_CONSTANTS: Mutex<PhysicsConstantsMutable> =
    Mutex::new(PhysicsConstantsMutable {
        gravity: 9.81,
        air_density: 1.225,
        drag_coefficient: 0.47,

    });