// TODO: Check for Miri and if so, shorten the tests etc.
// Use [cfg(miri)] or cfg!(miri)

mod fixture;
mod properties;

mod one_to_one;
mod one_to_set;
mod set_to_one;
mod set_to_set;