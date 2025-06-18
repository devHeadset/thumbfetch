use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use std::error::Error;

pub fn download_image(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut resp = get(url)?;
    let mut out = File::create(path)?;
    copy(&mut resp, &mut out)?;
    Ok(())
}
