use image::ImageBuffer;

const WIDTH: u32 = 10000;
const HEIGHT: u32 = 10000;

fn main() {
    let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
