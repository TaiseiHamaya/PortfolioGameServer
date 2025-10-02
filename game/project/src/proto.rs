#[path = "proto/types.u.pb.rs"]
#[allow(nonstandard_style, mismatched_lifetime_syntaxes)]
mod proto_types;

#[allow(unused_imports, nonstandard_style)]
pub use proto_types::*;

#[path = "proto/math.u.pb.rs"]
#[allow(nonstandard_style, mismatched_lifetime_syntaxes)]
mod proto_math;

#[allow(unused_imports, nonstandard_style)]
pub use proto_math::*;