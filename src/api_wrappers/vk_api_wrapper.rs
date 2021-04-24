use crate::api_wrappers::APIWrapper;

pub struct VkAPIWrapper {
    pub name: String,
}

impl APIWrapper for VkAPIWrapper {
    fn get_photo_list(album_url: &str) -> Vec<&str> {
        todo!()

    }
}