use std::{process::Command};
// use dotenv::dotenv;
use std::time::{Duration};
use std::thread;

pub async fn download(url: &str, key: &str, pref_format: &str) -> Result<String, std::io::Error>{
    println!("Starting");
    let _ = Command::new("youtube-dl")
        .arg("--rm-cache-dir")
        .output();
    let output = Command::new("youtube-dl")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg(pref_format)
       .arg("--output")
       .arg(format!("{}.mp3",key))
       .output().expect("Unable to download");
    println!("{:?}", output);
    println!("Done");
    Ok(String::from(key))
}

pub fn del_service(){
    loop{
        println!("Deleting all mp3 files");
        let _ = Command::new("rm")
            .arg("*.mp3")
            .output();
        thread::sleep(Duration::from_secs(86400));
    }
}

pub fn delete(name: &str) -> Result<String, String>{
    let _ = Command::new("rm")
        .arg(name)
        .output();
    Ok(String::from("OK"))
}
