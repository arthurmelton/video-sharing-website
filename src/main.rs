use rand::prelude::SliceRandom;
use std::fs::{OpenOptions, self, File};
use std::path::Path;
use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9377").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let mut buffer = [0;1048576];
            stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[..]);
            let wants = response.split(" ").nth(1).unwrap();
            if wants.starts_with("/upload") {
                if response.split("\r\n\r\n").nth(1).unwrap().replace(" ", "") == "done" {
                    let chars = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z', '0','1','2','3','4','5','6','7','8','9'];
                    let mut name = "./videos/".to_string();
                    for _x in 0..10 {
                        name.push_str(chars.choose(&mut rand::thread_rng()).unwrap().to_string().as_str());
                    }
                    while Path::new(&name).exists() {
                        name = "./videos/".to_string();
                        for _x in 0..10 {
                            name.push_str(chars.choose(&mut rand::thread_rng()).unwrap().to_string().as_str());
                        }
                    }
                    fs::rename(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").nth(0).unwrap()), name).unwrap();
                }
                else {
                    if Path::new(stream.peer_addr().unwrap().to_string().split(":").nth(0).unwrap()).exists() {
                        let mut f = OpenOptions::new().write(true).append(true).open(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").nth(0).unwrap())).unwrap();
                        f.write_all(response.split("\r\n\r\n").nth(1).unwrap().replace(" ", "").as_bytes()).expect("write failed");
                    }
                    else {
                        let mut f = OpenOptions::new().write(true).append(true).create(true).open(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").nth(0).unwrap())).unwrap();
                        f.write_all(response.split("\r\n\r\n").nth(1).unwrap().replace(" ", "").as_bytes()).expect("write failed");
                    }
                }
                stream.write("HTTP/1.1 200 Ok\r\n\r\n".as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            else {
                let file_wants = match wants {
                    "/" => "index.html".to_string(),
                    "/index.html" => "index.html".to_string(),
                    "/style.css" => "style.css".to_string(),
                    "/favicon.ico" => "favicon.ico".to_string(),
                    "/main.js" => "main.js".to_string(),
                    _ => {
                        let format = format!("./videos/{}", wants);
                        if wants.contains("..") {
                            "404.html".to_string()
                        }
                        else if Path::new(format.clone().as_str()).exists() {
                            format
                        }
                        else {
                            "404.html".to_string()
                        }
                    }
                };
                println!("{}", file_wants);
                let mut f = File::open(file_wants).expect("no file found");
                let mut buffer = Vec::new();
                for i in "HTTP/1.1 200 Ok\r\n\r\n".as_bytes() {
                    buffer.push(*i);
                }
                f.read_to_end(&mut buffer).expect("buffer overflow");
                stream.write(&buffer).unwrap();
                stream.flush().unwrap();
            }
        });
    }
}
