use crate::api_wrappers::{APIWrapper, ResourceType, get_api_wrapper};
use reqwest::{Response, Url};
use std::path::Path;
use std::fs::File;
use std::io::Write;
// use futures::StreamExt;
use async_fetcher::AsyncFetcher;
use futures::executor::block_on;
// use futures::stream::{Stream, StreamExt};
// use tokio_compat_02::FutureExt;
use std::str::FromStr;

fn define_resource_type_by_url(url: &str) -> Option<ResourceType> {
    match url {
        x if x.contains("vk.com") => Some(ResourceType::VK),
        _ => None
    }
}

fn prepare_dir(dir_path: &str) {
    std::fs::create_dir_all(&dir_path);
}

async fn download_file(downloadable_url: String, file_path: String) {
    /// Download file to dir
    let downloadable_url = String::from(downloadable_url);
    println!("download req for {}", downloadable_url);
    // let url = Url::parse(&downloadable_url.split("?").collect::<Vec<&str>>()[0]);
    let url = Url::parse(&downloadable_url);
    println!("{:?}", url);


    // let client = reqwest::Client::new();
    // let file_bytes = reqwest::get(downloadable_url).await
    let file_bytes = reqwest::blocking::get(downloadable_url)
        .expect("Error on downloading file")
        .bytes().expect("Error on getting bytes");

    let mut file = File::create(file_path).expect("File not created");
    file.write(&file_bytes).expect("Error on writing bytes to file");
}

fn download_files_to_dir(file_url_list: Vec<String>, dir_path: String) {
    // std::fs::create_dir_all(&dir_path);
    // let download_futures = futures::stream::iter(
    //     file_url_list.into_iter().enumerate().map(|(index, file_url)| {
    //         let file_path = format!("{}/{}.jpg", dir_path, index);
    //         let asd_future = download_file(file_url, file_path);
    //         asd_future
    //     })
    // ).buffer_unordered(8).collect::<Vec<()>>();
    // block_on(download_futures);
    for (index, file_url) in file_url_list.into_iter().enumerate() {
        let file_path = format!("{}/{}.jpg", dir_path, index);
        let asd_future = download_file(file_url, file_path);
        block_on(asd_future);
    }


    // for (index, file_url) in file_url_list.iter().enumerate() {
    //     let file_path = format!("{}/{}.jpg", dir_path, index);
    //     let download_futures = futures::stream::iter(
    //         download_file(file_url, &file_path)
    //     ).buffer_unordered(8).collect::<Vec<()>>();
    // }
}

