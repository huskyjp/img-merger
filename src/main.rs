mod args;
use args::Args;

use image::{ io::Reader, DynamicImage, ImageFormat};
use std::{ io::BufReader, fs::File };

#[derive(Debug)] 
enum FormatImageError {
    NotCompatibleFormatImage,
}

fn main() -> Result<(), FormatImageError> {
    let args = Args::new();
    let (first_image, first_image_format) = get_tuple_image_from_path(args.first_image);
    let (second_image, secon_image_format) = get_tuple_image_from_path(args.second_image);

    if first_image_format != secon_image_format {
    return Err(FormatImageError::NotCompatibleFormatImage)
    }
    println!("Image is compatible and successfully compiled!");
    Ok(())
}


fn get_tuple_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

   return (image, image_format)
}