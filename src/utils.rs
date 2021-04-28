use crate::api_wrappers::{APIWrapper, ResourceType};

fn define_resource_type_by_url(url: &str) -> Option<ResourceType> {
    match url {
        x if x.contains("vk.com") => Some(ResourceType::VK),
        _ => None
    }
}

fn prepare_dir() {
    todo!();
}

fn download_photos_to_dir() {
    todo!();
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_resource_def() {
        let url1 = "https://vk.com/decidetodevelop?z=album-192278639_27919574";
        let res_type1 = define_resource_type_by_url(url1);
        assert!(matches!(res_type1, Some(ResourceType::VK)));
    }

}