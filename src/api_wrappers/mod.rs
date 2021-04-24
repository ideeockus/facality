mod vk_api_wrapper;

pub trait APIWrapper {
    fn get_photo_list(album_url: &str) -> Vec<&str>;
}
