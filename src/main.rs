use ytmp3;

fn main(){
    let url = "https://www.youtube.com/watch?v=gQlMMD8auMs";
    let format = "mp3";
    ytmp3::download(url.clone(), format.clone());
}
