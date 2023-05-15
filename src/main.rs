use schnauzer::commands;

fn main() {
    match commands::handle_with_args() {
        Ok(_) => (),
        Err(err) => eprintln!("{:#?}", err),
    }
}
