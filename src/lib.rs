use std::fs::File;
use std::io::prelude::*;
use std::env;
use chrono::Local;
use std::path::PathBuf;


// TODO! add custom error to return
fn open_config(filename: String) -> std::io::Result<File> {
    // TODO! change env::var to something better
    let filename = format!("{}/.{filename}", env::var("HOME").unwrap());
    let file = File::open(filename)?;

    Ok(file)
}

fn get_string(mut file: File, target: &str) -> Result<String, ()> {
    let mut buffer = String::new();
    // TODO! cannot read file from new_zettel_note()
    file.read_to_string(&mut buffer).unwrap();

    let target = buffer.lines().find_map(|s| if s.contains(target) {Some(s)} else {None});

    if target.is_some() {
        return Ok(target.unwrap().to_string());
    }

    //println!("{:?}", buffer);
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

fn open_file() -> Result<File, ()> {
    let file_name: Vec<String> = env::args().collect();
    println!("{:?}", env::args());
    let file = File::open(&file_name[1]);
    if file.is_ok() {
        return Ok(file.unwrap());
    } else {
        // TODO! create custom error to return
        return Err(());
    }
}

// TODO! create custom error
fn create_zettel_note() -> Result<File, std::io::Error> {
    let config = open_config("foo.conf".to_string());
    let filename_format = get_string(config.unwrap(), "format").unwrap();

    // TODO! make it so remove_inclusive can take a &str insetead of a char
    let filename_format = remove_inclusive(&filename_format, '=').unwrap();
    let filename_format = remove_inclusive(&filename_format, ' ').unwrap();
    let filename_format = Local::now().format(&filename_format).to_string();
    
    let args: Vec<String> = env::args().collect();
    let mut filename = filename_format.clone();
    if args.len() > 1 {
        for i in 1..args.len() {
            let arg = {
            let mut temp = args[i].clone();
            temp.insert_str(0, "-");
            temp
            };
            filename += &arg;
        }
    }

    let filepath = PathBuf::from(env::var("HOME").expect("I'm changing this soon")
        + "/Zettelkasten/" + &filename + ".md");

    let mut file = File::create_new(&filepath)?;

    filename = filename.replace("-", " ");

    file.write_all(filename.as_bytes())?;

    Ok(file)
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
    #[test]
    fn test_open_file() {
        let file = open_file();

        assert_eq!(true, file.is_ok());
    }
    #[test]
    fn test_create_zettel_note() {
        let file = create_zettel_note();
        assert_eq!(true, file.is_ok());

        dbg!(&file);
        let string = get_string(file.unwrap(), "0").unwrap();
        //assert_eq!("format = %Y%m%d%H%M", &string);
    }
    /*#[test]
    fn test_write_to_file() {
        let file = open_file();
        let file = write_to_file(file);

        assert_eq!(true, file.is_ok());

        let string = get_string(file.unwrap(), "testing");
        assert_eq!(string, Ok("I am testing".to_string());
    }*/
}
