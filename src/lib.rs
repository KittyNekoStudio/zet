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

fn remove_inclusive(string: &str, target: char) -> Result<String, String> {
    let char_end = string.char_indices().find_map(|(idx, c)| 
        if c != target {None} else {Some(idx + 1)});

    if char_end.is_some() {
        return Ok(string[char_end.unwrap()..].to_string());
    } else {
        return Err(string.to_string());
    }
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
    fn remove_arbitrary_char_from_string_ok() {
        let string = "Hello = World!";
        assert_eq!(" World!", remove_inclusive(string, '=').unwrap());

        let string = " World!";
        assert_eq!("World!", remove_inclusive(string, ' ').unwrap());
     }
    #[test]
    fn remove_arbitrary_char_from_string_err() {
        let string = "Hello = World!";
        assert_eq!(true, remove_inclusive(string, '-').is_err());
    }
}
