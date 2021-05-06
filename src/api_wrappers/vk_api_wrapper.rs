use crate::api_wrappers::APIWrapper;
use std::env;
use std::collections::HashMap;
use reqwest::Response;
use serde_json::Value;
use rocket::http::RawStr;
use std::io::Read;
use std::panic;

pub struct VkAPIWrapper; //unit-like struct

impl APIWrapper for VkAPIWrapper {  //implementation APIWrapper
    fn get_photo_list(&self, album_url: &str) -> Option<Vec<String>> {
        let result = panic::catch_unwind( || {
            println!("{:?}", album_url);
            let album_data: Vec<&str> = album_url.split("album").collect::<Vec<&str>>()[1].split("_").collect();
            // println!("{:?}", album_data);
            let owner_id: String = album_data[0].to_string();
            let album_id: String = album_data[1].to_string();
            println!("owner: {}, album: {}", owner_id, album_id);

            let mut args: HashMap<&str, String> = HashMap::new();
            args.insert("owner_id", owner_id);
            args.insert("album_id", album_id);
            args.insert("count", "0".to_string());
            let photos_count = VkAPIWrapper::vk_api_req(
                "photos.get",
                &args,
            ).get("response").expect("Error in VK response")
                .get("count").unwrap().as_u64().unwrap();

            let mut photos_link_list: Vec<String> = Vec::new(); //список ссылок на фото
            let part_length = 20;
            for offset in (0..photos_count).step_by(part_length) {
                args.insert("count", part_length.to_string());
                args.insert("offset", offset.to_string());
                let photos_api_resp = VkAPIWrapper::vk_api_req(
                    "photos.get",
                    &args
                );
                let items = photos_api_resp.get("response").unwrap()
                    .get("items").unwrap();

                for item in items.as_array().unwrap() {
                    let last_size = item["sizes"].as_array().unwrap().last().unwrap();
                    let photo_url = last_size["url"].to_string();
                    // println!("{:?}", photo_url);
                    photos_link_list.push(photo_url);
                }

                // println!("response {:?}", asd);
                // let photo_items = photos_api_resp["response"]["items"];

            }
            photos_link_list
        });
        match result {
            Ok(res)=> Some(res),
            Err(error) => {
                println!("error 123123 in vk get_photos_list{:?}", error);
                None
            },
        }
    }
}

impl VkAPIWrapper {
    fn new() -> VkAPIWrapper {
        VkAPIWrapper
    }
    fn vk_api_req(method: &str, kwargs: &HashMap<&str, String>) -> Value{
        let vk_api_token = env::var("VK_API_TOKEN").expect("NO VK API TOKEN");
        let client = reqwest::blocking::Client::new();
        let version = "5.130";
        let mut params = String::new();

        for (k, v) in kwargs.iter() {
            params += &format!("{}={}&", k, v);
        }
        // println!("params is '{}'", params);

        let req_url = format!(
            "https://api.vk.com/method/{method}?{params}v={version}&access_token={token}",
            method=method, params=params, version=version, token=vk_api_token
        );

        // let req_url = format!("https://api.vk.com/method/{method}", method=method);

        // println!("generated url: {}", req_url);

        // let req_task = client.post(req_url).send();
        // req_task
        // let resp = client.post(req_url)
        //     .json(&kwargs).send();
        let resp = client.get(req_url).send();
        // let hm = HashMap::new();
        // println!("{:?}", resp);
        let t = resp.unwrap().text().unwrap_or(String::from("{'error': '123'}"));
        // let t = r#"{\'response\':[{\'first_name\':\'Artem\',\'id\':17348156,\'last_name\':\'Blinov\',\'can_access_closed\':true,\'is_closed\':false}]}"#;
        // println!("{:?}", t.replace("\\", ""));
        // println!("{:?}", t.contains("\\"));
        // println!("{:?}", t.text());
        serde_json::from_str(&t).expect("Unable to parse JSON")
        // println!("{:?}", v);
        // v.unwrap()

    }
}

#[cfg(test)]
mod vk_api_wrapper_tests {
    use super::*;

    #[test]
    fn test_vk_api_req() {
        println!("running test asd asd");
        let mut req_args = HashMap::new();
        req_args.insert("user_ids", "17348156".to_string());
        let resp = super::VkAPIWrapper::vk_api_req("users.get", &req_args);
        // println!("{:?}", resp);
    }

    #[test]
    fn test_get_photo_list() {
        let test_album_link = "https://vk.com/decidetodevelop?z=album-192278639_279195741";
        let photo_links = super::VkAPIWrapper::new().get_photo_list(test_album_link);
        // println!("{:?}", photo_links);

        // println!("{:?}", super::VkAPIWrapper::new().get_photo_list("not a link"));

    }
}