use chrono::Local;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
/*  let format = Local::now().format("%Y%m%d%H%M").to_string();
    let args: Vec<String> = env::args().collect();
    let mut filename = format.clone();
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

    let mut file = File::create(&filepath)?;

    filename = filename.replace("-", " ");

    file.write_all(filename.as_bytes())?;

    std::process::Command::new("vim")
        .arg(&filepath.into_os_string().into_string().unwrap())
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");*/

    println!("Hello, World!");

    Ok(())
}
