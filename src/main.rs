#[macro_use] extern crate rocket;

use crate::rocket::tokio::io::{AsyncWriteExt, AsyncReadExt};
use rocket::tokio::fs::File;
use std::fs;
use rocket::fs::{NamedFile, relative};
use std::path::{PathBuf, Path};
use rand::prelude::SliceRandom;
use rocket::http::ContentType;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};
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
    File::open(format!("./videos/{}", path)).await.ok()?;
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
    rocket::build()
        .mount("/", routes![video, videos, del_video, _static, write_video])
        .register("/", catchers![not_found])
}

