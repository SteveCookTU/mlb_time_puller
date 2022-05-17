use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};

#[derive(TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(i8)]
pub enum Timezone {
    EDT = -4,
    PDT = -7,
    CDT = -5,
    MDT = -6,
}

impl Display for Timezone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Timezone::EDT => write!(f, "EDT"),
            Timezone::PDT => write!(f, "PDT"),
            Timezone::CDT => write!(f, "CDT"),
            Timezone::MDT => write!(f, "MDT"),
        }
    }
}
