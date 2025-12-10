/// Types is a module that allows communication between other modules with shared names
pub mod types;

/// Elevator is a module that controls elevator movement, and the building state
pub mod elevator;

/// people is a module that controls people movement, and people state,
/// along with decision making
pub mod people;

/// control is a module which handles decision making for the elevator module
pub mod control;
