#![no_std]
#![feature(associated_type_bounds)]

pub mod display;

/// Should be implemented by all subsystems of the code.
pub trait Component {
    /// Called regularly from the main loop. Shouldn't block.
    /// us: Number of microseconds since device startup.
    fn tick(&mut self, us: u64);
}