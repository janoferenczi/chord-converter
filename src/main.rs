mod file_operations;
mod lexer;
mod help;

fn main() -> std::io::Result<()> {
    let input_path = file_operations::extract_input_path();
    let content_lines = match file_operations::read_file(input_path.as_str()) {
        Some(cl) => cl,
        None => panic!("Some panic")
    };
    
    lexer::process_line(content_lines[0].as_str());

    
    Ok(())
}
