use std::fs::File;
use std::io::prelude::*;
use std::env;

fn open_config(filename: String) -> std::io::Result<File> {
    let filename = format!("{}/.{filename}", env::var("HOME").unwrap());
    let file = File::open(filename)?;

    Ok(file)
}

fn get_string(mut file: File, target: &str) -> Result<String, ()> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let target = buffer.lines().find_map(|s| if s.contains(target) {Some(s)} else {None});

    if target.is_some() {
        return Ok(target.unwrap().to_string());
    }

    Err(())
}

// TODO! refactor this to take any arbitrary string
fn remove_until_after_equal(string: &str) -> String {
    let whitespace_end = string.char_indices().find_map(|(idx, c)| 
        if c != '=' {None} else {Some(idx + 1)})
        .unwrap_or_else(|| string.len());

    string[whitespace_end..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_string_from_file() {
        let file = open_config("foo.conf".to_string());
        assert_eq!(true, file.is_ok());

        let string = get_string(file.unwrap(), "Hello, World!");
        assert_eq!(string, Ok("Hello, World! Testing.".to_string()));
    }
    #[test]
    fn test_remove_whitespace_from_beginning() {
        let string = "Hello = World!".to_string();
        assert_eq!(" World!", remove_until_after_equal(&string));

        let string = "path = /hi/there".to_string();
        assert_eq!(" /hi/there", remove_until_after_equal(&string));
    }
}