fn download_photos_to_dir(photos_link_list: Vec<String>, dir_path: String) {
    std::fs::create_dir_all(&dir_path);
    // let client = reqwest::Client::new();
    // let asd = reqwest::get("123");

    // let tasks: Vec<_> = photos_link_list.iter().map(|url| {
    //     tokio::spawn(reqwest::get(url))
    // }).collect();
    //
    // for task in tasks {
    //     tasks
    //     let mut file = File::create(format!("{}/{}.jpg", dir_path, index)).expect("Not created");
    //     file.write(tasks)
    // }
    //
    // let download_futures = futures::stream::iter(
    //     photos_link_list.into_iter().enumerate().map(|(index, link)| {
    //         let dir_path = dir_path.clone();
    //         let client = reqwest::Client::new();
    //         async move {
    //             println!("downloading {}", link);
    //             let url = Url::parse("https://sun9-22.userapi.com/impg/JiQQwM0lM7k4Nl5qmV0jTx1rcMpfAo6Bic29Ig/STR1iPwmqKU.jpg?size=1280x853&quality=96&sign=14f2ee297ee0670b1afb9a5e956820db&c_uniq_tag=TFap3fcGpM8Nf2SzYcTlKUYzqq0JhcIW0ptqT6yoOT4&type=album").unwrap();
    //             println!("url: {}", url);
    //             match client.get(url).send().await {
    //                 Ok(resp) => {
    //                     match resp.bytes().await {
    //                         Ok(buf) => {
    //                             println!("bytes: {:?}", buf.len());
    //                             let mut file = File::create(format!("{}/{}.jpg", dir_path, index)).expect("Not created");
    //                             let mut content = std::io::Cursor::new(buf);
    //                             // println!("content: {:?}", content);
    //                             std::io::copy(&mut content, &mut file);
    //                             // println!("RESPONSE: {} bytes from {}", text.len(), path);
    //                         }
    //                         Err(err) => println!("BYTES READING ERROR {}, {}", link, err),
    //                     }
    //                 }
    //                 Err(err) => println!("{} - ERROR downloading {}; {}", index, link, err),
    //             }
    //         }
    //     })
    // ).buffer_unordered(8).collect::<Vec<()>>();
    // println!("Waiting...");
    // futures::executor::block_on(download_futures);
    // // fetches.await;

    // other code block
    // let iter_resp_bytes = futures::stream::iter(photos_link_list)
    //     .map(|url| {
    //         // let client = &client;
    //         async move {
    //             let resp = client.get(url).send().await.unwrap();
    //             resp.bytes().await
    //         }
    //     });
    // for (index, download_req_task) in iter_resp_bytes.enumerate() {
    //     let mut file = File::create(format!("{}.jpg", index)).expect("Not created");
    //     async {
    //         let buf = download_req_task.await.bytes().await;
    //         file.write(buf);
    //     };

    // let tasks_list: Vec<_> = photos_link_list.into_iter().map(
    //     |link| client.get(link).send()
    // ).collect();


    // async fn asd(url: &str) -> JoinHandle<_> {
    //     tokio::spawn(
    //         async {
    //             client.get(link).send()
    //         }
    //     )
    // }

    // let tasks_list: Vec<_> = photos_link_list.iter()
    //     .map(|link| tokio::spawn(client.get(link).send()))
    //     .collect();
    //
    // // let tasks_list: Vec<_> = photos_link_list.into_iter().map(
    // //     |link| client.get(link).send()
    // // ).collect();
    //
    // println!("task list: {:?}", tasks_list.len());
    // println!("ashdkas {:?}", tasks_list.last());
    //
    // // for (index, d_task) in tasks_list.enumerate() {
    // //     d_task.await.unwrap()
    // // }
    //
    //
    // for (index, download_req_task) in tasks_list.into_iter().enumerate() {
    //     let mut file = File::create(format!("{}/{}.jpg", dir_path, index)).expect("Not created");
    //     let downloading_block = async {
    //         println!("{:?}", download_req_task);
    //         download_req_task.await.unwrap().bytes();
    //         let buf = download_req_task.await.unwrap().bytes().await.unwrap();
    //         let mut content = std::io::Cursor::new(buf);
    //         println!("content: {:?}", content);
    //         std::io::copy(&mut content, &mut file);
    //         file.write();
    //     };
    //     futures::executor::block_on(downloading_block);
    // }
}

// fn download_photos_to_dir2(photos_link_list: Vec<String>, dir_path: String) {
//     // let client = reqwest::async::Client::new();
//     for photo_link in photos_link_list.into_iter() {
//         let fetcher = AsyncFetcher::new(&client, photo_link);
//     }
// }

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_resource_def() {
        let url1 = "https://vk.com/decidetodevelop?z=album-192278639_27919574";
        let res_type1 = define_resource_type_by_url(url1);
        assert!(matches!(res_type1, Some(ResourceType::VK)));
    }

    #[test]
    fn test_downloading() {
        // use api_we
        let url1 = "https://vk.com/decidetodevelop?z=album-192278639_279195741";
        let dir = "test_users_dir/album".to_string();
        let vk_api_wrapper = get_api_wrapper(ResourceType::VK);

        // let tmp_option_photos = vk_api_wrapper.get_photo_list(url1);
        // println!("{:?}", tmp_option_photos);
        let album_photos = vk_api_wrapper.get_photo_list(url1).unwrap();
        // download_photos_to_dir(album_photos, dir);
        download_files_to_dir(album_photos, dir);


        // download_photos_to_dir()
        // let res_type1 = define_resource_type_by_url(url1);
        // assert!(matches!(res_type1, Some(ResourceType::VK)));
    }

}