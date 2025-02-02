use zetlib::{create_zettel_note, open_file, write_to_file};

fn main() -> std::io::Result<()> {
    let template = open_file(1);
    let zettel = open_file(2);
    if zettel.is_ok() && template.is_ok() {
        let _ = write_to_file(&template.unwrap(), &zettel.unwrap());
    } else {
        create_zettel_note()?;
   }

    Ok(())
}
