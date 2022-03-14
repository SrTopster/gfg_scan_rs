#![windows_subsystem = "windows"]
use curl::easy::Easy;
use std::{str,thread,time};
use std::process::Command;
use std::fs::{File,OpenOptions};
use std::io::{Write,Read};
use rss::Channel;
use soloud::*;
use chrono;
use webbrowser;

fn log(name: &str) {
    let time = chrono::offset::Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    let buf = time+"|"+name+"\r\n";
    file.write_all(buf.as_bytes()).unwrap()
}
fn play_sound() {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("sci-fi.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0{
        thread::sleep(time::Duration::from_millis(100))
    }
}  
fn main() {
    loop{
        let mut data = Vec::new();
        let mut easy = Easy::new();
        easy.url("https://steamcommunity.com/groups/GrabFreeGames/rss/").unwrap();{
            let mut transfer = easy.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        if easy.response_code().unwrap() == 200 {
            let channel = Channel::read_from(&data as &[u8]).unwrap();
            let title = channel.items()[0].title().unwrap();
            let link = channel.items()[0].link().unwrap();
            let mut file = OpenOptions::new()
                .read(true)
                .create(true)
                .write(true)
                .open(".current")
                .unwrap();
            let mut temp = String::new();
            file.read_to_string(&mut temp).unwrap();
            if title != temp {
                let mut output = File::create(".current").unwrap();
                output.write(title.as_bytes()).unwrap();
                log(title);
                thread::spawn(||play_sound());
                let notify = Command::new("notifu64.exe")
                    .args(["/m","Clique para abrir no navegador.","/p",&title,"/i","icon.ico"])
                    .spawn()
                    .expect("falha a executar notifu64.exe")
                    .wait();
                if  notify.unwrap().code().unwrap() == 3{
                    webbrowser::open(&link).unwrap()    
                }
            }
        }
        else {
            println!("Request falha")
        }
        thread::sleep(time::Duration::from_secs(1800));
    }
}