use std::{error::Error, fs};

pub fn log(s: impl ToString) -> Result<(), Box<dyn Error>> {
    let user = crate::username::username()?;
    if !fs::exists(format!("/home/{user}/.cache/anthilus/anthilus.log")).unwrap_or(false) {
        println!("creating dir");
        fs::create_dir_all(format!("/home/{user}/.cache/anthilus/")).unwrap();
        fs::write(
            format!("/home/{user}/.cache/anthilus/anthilus.log"),
            "created ~/.cache/anthilus and ~/.cache/anthilus.log",
        )
        .unwrap();
    }

    let mut text =
        String::from_utf8(fs::read(format!("/home/{user}/.cache/anthilus/anthilus.log")).unwrap())
            .unwrap();
    text.push('\n');
    text.push_str(s.to_string().as_str());
    fs::write(format!("/home/{user}/.cache/anthilus/anthilus.log"), text).unwrap();

    Ok(())
}

pub fn print_logs() -> Result<(), Box<dyn Error>> {
    log("printing logs")?;
    let user = crate::username::username()?;
    println!(
        "{}",
        String::from_utf8(fs::read(format!(
            "/home/{user}/.cache/anthilus/anthilus.log"
        ))?)?
    );
    Ok(())
}
