
use std::{process::Command};

use futures::executor::block_on;

async fn download_audio(url: &str, pref_format: &str){
    // Command::new("cd ~").output().expect("Error");
    println!("Starting");
    let output = Command::new("youtube-dl")
       .arg(url)
       .arg("--extract-audio")
       .arg("--audio-format")
       .arg("{pref_format}")
       .output()
       .expect("Error!");
    println!("{:?}",output);
}

pub fn download(url: &str, pref_format: &str){
    block_on(download_audio(url, pref_format));
}

