use crate::fmt_ext::printable_string;
use crate::fmt_ext::printable_uuid_string;
use scroll::ctx::FromCtx;
use scroll::ctx::SizeWith;
use scroll::Endian;
use scroll::{IOread, SizeWith};
use std::fmt::{Debug, Display, LowerHex};

use super::auto_enum_fields::*;

pub mod filetype;
pub use filetype::*;

pub mod object_flags;
pub use object_flags::*;

pub mod machine;
pub use machine::*;

/// Represents vm_prot_t
pub type VmProt = Hi32;

pub type LoadCommandType = u32;

macro_rules! from_ctx_64_tuple_struct {
    ($t:ident, $main:ty, $alt:ty) => {
        // pub struct $t(pub $main);

        impl FromCtx<U64Context> for $t {
            fn from_ctx(this: &[u8], ctx: U64Context) -> Self {
                match ctx {
                    U64Context::Whole(e) => match e {
                        Endian::Little => $t(<$main>::from_le_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                        Endian::Big => $t(<$main>::from_be_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                    },
                    U64Context::Low32(e) => match e {
                        Endian::Little => $t(<$alt>::from_le_bytes(
                            this[..std::mem::size_of::<$alt>()].try_into().unwrap(),
                        ) as $main),
                        Endian::Big => $t(<$alt>::from_be_bytes(
                            this[..std::mem::size_of::<$alt>()].try_into().unwrap(),
                        ) as $main),
                    },
                }
            }
        }

        impl SizeWith<U64Context> for $t {
            fn size_with(ctx: &U64Context) -> usize {
                match ctx {
                    U64Context::Whole(_) => std::mem::size_of::<$main>(),
                    U64Context::Low32(_) => std::mem::size_of::<$alt>(),
                }
            }
        }
    };
    ($t:ident, $main:ty, 0) => {
        // pub struct $t(pub $main);

        impl FromCtx<U64Context> for $t {
            fn from_ctx(this: &[u8], ctx: U64Context) -> Self {
                match ctx {
                    U64Context::Whole(e) => match e {
                        Endian::Little => $t(<$main>::from_le_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                        Endian::Big => $t(<$main>::from_be_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                    },
                    U64Context::Low32(e) => $t(<$main>::default()),
                }
            }
        }

        impl SizeWith<U64Context> for $t {
            fn size_with(ctx: &U64Context) -> usize {
                match ctx {
                    U64Context::Whole(_) => std::mem::size_of::<$main>(),
                    U64Context::Low32(_) => 0,
                }
            }
        }
    };
}

macro_rules! num_display {
    ($t:ident) => {
        impl Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! hex_display {
    ($t:ident, $width:expr) => {
        impl Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#0width$x}", self.0, width = $width)
            }
        }
        
        impl LowerHex for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#0width$x}", self.0, width = $width)
            }
        }
        
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#0width$x}", self.0, width = $width)
            }
        }
    };
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu32(pub u32);
hex_display!(Hu32, 10);

// impl Debug for Hu32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#010x}", self.0)
//     }
// }

// impl LowerHex for Hu32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#010x}", self.0)
//     }
// }

// impl Display for Hu32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#010x}", self.0)
//     }
// }

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu32w4(pub u32);

impl Debug for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl LowerHex for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hi32(pub i32);

impl Debug for Hi32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl LowerHex for Hi32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Hi32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu64(pub u64);
from_ctx_64_tuple_struct!(Hu64, u64, u32);
hex_display!(Hu64, 18);

// impl Debug for Hu64 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#018x}", self.0)
//     }
// }

// impl LowerHex for Hu64 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#018x}", self.0)
//     }
// }

// impl Display for Hu64 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#018x}", self.0)
//     }
// }

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Uuid(pub [u8; 16]);

impl Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_uuid_string(&self.0))
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Segname(pub [u8; 16]);

impl Debug for Segname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Version32(pub u32);

impl Version32 {
    pub fn x(&self) -> u32 {
        const MASK: u32 = 0xFFFF0000;
        (self.0 & MASK) >> 16
    }

    pub fn y(&self) -> u32 {
        const MASK: u32 = 0x0000FF00;
        (self.0 & MASK) >> 8
    }

    pub fn z(&self) -> u32 {
        const MASK: u32 = 0x000000FF;
        self.0 & MASK
    }
}

impl Debug for Version32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.x(), self.y(), self.z())
    }
}

impl Display for Version32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.x(), self.y(), self.z())
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Version64(pub u64);

impl Version64 {
    pub fn a(&self) -> u64 {
        const MASK: u64 = 0xFFFFFF0000000000;
        (self.0 & MASK) >> 40
    }
    pub fn b(&self) -> u64 {
        const MASK: u64 = 0xFFC0000000;
        (self.0 & MASK) >> 30
    }
    pub fn c(&self) -> u64 {
        const MASK: u64 = 0x3FF00000;
        (self.0 & MASK) >> 20
    }
    pub fn d(&self) -> u64 {
        const MASK: u64 = 0xFFC00;
        (self.0 & MASK) >> 10
    }
    pub fn e(&self) -> u64 {
        const MASK: u64 = 0x3FF;
        self.0 & MASK
    }
}

impl Debug for Version64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.a(),
            self.b(),
            self.c(),
            self.d(),
            self.e()
        )
    }
}

impl Display for Version64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.a(),
            self.b(),
            self.c(),
            self.d(),
            self.e()
        )
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Str16Bytes(pub [u8; 16]);

impl Debug for Str16Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}

impl Display for Str16Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}

#[derive(Clone)]
pub enum U64Context {
    Whole(Endian),
    Low32(Endian),
}

impl Copy for U64Context {}

impl U64Context {
    pub fn endian(&self) -> &Endian {
        match self {
            U64Context::Whole(e) => e,
            U64Context::Low32(e) => e,
        }
    }

    pub fn is_64(&self) -> bool {
        match self {
            U64Context::Whole(_) => true,
            U64Context::Low32(_) => false,
        }
    }
}

pub struct u64_io(pub u64);
from_ctx_64_tuple_struct!(u64_io, u64, u32);
num_display!(u64_io);

pub struct u32opt(pub u32);
from_ctx_64_tuple_struct!(u32opt, u32, 0);
num_display!(u32opt);

// impl FromCtx<U64Context> for u64_io {
//     fn from_ctx(this: &[u8], ctx: U64Context) -> Self {
//         match ctx {
//             U64Context::Whole(e) => match e {
//                 Endian::Little => u64_io(u64::from_le_bytes(
//                     this[..std::mem::size_of::<u64>()].try_into().unwrap(),
//                 )),
//                 Endian::Big => u64_io(u64::from_be_bytes(
//                     this[..std::mem::size_of::<u64>()].try_into().unwrap(),
//                 )),
//             },
//             U64Context::Low32(e) => match e {
//                 Endian::Little => u64_io(u32::from_le_bytes(
//                     this[..std::mem::size_of::<u32>()].try_into().unwrap(),
//                 ) as u64),
//                 Endian::Big => u64_io(u32::from_be_bytes(
//                     this[..std::mem::size_of::<u32>()].try_into().unwrap(),
//                 ) as u64),
//             },
//         }
//     }
// }

// impl SizeWith<U64Context> for u64_io {
//     fn size_with(ctx: &U64Context) -> usize {
//         match ctx {
//             U64Context::Whole(_) => std::mem::size_of::<u64>(),
//             U64Context::Low32(_) => std::mem::size_of::<u32>(),
//         }
//     }
// }
