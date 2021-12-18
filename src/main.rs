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
            let mut request:Vec<u8> = Vec::new();
            let mut buf = [0; 4096];
            let mut start;
            let mut total = 0;
            let mut continues = true;
            while continues {
                start = request.len();
                let len = stream.read(&mut buf).unwrap();
                request.extend_from_slice(&buf[..len]);
                let returns = if_contains(request.clone(), start, total);
                continues = !returns.0;
                total = returns.1;
            }
            let response:String = String::from_utf8_lossy(&request).to_string();
            println!("{}", response.len());
            let response = response[..response.len()-63].to_string();
            if response.split(' ').count() > 1 {
                let wants = response.split(' ').nth(1).unwrap();
                if wants.starts_with("/upload") {
                    if wants == "/upload?done" {
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
                        fs::rename(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").next().unwrap()), name.clone()).unwrap();
                        stream.write_all(format!("HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}", name.len()-9, name[9..].to_string()).as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                    else {
                        let mut f = OpenOptions::new().write(true).append(true).create(true).open(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").next().unwrap())).unwrap();
                        f.write_all(response.split("\r\n\r\n").nth(2).unwrap().as_bytes()).expect("write failed");
                        stream.write_all("HTTP/1.1 200 Ok\r\n\r\n".as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
                else {
                    let file_wants = match wants {
                        "/" => "index.html".to_string(),
                        "/index.html" => "index.html".to_string(),
                        "/style.css" => "style.css".to_string(),
                        "/favicon.ico" => "favicon.ico".to_string(),
                        "/main.js" => "main.js".to_string(),
                        _ => {
                            let format = format!("./videos{}", wants);
                            if wants.contains("..") {
                                "404.html".to_string()
                            }
                            else if Path::new(format.as_str()).exists() {
                                "video.html".to_string()
                            }
                            else if Path::new(format!(".{}", wants).as_str()).exists() && (wants.starts_with("/assets/") || wants.starts_with("/videos/")) {
                                format!(".{}", wants)
                            }
                            else {
                                "404.html".to_string()
                            }
                        }
                    };
                    println!("{}", file_wants);
                    let mut f = File::open(file_wants.clone()).expect("no file found");
                    let mut buffer = Vec::new();
                    if file_wants.ends_with(".css") {
                        for i in "HTTP/1.1 200 Ok\r\nContent-type: text/css; charset=utf-8\r\n\r\n".as_bytes() {
                            buffer.push(*i);
                        }
                    }
                    else if file_wants.ends_with(".js") {
                        for i in "HTTP/1.1 200 Ok\r\nContent-type: text/javascript; charset=utf-8\r\n\r\n".as_bytes() {
                            buffer.push(*i);
                        }
                    }
                    else {
                        for i in "HTTP/1.1 200 Ok\r\n\r\n".as_bytes() {
                            buffer.push(*i);
                        }
                    }
                    f.read_to_end(&mut buffer).expect("buffer overflow");
                    if file_wants == "video.html" {
                        buffer = String::from_utf8(buffer).unwrap().replace("$video_id", &wants[1..]).as_bytes().to_vec();
                    }
                    stream.write(&buffer).unwrap();
                    stream.flush().unwrap();
                }
            }
        });
    }
}

fn if_contains(request:Vec<u8>, start:usize, total:usize) -> (bool, usize) {
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;
    let mut index = start;
    let mut post = total;
    while index < request.len() {
        if index == 4 && &[one, two, three, four] == b"POST" {
            post = 1;
        }
        one = two;
        two = three;
        three = four;
        four = request[index];
        index+=1;
        if (&[one, two, three, four] == b"\r\n\r\n" && post == 0) || (post == 60 && &[one, two, three, four] == b"----") {
            return (true, post);
        }
        else if &[one, two, three, four] == b"\r\n\r\n" || (post > 0 && &[one, two, three, four] == b"----") {
            post += 1;
        }
    }
    return (false, post);
}