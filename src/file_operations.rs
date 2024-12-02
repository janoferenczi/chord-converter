use std::fs;

pub fn read_file(_path: &str) -> std::io::Result<()> {
    println!("Reading file from path {_path}");
    let file_content = fs::read_to_string(_path)?;
    let lines:Vec<&str> = file_content.split("\n").collect();

    println!("The linse are {:?}", lines);

    Ok(())
}
