mod args;
use args::Args;
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat, imageops::FilterType::Triangle};
use std::{io::BufReader, fs::File};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataErrors{
    DifferentImageFormats,
    BufferTooSmall,

}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall)
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result <(), ImageDataErrors>{
    let args = Args::new();
    let (image_1, image_format_1) = find_image_from_part(args.image_1);
    let (image_2, image_format_2) = find_image_from_part(args.image_2);

    if image_format_1 != image_format_2{
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_s, image_p) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_s.width(), image_s.height(), args.output);
    let combined_data = combine_images(image_s, image_p);
    output.set_data(combined_data);
    // println!("{:?}", args);

    image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format_1).unwrap();
    Ok(())
}

fn find_image_from_part(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)

}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2:(u32, u32))-> (u32, u32){
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 {dim_1} else {dim_2};
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage){
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle),image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8>{
    let vec_1 = image_1.to_rgb8().into_vec();
    let vec_2 = image_2.to_rgb8().into_vec();

    alternate_pixels(vec_1, vec_2)

}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = Vec::with_capacity(vec_1.len());

    let mut i = 0;
    while i < vec_1.len() {
        // Ensure that we don't exceed the bounds
        let end = if i + 3 < vec_1.len() { i + 3 } else { vec_1.len() - 1 };
        
        if i % 8 == 0 {
            combined_data.extend(set_rgba(&vec_1, i, end));
        } else {
            combined_data.extend(set_rgba(&vec_2, i, end));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    if start >= vec.len() || end >= vec.len() || start > end {
        eprintln!("Vector length: {}, Start: {}, End: {}", vec.len(), start, end);
        panic!("Start or end index is out of bounds");
    }
    vec[start..=end].to_vec()
}