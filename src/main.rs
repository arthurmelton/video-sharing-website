#[macro_use] extern crate rocket;

use rocket::response::stream::ReaderStream;
use crate::rocket::tokio::io::{AsyncWriteExt, AsyncReadExt};
use rocket::tokio::fs::File;
use std::fs;
use rocket::fs::{NamedFile, relative};
use std::path::{PathBuf, Path};
use rand::prelude::SliceRandom;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use rocket::Data;
use rocket_seek_stream::SeekStream;

const CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9',
];

#[get("/video/<path>")]
async fn video(path: &str) -> Option<(ContentType, String)> {
    let mut file = File::open("./www/video.html").await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;
    Some((ContentType::HTML, contents.replace("$video_id", path)))
}

#[get("/videos/<path>")]
async fn videos<'a>(path: &str) -> std::io::Result<(ContentType, std::io::Result<SeekStream<'a>>)> {
    Ok((ContentType::Binary, SeekStream::from_path(format!("./videos/{}", path))))
}

#[post("/upload", data = "<video>")]
async fn write_video(content_type: &ContentType, video: Data<'_>) -> Option<String> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::raw("video").size_limit(536870912),
        ]
    );
    let multipart_form_data = MultipartFormData::parse(content_type, video, options).await.unwrap();
    let mut name = String::new();
    let mut first = true;
    while Path::new(&name).exists() || first {
        name = "./videos/".to_string();
        for _x in 0..10 {
            name.push_str(
                CHARS
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
        }
        first = false;
    }
    let mut file = File::create(name.clone()).await.ok()?;
    file.write_all(&multipart_form_data.raw.get("video")?[0].raw).await.ok()?;
    Some(name[9..].to_string())
}

#[delete("/delete?<id>")]
async fn del_video(id: &str) -> Option<()> {
    fs::remove_file(format!("./videos/{}", id)).ok()?;
    Some(())
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open("./www/404.html").await.ok()
}

#[get("/<path..>")]
async fn _static(mut path: PathBuf) -> Option<rocket::fs::NamedFile> {
    if path == PathBuf::new() {
        path = PathBuf::from("index.html");
    }
    let path = Path::new(relative!("www")).join(path);
    rocket::fs::NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![video, videos, del_video, _static, write_video])
}

/*
fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    fs::create_dir_all("videos").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let mut request: Vec<u8> = Vec::new();
            let mut buf = [0; 4096];
            while stream.read(&mut buf).unwrap() == 4096 {
                for i in buf {
                    request.push(i);
                }
            }
            for i in buf {
                request.push(i);
            }
            let response: String = String::from_utf8_lossy(&request[..4]).to_string();
            if response == "POST" {
                for _ in 0..response
                    .split("\n")
                    .nth(response.split("\n").count() - 2)
                    .unwrap()
                    .len()
                    + 3
                {
                    request.pop();
                }
                for _ in 0..response.split("Content-Type: ").next().unwrap().len()
                    + 14
                    + response.split("Content-Type: ").nth(1).unwrap().len()
                {
                    request.remove(0);
                }
                let mut index = 0;
                let mut second = false;
                loop {
                    let i = response.lines().nth(index).unwrap();
                    if i.starts_with("Content-Type: ") {
                        if second == true {
                            for _ in 0..i.len() + 4 {
                                request.remove(0);
                            }
                            break;
                        }
                        second = !second;
                    }
                    index += 1;
                }
            }
            if response.split(' ').count() > 1 {
                let wants = response.split(' ').nth(1).unwrap();
                if wants.starts_with("/upload") {
                    if wants == "/upload?done" {
                        let chars = [
                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                            'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
                            'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3',
                            '4', '5', '6', '7', '8', '9',
                        ];
                        let mut name = "./videos/".to_string();
                        for _x in 0..10 {
                            name.push_str(
                                chars
                                    .choose(&mut rand::thread_rng())
                                    .unwrap()
                                    .to_string()
                                    .as_str(),
                            );
                        }
                        while Path::new(&name).exists() {
                            name = "./videos/".to_string();
                            for _x in 0..10 {
                                name.push_str(
                                    chars
                                        .choose(&mut rand::thread_rng())
                                        .unwrap()
                                        .to_string()
                                        .as_str(),
                                );
                            }
                        }
                        fs::rename(
                            format!(
                                "./videos/{}",
                                stream
                                    .peer_addr()
                                    .unwrap()
                                    .to_string()
                                    .split(":")
                                    .next()
                                    .unwrap()
                            ),
                            name.clone(),
                        )
                        .unwrap();
                        stream
                            .write_all(
                                format!(
                                    "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}",
                                    name.len() - 9,
                                    name[9..].to_string()
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                        stream.flush().unwrap();
                    } else {
                        println!(
                            "{}",
                            stream
                                .peer_addr()
                                .unwrap()
                                .to_string()
                                .split(":")
                                .next()
                                .unwrap()
                        );
                        let mut f = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(format!(
                                "./videos/{}",
                                stream
                                    .peer_addr()
                                    .unwrap()
                                    .to_string()
                                    .split(":")
                                    .next()
                                    .unwrap()
                            ))
                            .unwrap();
                        f.write_all(&request).expect("write failed");
                        stream
                            .write_all("HTTP/1.1 200 Ok\r\n\r\n".as_bytes())
                            .unwrap();
                        stream.flush().unwrap();
                    }
                } else if wants.starts_with("/delete") {
                    fs::remove_file(format!("./videos/{}", wants[8..].to_string())).unwrap();
                    stream
                        .write_all("HTTP/1.1 200 Ok\r\n\r\n".as_bytes())
                        .unwrap();
                    stream.flush().unwrap();
                } else {
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
                            } else if Path::new(format.as_str()).exists() {
                                "video.html".to_string()
                            } else if Path::new(format!(".{}", wants).as_str()).exists()
                                && (wants.starts_with("/assets/") || wants.starts_with("/videos/"))
                            {
                                format!(".{}", wants)
                            } else {
                                "404.html".to_string()
                            }
                        }
                    };
                    println!("{}", file_wants);
                    let mut f = File::open(file_wants.clone()).expect("no file found");
                    let mut buffer = Vec::new();
                    if file_wants.ends_with(".css") {
                        for i in "HTTP/1.1 200 Ok\r\nContent-type: text/css; charset=utf-8\r\n\r\n"
                            .as_bytes()
                        {
                            buffer.push(*i);
                        }
                    } else if file_wants.ends_with(".js") {
                        for i in "HTTP/1.1 200 Ok\r\nContent-type: text/javascript; charset=utf-8\r\n\r\n".as_bytes() {
                            buffer.push(*i);
                        }
                    } else {
                        for i in "HTTP/1.1 200 Ok\r\n\r\n".as_bytes() {
                            buffer.push(*i);
                        }
                    }
                    f.read_to_end(&mut buffer).expect("buffer overflow");
                    if file_wants == "video.html" {
                        buffer = String::from_utf8(buffer)
                            .unwrap()
                            .replace("$video_id", &wants[1..])
                            .replace("$host", &host)
                            .as_bytes()
                            .to_vec();
                    }
                    stream.write(&buffer).unwrap();
                    stream.flush().unwrap();
                }
            }
        });
    }
}
*/
