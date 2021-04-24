use crate::api_wrappers::APIWrapper;
use std::env;
use std::collections::HashMap;
use reqwest::Response;
use serde_json::{Result, Value};
use rocket::http::RawStr;
use std::io::Read;

pub struct VkAPIWrapper; //unit-like struct

impl APIWrapper for VkAPIWrapper {  //implementation APIWrapper
    fn get_photo_list(album_url: &str) -> Vec<&str> {
       // VkAPIWrapper::vk_api_req();
        Vec::new()
    }
}

impl VkAPIWrapper {
    fn vk_api_req(method: &str, kwargs: &HashMap<&str, &str>) {
        let vk_api_token = env::var("VK_API_TOKEN").unwrap();
        let client = reqwest::blocking::Client::new();
        let version = "5.130";
        let mut params = String::new();

        for (k, v) in kwargs.iter() {
            params += &format!("{}={}&", k, v);
        }
        println!("params is '{}'", params);

        let req_url = format!(
            "https://api.vk.com/method/{method}?{params}v={version}&access_token={token}",
            method=method, params=params, version=version, token=vk_api_token
        );

        // let req_url = format!("https://api.vk.com/method/{method}", method=method);

        println!("generated url: {}", req_url);

        // let req_task = client.post(req_url).send();
        // req_task
        // let resp = client.post(req_url)
        //     .json(&kwargs).send();
        let resp = client.get(req_url).send();
        // let hm = HashMap::new();
        println!("{:?}", resp);
        let t = resp.unwrap().text().unwrap();
        // let t = r#"{\'response\':[{\'first_name\':\'Artem\',\'id\':17348156,\'last_name\':\'Blinov\',\'can_access_closed\':true,\'is_closed\':false}]}"#;
        println!("{:?}", t.replace("\\", ""));
        println!("{:?}", t.contains("\\"));
        // println!("{:?}", t.text());
        let v: Value = serde_json::from_str(&t).unwrap();
        println!("{:?}", v);
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
        req_args.insert("user_ids", "17348156");
        let resp = super::VkAPIWrapper::vk_api_req("users.get", &req_args);
        println!("{:?}", resp);
    }

    #[test]
    fn test_get_photo_list() {

    }
}