// no code in this module should have any `unsafe` calls.
// Anything needing an unsafe call should be abstracted using either `hal_call!` or something else in the `hal` module.

mod digital_out;
pub use self::digital_out::*;

mod robot_base;
pub use self::robot_base::*;