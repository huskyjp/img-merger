mod args;
use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat};
use std::{ io::BufReader, fs::File };

fn main() {
    let args = Args::new();
    println!("Hello, world!");
}

fn get_tuple_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

   return (image, image_format)
}