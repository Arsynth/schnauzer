# libschnauzer

Schnauzer is both library and tool for parsing mach-o files

### Features

* Zero copy. Does not loads whole binary into memory. Uses iterators to list potentially large amount of items
* Endian aware
* Implements derive macro for automatic field enumeration, that, for example, very convenient for printing arbitary load commands. There even no need to write large `match` blocks for any type of load command
* Prints file structure in color for better user experience

### Installation

```shell
cargo install schnauzer
```

### Example output

Put something like in your console:

```shell
schnauzer /bin/cat
```

And you get:

![example output](https://github.com/Arsynth/schnauzer/readme_res/example_output.png)

### Documentation
docs.rs/schnauzer/0.1.7

### Usage

```toml
[dependencies]
schnauzer = "0.1.7"
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