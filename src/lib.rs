//! Real Time For the Masses (RTFM), a framework for building concurrent
//! applications, for MSP430 microcontrollers
//!
//! # Examples
//!
//! In increasing grade of complexity. See the [examples](./examples/index.html)
//! module.
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate msp430;
extern crate msp430_rtfm_macros;
extern crate rtfm_core;
extern crate static_ref;

pub mod examples;

use core::u8;

pub use rtfm_core::{Resource, Static, Threshold};
#[doc(hidden)]
pub use msp430::interrupt::enable;
pub use msp430_rtfm_macros::app;
use msp430::interrupt;

/// Executes the closure `f` in a preemption free context
///
/// During the execution of the closure no task can preempt the current task.
#[inline]
pub fn atomic<R, F>(t: &mut Threshold, f: F) -> R
where
    F: FnOnce(&mut Threshold) -> R,
{
    if t.value() == u8::MAX {
        f(t)
    } else {
        interrupt::disable();
        let r = f(&mut unsafe { Threshold::max() });
        unsafe { interrupt::enable() };
        r
    }
}

#[inline]
#[doc(hidden)]
pub unsafe fn claim<T, R, F>(data: T, t: &mut Threshold, f: F) -> R
where
    F: FnOnce(T, &mut Threshold) -> R,
{
    if t.value() > 0 {
        f(data, t)
    } else {
        atomic(t, |t| f(data, t))
    }
}
