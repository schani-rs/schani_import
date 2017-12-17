use std::error::Error;
use std::io::Read;
use std::io::Write;

use models::Import;

use hyper;
use hyper::client::Client;
use hyper::method::Method;
use serde_json::{self, Value};
use serde_urlencoded;

fn build_new_raw_image_body(
    user_id: i32,
    camera: &String,
    longitude: f64,
    latitude: f64,
) -> String {
    serde_urlencoded::to_string([
        ("user_id", user_id.to_string()),
        ("camera", camera.to_owned()),
        ("longitude", longitude.to_string()),
        ("latitude", latitude.to_string()),
    ]).unwrap()
}

fn save_raw_image_data(import: &Import) -> Result<i32, String> {
    let client = Client::new();
    let mut resp: hyper::client::Response = try!(
        client
            .post("http://store:8000/api/raw_images")
            .header(hyper::header::ContentType::form_url_encoded())
            .body(&build_new_raw_image_body(
                import.user_id,
                &import.camera,
                import.latitude,
                import.longitude,
            ))
            .send()
            .map_err(|err| format!("could not save image data to store: {}", err))
    );
    if resp.status != hyper::status::StatusCode::Created {
        return Err(format!(
            "unexpected HTTP status {} when sending image data to store",
            resp.status
        ));
    }

    let mut resp_text = String::new();
    try!(
        resp.read_to_string(&mut resp_text)
            .map_err(|err| { format!("could not read response: {}", err) })
    );
    let resp_json: Value = try!(
        serde_json::from_str(&resp_text)
            .map_err(|err| { format!("could not read response JSON: {}", err) })
    );
    let id = resp_json["id"].as_u64().unwrap() as i32;

    Ok(id)
}

fn build_new_image_body(
    title: String,
    description: String,
    license: String,
    side_car_file: String,
    raw_image_id: i32,
) -> String {
    serde_urlencoded::to_string([
        ("title", title.to_owned()),
        ("description", description.to_owned()),
        ("license", license.to_owned()),
        ("side_car_file", side_car_file.to_owned()),
        ("raw_image_id", raw_image_id.to_string()),
    ]).unwrap()
}

fn save_image_data(
    title: String,
    description: String,
    license: String,
    side_car_file: String,
    raw_image_id: i32,
) -> Result<i32, String> {
    let client = Client::new();
    let mut resp: hyper::client::Response = try!(
        client
            .post("http://store:8000/api/images")
            .header(hyper::header::ContentType::form_url_encoded())
            .body(&build_new_image_body(
                title,
                description,
                license,
                side_car_file,
                raw_image_id,
            ))
            .send()
            .map_err(|e| format!("{}", e.description()))
    );
    if resp.status != hyper::status::StatusCode::Created {
        return Err(format!(
            "unexpected HTTP status {} when sending image data to store",
            resp.status
        ));
    }

    let mut resp_text = String::new();
    try!(
        resp.read_to_string(&mut resp_text)
            .map_err(|e| { format!("{}", e.description()) })
    );
    let resp_json: Value = try!(
        serde_json::from_str(&resp_text)
            .map_err(|err| { format!("could not read response JSON: {}", err) })
    );
    let id = resp_json["id"].as_u64().unwrap() as i32;

    Ok(id)
}

fn save_raw_file(raw_file_id: i32, data: &mut Read) -> Result<(), Box<Error>> {
    let mut buffer = vec![];
    try!(data.read_to_end(&mut buffer));

    let image_url = try!(hyper::Url::parse(&format!(
        "http://store:8000/api/raw_images/{}",
        raw_file_id
    ),));
    let mut req = try!(hyper::client::Request::new(Method::Post, image_url));
    req.headers_mut()
        .set(hyper::header::ContentLength(buffer.len() as u64));
    let mut stream = try!(req.start());
    try!(stream.write_all(buffer.as_slice()));
    try!(stream.flush());
    let resp = try!(stream.send());

    assert_eq!(resp.status, hyper::status::StatusCode::Created);

    Ok(())
}

fn save_sidecar_file(image_id: i32, data: &mut Read) -> Result<(), Box<Error>> {
    let mut buffer = vec![];
    try!(data.read_to_end(&mut buffer));

    let image_url = try!(hyper::Url::parse(&format!(
        "http://store:8000/api/images/{}/sidecar",
        image_id
    )));
    let mut req = try!(hyper::client::Request::new(Method::Post, image_url));
    req.headers_mut()
        .set(hyper::header::ContentLength(buffer.len() as u64));
    let mut stream = try!(req.start());
    try!(stream.write_all(buffer.as_slice()));
    try!(stream.flush());
    let resp = try!(stream.send());

    assert_eq!(resp.status, hyper::status::StatusCode::Created);

    Ok(())
}

pub fn transfer_raw_image_to_store(import: &Import, data: &mut Read) -> Result<i32, Box<Error>> {
    let raw_file_id = try!(save_raw_image_data(import));
    try!(save_raw_file(raw_file_id, data));

    Ok(raw_file_id)
}

pub fn transfer_sidecar_file_to_store(import: &Import, data: &mut Read) -> Result<i32, Box<Error>> {
    let image_id = try!(save_image_data(
        "test".to_string(),
        "desc".to_string(),
        "CC".to_string(),
        "default".to_string(),
        try!(import.raw_image_id.ok_or("no raw_image_id set")),
    ));
    try!(save_sidecar_file(image_id, data));

    Ok(image_id)
}
