
use std::{process::Command};

pub async fn download(url: &str, pref_format: &str) -> Result<String, String>{
    // Command::new("cd ~").output().expect("Error");
    println!("Starting");
    let output = Command::new("youtube-dl")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg(pref_format)
       .output(); 
    Ok(String::from(format!("Done!{:?}",output)))
}


