use std::fmt::Debug;
use std::fmt::Display;

use scroll::{IOread, SizeWith};

use super::Hu32;

pub mod object_flags_constants {
    /// The object file has no undefined references
    pub const MH_NOUNDEFS: u32 = 0x1;
    /// the object file is the output of an incremental link against a base file and can't be link edited again  
    pub const MH_INCRLINK: u32 = 0x2;
    /// the object file is input for the dynamic linker and can't be staticly link edited again  
    pub const MH_DYLDLINK: u32 = 0x4;
    /// the object file's undefined references are bound by the dynamic linker when loaded.  
    pub const MH_BINDATLOAD: u32 = 0x8;
    /// the file has its dynamic undefined references prebound.   
    pub const MH_PREBOUND: u32 = 0x10;
    /// the file has its read-only and read-write segments split  
    pub const MH_SPLIT_SEGS: u32 = 0x20;
    /// the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)   
    pub const MH_LAZY_INIT: u32 = 0x40;
    /// the image is using two-level name space bindings   
    pub const MH_TWOLEVEL: u32 = 0x80;
    /// the executable is forcing all images to use flat name space bindings  
    pub const MH_FORCE_FLAT: u32 = 0x100;
    /// this umbrella guarantees no multiple defintions of symbols in its sub-images so the two-level 
    /// namespace hints can always be used.  
    pub const MH_NOMULTIDEFS: u32 = 0x200;
    /// do not have dyld notify the prebinding agent about this executable  
    pub const MH_NOFIXPREBINDING: u32 = 0x400;
    /// the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.  
    pub const MH_PREBINDABLE: u32 = 0x800;
    /// indicates that this binary binds to all two-level namespace modules of its dependent libraries. 
    /// Only used when [MH_PREBINDABLE] and [MH_TWOLEVEL] are both set.  
    pub const MH_ALLMODSBOUND: u32 = 0x1000;
    /// safe to divide up the sections into sub-sections via symbols for dead code stripping  
    pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 0x2000;
    /// the binary has been canonicalized via the unprebind operation  
    pub const MH_CANONICAL: u32 = 0x4000;
    /// the final linked image contains external weak symbols  
    pub const MH_WEAK_DEFINES: u32 = 0x8000;
    /// the final linked image uses weak symbols  
    pub const MH_BINDS_TO_WEAK: u32 = 0x10000;
    /// When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.  
    pub const MH_ALLOW_STACK_EXECUTION: u32 = 0x20000;
    /// When this bit is set, the binary declares it is safe for use in processes with uid zero  
    pub const MH_ROOT_SAFE: u32 = 0x40000;
    /// When this bit is set, the binary declares it is safe for use in processes when issetugid() is true  
    pub const MH_SETUID_SAFE: u32 = 0x80000;
    /// When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported  
    pub const MH_NO_REEXPORTED_DYLIBS: u32 = 0x100000;
    /// When this bit is set, the OS will load the main executable at a random address. Only used in [MH_EXECUTE] filetypes.  
    pub const MH_PIE: u32 = 0x200000;
    /// Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will 
    /// automatically not create a [LC_LOAD_DYLIB] load command to the dylib if no symbols are being referenced from the dylib.  
    pub const MH_DEAD_STRIPPABLE_DYLIB: u32 = 0x400000;
    /// Contains a section of type [S_THREAD_LOCAL_VARIABLES]  
    pub const MH_HAS_TLV_DESCRIPTORS: u32 = 0x800000;
    /// When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that 
    /// don't require it. Only used in [MH_EXECUTE] filetypes.  
    pub const MH_NO_HEAP_EXECUTION: u32 = 0x1000000;
    /// The code was linked for use in an application extension.  
    pub const MH_APP_EXTENSION_SAFE: u32 = 0x02000000;
}


#[derive(IOread, SizeWith)]
pub struct ObjectFlags(pub u32);

impl Debug for ObjectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Hu32(self.0))
    }
}

impl Display for ObjectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Hu32(self.0))
    }
}

use self::object_flags_constants::*;

impl ObjectFlags {
    /// See [`MH_NOUNDEFS`]
    pub fn is_no_undefs(&self) -> bool {
        self.0 & MH_NOUNDEFS > 0
    }

    /// See [`MH_INCRLINK`]
    pub fn is_incremental_link(&self) -> bool {
        self.0 & MH_INCRLINK > 0
    }

