// TODO: Check for Miri and if so, shorten the tests etc.
// Use [cfg(miri)] or cfg!(miri)

mod common_fixture;
mod iterbank;

mod mappy_fixture;
mod mappy_properties;

// each pom test has its own fixture and properties

mod setty_fixture;
mod setty_properties;

mod raw_junctions;
mod raw_structures;

mod shared_junctions;
mod shared_structures;

mod raw_poms;
mod floating_poms;
mod shared_poms;