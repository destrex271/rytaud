use std::{process::Command};
use dotenv::dotenv;

pub async fn download(url: &str, key: &str, pref_format: &str) -> Result<String, String>{
    println!("Starting");
    let output = Command::new("youtube-dl")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg(pref_format)
       .arg("--output")
       .arg(format!("{}.mp3",key))
       .output();
    Ok(String::from(format!("Ok")))
}

pub fn delete(name: &str) -> Result<String, String>{
    let _ = Command::new("rm")
        .arg(name)
        .output();
    Ok(String::from("OK"))
}
