use crate::api_wrappers::vk_api_wrapper::VkAPIWrapper;

mod vk_api_wrapper;

pub trait APIWrapper {
    fn get_photo_list(album_url: &str) -> Option<Vec<String>>;
}

pub enum ResourceType {
    VK, TG, FB, IG
}

pub fn get_api_wrapper(resource_type: ResourceType) -> impl APIWrapper {
    match resource_type {
        ResourceType::VK => VkAPIWrapper,
        _ => panic!()  // no wrapper for album resource type
    }
}