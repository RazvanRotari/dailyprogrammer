extern crate image;  

use std::io::File;
use std::os;
use image::{open, Rgba, DynamicImage, ImageBuf, GenericImage, ImageRgba8};


fn main() {   
//    let file_path = Path::new(os::args()[1].as_slice());
    let file_path = Path::new("a.png");
    let image_result = open(&file_path);
    let img:DynamicImage;
    match image_result {
        Ok(r) => img = r,
        _ => return
    }

    let image_buffer = img.to_rgba();
    let (width, height) = image_buffer.dimensions();
    let output_image_buffer = ImageBuf::<u8, Rgba<u8>>::from_fn(width, height, |x, y| {
        let red_weight = 0.21f32;
        let green_weight = 0.72f32;
        let blue_weight = 0.07f32;
        
        let (red, green, blue, alpha) = image_buffer.get_pixel(x,y).channels();
        let gray = (red_weight * red as f32 + green_weight * green as f32 + blue_weight * blue as f32) as u8;
        Rgba(gray, gray, gray, alpha)
        
    });
    let mut output_file_path = file_path.clone();
    output_file_path.set_extension("grayscale.png");
    let out_file = File::create(&output_file_path).unwrap();
    println!("The new file is :{}", output_file_path.display());
    let _ = ImageRgba8(output_image_buffer).save(out_file, image::PNG);
}
