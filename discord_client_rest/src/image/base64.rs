use crate::image::ImageType;
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub fn encode_image(image: Vec<u8>, image_type: ImageType) -> String {
    let base64_image = STANDARD.encode(&image);
    match image_type {
        ImageType::Png => format!("data:image/png;base64,{}", base64_image),
        ImageType::Jpeg => format!("data:image/jpeg;base64,{}", base64_image),
        ImageType::Gif => format!("data:image/gif;base64,{}", base64_image),
    }
}
