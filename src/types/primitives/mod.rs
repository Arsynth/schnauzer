use crate::fmt_ext::printable_string;
use crate::fmt_ext::printable_uuid_string;
use scroll::ctx::FromCtx;
use scroll::ctx::SizeWith;
use scroll::Endian;
use scroll::{IOread, SizeWith};
use std::fmt::{Debug, Display, LowerHex};

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
    ($t:ident, $main:ty, 0) => {
        // pub struct $t(pub $main);

        impl FromCtx<X64Context> for $t {
            fn from_ctx(this: &[u8], ctx: X64Context) -> Self {
                match ctx {
                    X64Context::On(e) => match e {
                        Endian::Little => $t(<$main>::from_le_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                        Endian::Big => $t(<$main>::from_be_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                    },
                    X64Context::Off(_) => $t(<$main>::default()),
                }
            }
        }

        impl SizeWith<X64Context> for $t {
            fn size_with(ctx: &X64Context) -> usize {
                match ctx {
                    X64Context::On(_) => std::mem::size_of::<$main>(),
                    X64Context::Off(_) => 0,
                }
            }
        }
    };
    ($t:ident, $main:ty, $alt:ty) => {
        // pub struct $t(pub $main);

        impl FromCtx<X64Context> for $t {
            fn from_ctx(this: &[u8], ctx: X64Context) -> Self {
                match ctx {
                    X64Context::On(e) => match e {
                        Endian::Little => $t(<$main>::from_le_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                        Endian::Big => $t(<$main>::from_be_bytes(
                            this[..std::mem::size_of::<$main>()].try_into().unwrap(),
                        )),
                    },
                    X64Context::Off(e) => match e {
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

        impl SizeWith<X64Context> for $t {
            fn size_with(ctx: &X64Context) -> usize {
                match ctx {
                    X64Context::On(_) => std::mem::size_of::<$main>(),
                    X64Context::Off(_) => std::mem::size_of::<$alt>(),
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
hex_display!(Hi32, 10);

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu64(pub u64);
from_ctx_64_tuple_struct!(Hu64, u64, u32);
hex_display!(Hu64, 18);

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

#[derive(Clone, Debug)]
pub enum X64Context {
    On(Endian),
    Off(Endian),
}

impl Copy for X64Context {}

impl X64Context {
    pub fn endian(&self) -> &Endian {
        match self {
            X64Context::On(e) => e,
            X64Context::Off(e) => e,
        }
    }

    pub fn is_64(&self) -> bool {
        match self {
            X64Context::On(_) => true,
            X64Context::Off(_) => false,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct u64_io(pub u64);
from_ctx_64_tuple_struct!(u64_io, u64, u32);
num_display!(u64_io);

#[allow(non_camel_case_types)]
pub struct u32opt(pub u32);
from_ctx_64_tuple_struct!(u32opt, u32, 0);
num_display!(u32opt);

#[cfg(test)]
mod test {
    use super::*;
    use scroll::IOread;
    use std::io::{Cursor};

    #[test]
    fn u64_ctx_u32opt_test() {
        let bytes: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7];

        let ctx = X64Context::On(Endian::Big);
        assert_eq!(
            u32opt::size_with(&ctx),
            4,
            "Unexpected type size with context: {:?}",
            ctx
        );

        let mut cur = Cursor::new(bytes);
        let opt_32: u32opt = cur.ioread_with(ctx).unwrap();
        assert_eq!(
            cur.position(),
            4,
            "Invalid position after read. Context: {:?}",
            ctx
        );
        cur.set_position(0);
        let just_32: u32 = cur.ioread_with(ctx.endian().clone()).unwrap();
        assert_eq!(opt_32.0, just_32, "Context: {:?}", ctx);

        let ctx = X64Context::Off(Endian::Big);
        assert_eq!(
            u32opt::size_with(&ctx),
            0,
            "Unexpected type size with context: {:?}",
            ctx
        );
        let mut cur = Cursor::new(bytes);
        let opt_32: u32opt = cur.ioread_with(ctx).unwrap();
        assert_eq!(cur.position(), 0, "Invalid position after read.\n Note: Values that optional to read should not read and affect stream.\n Context: {:?}", ctx);
        assert_eq!(opt_32.0, 0, "Context: {:?}", ctx);
    }
    
    #[test]
    fn u64_ctx_u64_io_test() {
        u64_ctx_u64_io_endian(Endian::Big);
        u64_ctx_u64_io_endian(Endian::Little);
    }

    fn u64_ctx_u64_io_endian(endian: scroll::Endian) {
        let bytes: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7];

        let ctx = X64Context::On(endian);
        assert_eq!(
            u64_io::size_with(&ctx),
            8,
            "Unexpected type size with context: {:?}",
            ctx
        );

        let mut cur = Cursor::new(bytes);
        let u64io: u64_io = cur.ioread_with(ctx).unwrap();
        assert_eq!(
            cur.position(),
            8,
            "Invalid position after read. Context: {:?}",
            ctx
        );
        cur.set_position(0);
        let just_64: u64 = cur.ioread_with(ctx.endian().clone()).unwrap();
        assert_eq!(u64io.0, just_64, "Context: {:?}", ctx);

        let ctx = X64Context::Off(endian);
        assert_eq!(
            u32opt::size_with(&ctx),
            0,
            "Unexpected type size with context: {:?}",
            ctx
        );
        let mut cur = Cursor::new(bytes);
        let u64io: u64_io = cur.ioread_with(ctx).unwrap();
        assert_eq!(cur.position(), 4, "Invalid position after read.\n Note: u64_io should read as u32 with context Off.\n Context: {:?}", ctx);
        cur.set_position(0);
        let just_32: u32 = cur.ioread_with(ctx.endian().clone()).unwrap();
        assert_eq!(u64io.0, just_32 as u64, "Context: {:?}", ctx);
    }
}
