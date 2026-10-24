use std::{fs, error::Error};

pub fn log(s: impl ToString) -> Result<(), Box<dyn Error>> {
    if !fs::exists("/var/log/anthilus")? {
        fs::write("/var/log/anthilus", "")?;
    }

    let mut text = String::from_utf8(fs::read("/var/log/anthilus")?)?;
    text.push('\n');
    text.push_str(s.to_string().as_str());
    fs::write("/var/log/anthilus", text)?;

    Ok(())
}
