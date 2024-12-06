#[warn(dead_code)]
enum Keywords {
    Title,
    Author,
    Comment
}

pub fn process_line(_line: &str) {
    let line: String = String::from(_line);
    let splitted_line: Vec<String> = line.split(" ").map(String::from).collect();

    println!("{:?}", splitted_line);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        assert_eq(1, 1); 
    }
}