    /// See [`MH_DYLDLINK`]
    pub fn is_dyld_input(&self) -> bool {
        self.0 & MH_DYLDLINK > 0
    }

    /// See [`MH_BINDATLOAD`]
    pub fn is_undefs_bound_by_dyld(&self) -> bool {
        self.0 & MH_BINDATLOAD > 0
    }

    /// See [`MH_PREBOUND`]
    pub fn is_prebound_undefs(&self) -> bool {
        self.0 & MH_PREBOUND > 0
    }
    
    /// See [`MH_SPLIT_SEGS`]
    pub fn is_ro_rw_segs_splitted(&self) -> bool {
        self.0 & MH_SPLIT_SEGS > 0
    }

    /// Obsolete
    /// See [`MH_LAZY_INIT`]
    pub fn is_lazy_init(&self) -> bool {
        self.0 & MH_LAZY_INIT > 0
    }

    /// See [`MH_TWOLEVEL`]
    pub fn is_two_level_name_space(&self) -> bool {
        self.0 & MH_TWOLEVEL > 0
    }

    /// See [`MH_FORCE_FLAT`]
    pub fn is_force_flat(&self) -> bool {
        self.0 & MH_FORCE_FLAT > 0
    }

    /// See [`MH_NOMULTIDEFS`]
    pub fn is_no_multiple_defs(&self) -> bool {
        self.0 & MH_NOMULTIDEFS > 0
    }
    
    /// See [`MH_NOFIXPREBINDING`]
    pub fn is_no_fix_prebinding(&self) -> bool {
        self.0 & MH_NOFIXPREBINDING > 0
    }

    /// See [`MH_PREBINDABLE`]
    pub fn is_prebindable(&self) -> bool {
        self.0 & MH_PREBINDABLE > 0
    }

    /// See [`MH_ALLMODSBOUND`]
    pub fn is_all_modules_bound(&self) -> bool {
        self.0 & MH_ALLMODSBOUND > 0
    }

    /// See [`MH_SUBSECTIONS_VIA_SYMBOLS`]
    pub fn is_subsections_via_symbols(&self) -> bool {
        self.0 & MH_SUBSECTIONS_VIA_SYMBOLS > 0
    }

    /// See [`MH_CANONICAL`]
    pub fn is_canonical(&self) -> bool {
        self.0 & MH_CANONICAL > 0
    }

    /// See [`MH_WEAK_DEFINES`]
    pub fn is_weak_defines(&self) -> bool {
        self.0 & MH_WEAK_DEFINES > 0
    }

    /// See [`MH_BINDS_TO_WEAK`]
    pub fn is_bind_to_weak(&self) -> bool {
        self.0 & MH_BINDS_TO_WEAK > 0
    }

    /// See [`MH_ALLOW_STACK_EXECUTION`]
    pub fn is_allow_stack_execution(&self) -> bool {
        self.0 & MH_ALLOW_STACK_EXECUTION > 0
    }

    /// See [`MH_ROOT_SAFE`]
    pub fn is_root_safe(&self) -> bool {
        self.0 & MH_ROOT_SAFE > 0
    }

    /// See [`MH_SETUID_SAFE`]
    pub fn is_setuid_safe(&self) -> bool {
        self.0 & MH_SETUID_SAFE > 0
    }

    /// See [`MH_NO_REEXPORTED_DYLIBS`]
    pub fn is_no_reexported_dylibs(&self) -> bool {
        self.0 & MH_NO_REEXPORTED_DYLIBS > 0
    }

    /// See [`MH_PIE`]
    pub fn is_pie(&self) -> bool {
        self.0 & MH_PIE > 0
    }

    /// See [`MH_DEAD_STRIPPABLE_DYLIB`]
    pub fn is_dead_strippable_dylib(&self) -> bool {
        self.0 & MH_DEAD_STRIPPABLE_DYLIB > 0
    }

    /// See [`MH_HAS_TLV_DESCRIPTORS`]
    pub fn is_has_tlv_descriptors(&self) -> bool {
        self.0 & MH_HAS_TLV_DESCRIPTORS > 0
    }

    /// See [`MH_NO_HEAP_EXECUTION`]
    pub fn is_no_heap_execution(&self) -> bool {
        self.0 & MH_NO_HEAP_EXECUTION > 0
    }

    /// See [`MH_APP_EXTENSION_SAFE`]
    pub fn is_app_extension_safe(&self) -> bool {
        self.0 & MH_APP_EXTENSION_SAFE > 0
    }
}