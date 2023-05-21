use crate::primitives::Hu32;

use super::load_command::*;
use uuid::Uuid;

pub fn zero_terminated_str(from: &[u8]) -> core::result::Result<&str, core::str::Utf8Error> {
    let mut nul_range_end = 0_usize;
    for b in from {
        if *b == 0 {
            break;
        }
        nul_range_end += 1;
    }
    return std::str::from_utf8(&from[0..nul_range_end]);
}

pub fn printable_string(from: &[u8]) -> String {
    let s = zero_terminated_str(from);
    if let Ok(s) = s {
        format!("{}", s)
    } else {
        format!("{:?}", from)
    }
}

pub fn printable_uuid_string(from: &[u8; 16]) -> String {
    let uuid = Uuid::from_slice(from);
    if let Ok(uuid) = uuid {
        uuid.hyphenated().encode_upper(&mut Uuid::encode_buffer()).to_string()
    } else {
        printable_string(from)
    }
}

pub fn load_command_to_string(cmd: u32) -> String {
    match cmd {
        LC_SEGMENT => "LC_SEGMENT".to_string(),
        LC_SYMTAB => "LC_SYMTAB".to_string(),
        LC_SYMSEG => "LC_SYMSEG".to_string(),
        LC_THREAD => "LC_THREAD".to_string(),
        LC_UNIXTHREAD => "LC_UNIXTHREAD".to_string(),
        LC_LOADFVMLIB => "LC_LOADFVMLIB".to_string(),
        LC_IDFVMLIB => "LC_IDFVMLIB".to_string(),
        LC_IDENT => "LC_IDENT".to_string(),
        LC_FVMFILE => "LC_FVMFILE".to_string(),
        LC_PREPAGE => "LC_PREPAGE".to_string(),
        LC_DYSYMTAB => "LC_DYSYMTAB".to_string(),
        LC_LOAD_DYLIB => "LC_LOAD_DYLIB".to_string(),
        LC_ID_DYLIB => "LC_ID_DYLIB".to_string(),
        LC_LOAD_DYLINKER => "LC_LOAD_DYLINKER".to_string(),
        LC_ID_DYLINKER => "LC_ID_DYLINKER".to_string(),
        LC_PREBOUND_DYLIB => "LC_PREBOUND_DYLIB".to_string(),
        LC_ROUTINES => "LC_ROUTINES".to_string(),
        LC_SUB_FRAMEWORK => "LC_SUB_FRAMEWORK".to_string(),
        LC_SUB_UMBRELLA => "LC_SUB_UMBRELLA".to_string(),
        LC_SUB_CLIENT => "LC_SUB_CLIENT".to_string(),
        LC_SUB_LIBRARY => "LC_SUB_LIBRARY".to_string(),
        LC_TWOLEVEL_HINTS => "LC_TWOLEVEL_HINTS".to_string(),
        LC_PREBIND_CKSUM => "LC_PREBIND_CKSUM".to_string(),
        LC_LOAD_WEAK_DYLIB => "LC_LOAD_WEAK_DYLIB".to_string(),
        LC_SEGMENT_64 => "LC_SEGMENT_64".to_string(),
        LC_ROUTINES_64 => "LC_ROUTINES_64".to_string(),
        LC_UUID => "LC_UUID".to_string(),
        LC_RPATH => "LC_RPATH".to_string(),
        LC_CODE_SIGNATURE => "LC_CODE_SIGNATURE".to_string(),
        LC_SEGMENT_SPLIT_INFO => "LC_SEGMENT_SPLIT_INFO".to_string(),
        LC_REEXPORT_DYLIB => "LC_REEXPORT_DYLIB".to_string(),
        LC_LAZY_LOAD_DYLIB => "LC_LAZY_LOAD_DYLIB".to_string(),
        LC_ENCRYPTION_INFO => "LC_ENCRYPTION_INFO".to_string(),
        LC_DYLD_INFO => "LC_DYLD_INFO".to_string(),
        LC_DYLD_INFO_ONLY => "LC_DYLD_INFO_ONLY".to_string(),
        LC_LOAD_UPWARD_DYLIB => "LC_LOAD_UPWARD_DYLIB".to_string(),
        LC_VERSION_MIN_MACOSX => "LC_VERSION_MIN_MACOSX".to_string(),
        LC_VERSION_MIN_IPHONEOS => "LC_VERSION_MIN_IPHONEOS".to_string(),
        LC_FUNCTION_STARTS => "LC_FUNCTION_STARTS".to_string(),
        LC_DYLD_ENVIRONMENT => "LC_DYLD_ENVIRONMENT".to_string(),
        LC_MAIN => "LC_MAIN".to_string(),
        LC_DATA_IN_CODE => "LC_DATA_IN_CODE".to_string(),
        LC_SOURCE_VERSION => "LC_SOURCE_VERSION".to_string(),
        LC_DYLIB_CODE_SIGN_DRS => "LC_DYLIB_CODE_SIGN_DRS".to_string(),
        LC_ENCRYPTION_INFO_64 => "LC_ENCRYPTION_INFO_64".to_string(),
        LC_LINKER_OPTION => "LC_LINKER_OPTION".to_string(),
        LC_LINKER_OPTIMIZATION_HINT => "LC_LINKER_OPTIMIZATION_HINT".to_string(),
        LC_VERSION_MIN_TVOS => "LC_VERSION_MIN_TVOS".to_string(),
        LC_VERSION_MIN_WATCHOS => "LC_VERSION_MIN_WATCHOS".to_string(),
        LC_NOTE => "LC_NOTE".to_string(),
        LC_BUILD_VERSION => "LC_BUILD_VERSION".to_string(),
        _ => format!("{:#x}", Hu32(cmd)),
    }
}
