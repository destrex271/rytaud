use std::{process::Command};
// use dotenv::dotenv;
use std::time::{Duration};
use std::thread;

pub async fn download(url: &str, key: &str, pref_format: &str) -> Result<String, String>{
    println!("Starting");
    let _ = Command::new("/yt-dlp_linux")
        .arg("--rm-cache-dir")
        .output();
    let output = Command::new("/yt-dlp_linux")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg(pref_format)
       .arg("--output")
       .arg(format!("{}.mp3",key))
       .output().expect("Unable to download");
    // Error Handling
    println!("{:?}",output.status);
    if !output.status.success(){
        return Err(String::from("Something went Wrong"));
    }
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
