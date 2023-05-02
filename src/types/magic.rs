use std::fmt::{Debug, Display};

#[derive(PartialEq)]
pub enum Magic {
    Fat,
    FatReverse,
    Bin32,
    Bin32Reverse,
    Bin64,
    Bin64Reverse,
}

impl Magic {
    pub fn raw_value(&self) -> u32 {
        match self {
            Magic::Fat => 0xcafebabe,
            Magic::FatReverse => 0xbebafeca,
            Magic::Bin32 => 0xfeedface,
            Magic::Bin32Reverse => 0xcefaedfe,
            Magic::Bin64 => 0xfeedfacf,
            Magic::Bin64Reverse => 0xcffaedfe,
        }
    }

    pub fn is_fat(&self) -> bool {
        match self {
            Self::Fat | Self::FatReverse => true,
            _ => false,
        }
    }

    pub fn is_reverse(&self) -> bool {
        match self {
            Magic::FatReverse => true,
            Magic::Bin32Reverse => true,
            Magic::Bin64Reverse => true,
            _ => false,
        }
    }

    pub fn is_64(&self) -> bool {
        match self {
            Self::Bin64 | Self::Bin64Reverse => true,
            _ => false,
        }
    }
}

impl TryInto<Magic> for u32 {
    type Error = crate::result::Error;

    fn try_into(self) -> std::result::Result<Magic, Self::Error> {
        match self {
            0xcafebabe => Ok(Magic::Fat),
            0xbebafeca => Ok(Magic::FatReverse),
            0xfeedface => Ok(Magic::Bin32),
            0xcefaedfe => Ok(Magic::Bin32Reverse),
            0xfeedfacf => Ok(Magic::Bin64),
            0xcffaedfe => Ok(Magic::Bin64Reverse),
            _ => Err(Self::Error::BadMagic(self)),
        }
    }
}

impl Magic {
    pub(super) fn endian(&self) -> scroll::Endian {
        if self.is_fat() || !self.is_reverse() {
            scroll::BE
        } else {
            scroll::LE
        }
    }
}

impl Clone for Magic {
    fn clone(&self) -> Self {
        match self {
            Self::Fat => Self::Fat,
            Self::FatReverse => Self::FatReverse,
            Self::Bin32 => Self::Bin32,
            Self::Bin32Reverse => Self::Bin32Reverse,
            Self::Bin64 => Self::Bin64,
            Self::Bin64Reverse => Self::Bin64Reverse,
        }
    }
}

impl Copy for Magic {}

impl Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.raw_value())
    }
}

impl Debug for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.raw_value())
    }
}