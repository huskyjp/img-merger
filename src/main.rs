mod args;
use args::Args;

use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView };
use std::{ io::BufReader, fs::File, convert::TryInto };

#[derive(Debug)] 
enum FormatImageError {
    NotCompatibleFormatImage,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {

    fn new(width: u32, height: u32, name: String) -> Self {
        // let buf_cap = 3_655_744;
        let buf_cap = height * width * 4;
        let buf = Vec::with_capacity(buf_cap.try_into().unwrap());

        FloatingImage {
            width,
            height,
            data: buf,
            name,
        }
    }

}

fn main() -> Result<(), FormatImageError> {
    let args = Args::new();
    let (first_image, first_image_format) = get_tuple_image_from_path(args.first_image);
    let (second_image, secon_image_format) = get_tuple_image_from_path(args.second_image);

    if first_image_format != secon_image_format {
    return Err(FormatImageError::NotCompatibleFormatImage)
    }
    println!("Image is compatible and successfully compiled!");

    // resize the size of input images
    let (first_image, second_image) = make_size_standarised(first_image, second_image);
    let resized_output = FloatingImage::new(first_image.width(), first_image.height(), args.final_output);
    Ok(())
}


fn get_tuple_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

   return (image, image_format)
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn make_size_standarised(first_image: DynamicImage, second_image: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(first_image.dimensions(), second_image.dimensions());

    // resize first_image since first_image is larger
    if second_image.dimensions() == (width, height) {
        return (first_image.resize_exact(width, height, Triangle), second_image)
    } else {
        return (first_image, second_image.resize_exact(width, height, Triangle))
    }
}

fn merge_images(first_image: DynamicImage, second_image: DynamicImage) -> Vec<u8> {
    let first_image_vec = first_image.to_rgba8().into_vec();
    let second_image_vec = second_image.to_rgba8().into_vec();
    
   return alternate_pixels(first_image_vec, second_image_vec)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let merged_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i % 8 == 0 {
            merged_data.splice(i..=i+ 3, set_rgba(vec1));
        } else {
            merged_data.splice(i..=i+3, set_rgba(vec2));
        }
    }
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of range")
        };
        rgba.push(val);
    }

    return rgba

}