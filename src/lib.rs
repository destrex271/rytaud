use std::{process::Command};
// use dotenv::dotenv;
use std::time::{self, Duration};
use std::thread;

pub async fn download(url: &str, key: &str, pref_format: &str) -> Result<String, String>{
    println!("Starting");
    Command::new("youtube-dl")
        .arg("--rm-cache-dir")
        .output();
    let output = Command::new("youtube-dl")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg(pref_format)
       .arg("--output")
       .arg(format!("{}.mp3",key))
       .output();
    println!("{:?}", output);
    Ok(String::from(format!("Ok")))
}

pub fn del_service(){
    loop{
        println!("Deleting all mp3 files");
        let t = time::SystemTime::now().elapsed();
        println!("{:?}", t);
        thread::sleep(Duration::from_secs(86400));
    }
    // let _ = Command::new("rm").arg("*.mp3");
}

pub fn delete(name: &str) -> Result<String, String>{
    let _ = Command::new("rm")
        .arg(name)
        .output();
    Ok(String::from("OK"))
}
