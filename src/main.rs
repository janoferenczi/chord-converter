use std::env;

mod file_operations;
mod help;


fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No input was providede!");
        help::print_usage();

        return Ok(());
    }

    let path: &str = args[1].as_str();
    file_operations::read_file(path);

    Ok(())
}
