# Schnauzer

![](https://github.com/Arsynth/schnauzer/actions/workflows/rust.yml/badge.svg)

`schnauzer` is both library and tool for parsing mach-o files, printing symbol table, used dynamic libraries, relative paths, relocation entries, e.t.c.. [See full list of commands](#supported-commands).

[Changelog](https://github.com/Arsynth/schnauzer/blob/master/CHANGELOG.md)

## Library features

* Zero copy. Does not loads whole binary into memory. Uses iterators to list potentially large amount of items
* Endian aware
* Implements derive macro for automatic field enumeration, that, for example, very convenient for printing arbitary load commands. There even no need to write large `match` blocks for any type of load command
* Prints file structure in color for better user experience

## Installation

```shell
cargo install schnauzer
```

## Arguments

Call to `schnauzer` should have the next pattern:
```shell
schnauzer [CMD] path_to_binary
```
Where `CMD` is optional. See [Supported commands](#supported-commands) section

Specify `-h` or `--help` with or without specify subcommand to see description and supported arguments of the subcommand.

You also can specify path with `-p` or `--path` (But this not required)



## Supported commands
* [`schnauzer lc FILE [--path <FILE>] [--help] [--arch <NAME>] [--short] [--noidx]`](#lc)
* [`schnauzer syms FILE [--path <FILE>] [--help] [--arch <NAME>] [--short] [--noidx]`](#syms)
* [`schnauzer rpaths FILE [--path <FILE>] [--help] [--arch <NAME>] [--short] [--noidx]`](#rpaths)
* [`schnauzer dylibs FILE [--path <FILE>] [--help] [--arch <NAME>] [--short] [--noidx]`](#dylibs)
* [`schnauzer segs FILE [--path <FILE>] [--help] [--arch <NAME>] [--segs] [--sects] [--short] [--noidx]`](#segs)
* [`schnauzer fat FILE [--path <FILE>] [--help] [--arch <NAME>]`](#fat)
* [`schnauzer headers FILE [--path <FILE>] [--help] [--arch <NAME>] [--short] [--noidx]`](#headers)
* [`schnauzer rel FILE [--path <FILE>] [--help] [--arch <NAME>]`](#rel)
* [`schnauzer data FILE [--path -p <FILE>] [--help -h] [--arch -a <NAME>] --sect -s <segname sectname>`](#data)

### Default
```shell
# Prints almost all binary info
schnauzer path_to_binary
```
```
Fat arch:
 |*cputype: 16777223
 |*cpusubtype: 3
 |*offset: 16384
 |*size: 70080
 |*align: 14
 |*Mach header: 
   |*magic: 0xcffaedfe
   |*cputype: 16777223
   |*cpusubtype: 3
   |*filetype: Exec
   |*ncmds: 17
   |*sizeofcmds: 1544
   |*flags: 0x00200085
   |*reserved: 0x00000000
   |*Load commands: 
   [0] cmd: LC_SEGMENT_64 cmdsize: 72
     |*segname: __PAGEZERO
     |*vmaddr: 0x0000000000000000
     |*vmsize: 0x0000000100000000
     |*fileoff: 0
     |*filesize: 0
     |*maxprot: 0x00000000
     |*initprot: 0x00000000
     |*nsects: 0
     |*flags: 0x00000000
...
```

### lc
```shell
# Prints load commands
schnauzer lc path_to_binary
```
```
Arch #0 (Arch: x86_64, File type: Exec, Flags: 0x00200085):
cmd: LC_SEGMENT_64 cmdsize: 72
 segname: __PAGEZERO
 vmaddr: 0x0000000000000000
 vmsize: 0x0000000100000000
 fileoff: 0
 filesize: 0
 maxprot: 0x00000000
 initprot: 0x00000000
 nsects: 0
 flags: 0x00000000
cmd: LC_SEGMENT_64 cmdsize: 552
 segname: __TEXT
 vmaddr: 0x0000000100000000
 vmsize: 0x0000000000004000
 fileoff: 0

```

### syms
```shell
# Prints symtab
schnauzer syms path_to_binary
```
```
Arch #1 (Arch: x86_64, File type: Exec, Flags: 0x00200085):
[0] Stab(Nopt) radr://5614542
[1] External __mh_execute_header
Section: 1, n_desc: 16, Address: 0x0000000100000000
[2] External __DefaultRuneLocale
n_desc: 256, n_value: 0
[3] External ___error
n_desc: 256, n_value: 0
[4] External ___maskrune
n_desc: 256, n_value: 0
[5] External ___stack_chk_fail
n_desc: 256, n_value: 0
[6] External ___stack_chk_guard
...
```

### rpaths
```shell
# Prints relative paths
schnauzer rpaths path_to_binary
```
```
Arch #0 (Arch: arm64, File type: Exec, Flags: 0x04a18085):
[0] /usr/lib/swift
[1] @loader_path/Frameworks
[2] @loader_path/Frameworks
[3] @executable_path/Frameworks
[4] /usr/lib/swift
[5] @executable_path/Frameworks
[6] @loader_path/Frameworks
```

### dylibs
```shell
# Prints used dynamic libraries
schnauzer dylibs path_to_binary
```
```
Arch #1 (Arch: x86_64, File type: Exec, Flags: 0x00200085):
[0] /usr/lib/libSystem.B.dylib (Timestamp: 2, Current version: 1311.0.0, Compatibility version: 1.0.0)

Arch #2 (Arch: arm64e, File type: Exec, Flags: 0x00200085):
[0] /usr/lib/libSystem.B.dylib (Timestamp: 2, Current version: 1311.0.0, Compatibility version: 1.0.0)
```

### segs
```shell
# Prints all the segments with sections
schnauzer segs path_to_binary
```

```
Usage:

        schnauzer segs path_to_binary [--segs] [--sects] [--short] [--noidx]

        --segs - Print only segments
        --sects - Print only sections
        --short - Print only values and only identifying fields
        --noidx - Disable printing indices
```


```
...
[1] Segment (segname: __TEXT, vmaddr: 0x0000000100000000, vmsize: 0x0000000000004000, fileoff: 0, filesize: 16384, maxprot: 0x00000005, initprot: 0x00000005, nsects: 6, flags: 0x00000000):
  Section #1 __text Segment __TEXT:
  |*addr: 0x000000010000335c
  |*size: 0x000000000000094f
  |*offset: 13148
  |*align: 2
  |*reloff: 0
  |*nreloc: 0
  |*flags: 0x80000400
  |*reserved1: 0
  |*reserved2: 0
  |*reserved3: 0
...
```

### fat
```shell
# Prints the fat archs
schnauzer fat path_to_binary
```
```
[0] Arch: x86_64, Offset: 16384, Size: 70080, Align: 14
[1] Arch: arm64e, Offset: 98304, Size: 53488, Align: 14
```

### headers
```shell
# Prints headers
schnauzer headers path_to_binary
```
```
[0] Magic: cffaedfe, Arch: x86_64, Capabilities: 0x00, File type: Exec, Commands: 17, Size of commands: 1544, Flags: 0x00200085
Flags(detailed):
MH_NOUNDEFS
MH_DYLDLINK
MH_TWOLEVEL
MH_PIE
[1] Magic: cffaedfe, Arch: arm64e, Capabilities: 0x80, File type: Exec, Commands: 18, Size of commands: 1368, Flags: 0x00200085
Flags(detailed):
MH_NOUNDEFS
MH_DYLDLINK
MH_TWOLEVEL
MH_PIE
```

### rel
```shell
# Prints relocation entries
schnauzer rel path_to_binary
```
```
__TEXT __text (5 entries)
address  pcrel length extern type scattered symbolnum/value 
000000b0 1     2      1      2    false     10              
000000ac 0     2      1      4    false     2               
000000a8 1     2      1      3    false     2               
0000009c 0     2      1      4    false     1               
00000098 1     2      1      3    false     1               
__DATA __const (1 entries)
address  pcrel length extern type scattered symbolnum/value 
00000000 0     3      1      0    false     4               
__DWARF __debug_info (16 entries)
address  pcrel length extern type scattered symbolnum/value 
000004c2 0     3      0      0    false     1               
000004ba 0     3      0      0    false     1               
00000499 0     3      0      0    false     1               
00000491 0     3      0      0    false     1               
00000477 0     3      0      0    false     1              
```

### data
```shell
# Prints hex dump
schnauzer data path_to_binary -s __TEXT __cstring
```
```
Arch #0 (Arch: x86_64, File type: Exec, Flags: 0x00200085):
__TEXT __cstring
0000000100003f2a 0062656c 6e737475 76007374 646f7574 |.belnstuv.stdout|
0000000100003f3a 00757361 67653a20 63617420 5b2d6265 |.usage: cat [-be|
0000000100003f4a 6c6e7374 75765d20 5b66696c 65202e2e |lnstuv] [file ..|
0000000100003f5a 2e5d0a00 2d007374 64696e00 25730072 |.]..-.stdin.%s.r|
```

### Usage

```toml
[dependencies]
schnauzer = "0.3.2"
```

### Examples

Simple debug print

```rust
use schnauzer::ObjectType;
use schnauzer::Parser;
use std::path::Path;

fn main() {
    let mut args = std::env::args();
    let _exec_name = args.next();

    let path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Not enough arguments. Provide a valid path to binary");
            std::process::exit(1);
        }
    };
    let path = Path::new(&path);

    let parser = match Parser::build(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Could not create parser at '{:?}': {e}", path);
            std::process::exit(1);
        }
    };

    let object = match parser.parse() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error while parsing: {:#?}", e);
            std::process::exit(1);
        }
    };

    handle_object(object);
}

fn handle_object(obj: ObjectType) {
    println!("***Object***");
    println!("{:#?}", obj);
}
```

Using `AutoEnumFields` derive (code taken from `src/main.rs`)

```rust
let h = macho.header();
for field in h.all_fields() {
    out_dashed_field(field.name, field.value, level);
}
```

# Contacts

You may email me: 
[arsynthdev@gmail.com](mailto:arsynthdev@gmail.com)

[GitHub profile](https://github.com/Arsynth)