use rand::prelude::SliceRandom;
use std::fs::{OpenOptions, self, File};
use std::path::Path;
use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    fs::create_dir_all("videos").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let mut request:Vec<u8> = Vec::new();
            let mut buf = [0; 4096];
            let mut start;
            let mut total = 0;
            let mut continues = true;
            let mut looks_for = "".to_string();
            let mut host = "".to_string();
            while continues {
                if request.len() > 31 {
                    start = request.len()-31;
                }
                else {
                    start = request.len(); 
                }
                let len = stream.read(&mut buf).unwrap();
                request.extend_from_slice(&buf[..len]);
                if start == 0 {
                    for i in String::from_utf8_lossy(&buf).to_string().lines() {
                        if i.starts_with("Content-Type") {
                            match i.split("boundary=").nth(1) {
                                Some(x) => {looks_for = format!("{}", x[25..].to_string()).trim().to_string()},
                                None => {}
                            }
                        }
                        else if i.starts_with("Host: ") {
                            host = i[6..].to_string();
                        }
                    }
                }
                let returns = if_contains(request.clone(), start, total, looks_for.clone());
                continues = !returns.0;
                total = returns.1;
            }
            let response:String = String::from_utf8_lossy(&request).to_string();
            if response.starts_with("POST") {
                for _ in 0..response.split("\n").nth(response.split("\n").count()-2).unwrap().len()+3 {
                    request.pop();
                }
                for _ in 0..response.split("Content-Type: ").next().unwrap().len()+14+response.split("Content-Type: ").nth(1).unwrap().len() {
                    request.remove(0);
                }
                let mut index = 0;
                let mut second = false;
                loop {
                    let i = response.lines().nth(index).unwrap();
                    if i.starts_with("Content-Type: ") {
                        if second == true {
                            for _ in 0..i.len()+4 {
                                request.remove(0);
                            }
                            break;
                        }
                        second=!second;
                    }
                    index+=1;
                }
            }
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
                        println!("{}", stream.peer_addr().unwrap().to_string().split(":").next().unwrap());
                        let mut f = OpenOptions::new().write(true).append(true).create(true).open(format!("./videos/{}", stream.peer_addr().unwrap().to_string().split(":").next().unwrap())).unwrap();
                        f.write_all(&request).expect("write failed");
                        stream.write_all("HTTP/1.1 200 Ok\r\n\r\n".as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
                else if wants.starts_with("/delete") {
                    fs::remove_file(format!("./videos/{}", wants[8..].to_string())).unwrap();
                    stream.write_all("HTTP/1.1 200 Ok\r\n\r\n".as_bytes()).unwrap();
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
                        buffer = String::from_utf8(buffer).unwrap().replace("$video_id", &wants[1..]).replace("$host", &host).as_bytes().to_vec();
                    }
                    stream.write(&buffer).unwrap();
                    stream.flush().unwrap();
                }
            }
        });
    }
}

fn if_contains(request:Vec<u8>, start:usize, total:usize, looks_for:String) -> (bool, usize) {
    let mut index = start;
    let mut post = total;
    let mut length = looks_for.len();
    if looks_for.len() < 3 {
        length = 4;
    }
    let mut looks = vec![0;length];
    while index < request.len() {
        if index == 5 && &[looks[length-4], looks[length-3], looks[length-2], looks[length-1]] == b"POST" {
            post = 1;
        }
        for i in 0..length {
            if index >= length-i {
                looks[i]=request[index-(length-i)];
            }
        }
        index+=1;
        if (&[looks[length-4], looks[length-3], looks[length-2], looks[length-1]] == b"Host" && post == 0) || (post > 1 && index > 4096 && &looks == looks_for.as_bytes()) {
            return (true, post);
        }
        else if &[looks[length-4], looks[length-3], looks[length-2], looks[length-1]] == b"Host" || (post > 0 && &looks == looks_for.as_bytes()) {
            post += 1;
        }
    }
    return (false, post);
}
