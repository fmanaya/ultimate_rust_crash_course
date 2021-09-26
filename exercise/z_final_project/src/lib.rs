//extern crate colored; // not needed in Rust 2018

use colored::*;
use std::path::Path;
use std::convert::TryInto;

pub fn blur(infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(2.0);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, brightness: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(brightness); // takes one argument, an i32.  Positive numbers brighten the image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    //let img2 = 
    img.crop(x, y, width, height); // takes four arguments: x: u32, y: u32, width: u32, height: u32
                                              // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String, angle: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    
    // There are 3 rotate functions to choose from (all clockwise):
    match angle {
        90 => {
            let img2 = img.rotate90();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        180 => {
            let img2 = img.rotate180();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        270 => {
            let img2 = img.rotate270();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        _ => {
            //print_usage_and_exit();  // TODO: usage
        }
    }
   
}

pub fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn grayscale(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn generate(outfile: String, rgb: [u8; 3]) {
    // Create an ImageBuffer -- see fractal() for an example
    let w: u32 = 200;
    let h: u32 = 300;
    let mut f_rgb = rgb;
    let mut imgbuf = image::ImageBuffer::new(w, h);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    
    let tercio = h / 3;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
         // Set the image to some solid color. -- see fractal() for an example
        //*pixel = image::Rgb([red, 0, 0]);

        if y < tercio {
            // rango 1
            println!("x {}, y:{}", x.to_string().red(), y.to_string().red());
            *pixel = image::Rgb(f_rgb);
        } else if y < (tercio * 2){
            f_rgb[0] = rgb[1];
            f_rgb[1] = rgb[2];
            f_rgb[2] = rgb[0];
            println!("x {}, y:{}", x.to_string().green(), y.to_string().green());
            *pixel = image::Rgb(f_rgb);
        } else {
            f_rgb[0] = rgb[2];
            f_rgb[1] = rgb[0];
            f_rgb[2] = rgb[1];
            println!("x {}, y:{}", x.to_string().blue(), y.to_string().blue());
            *pixel = image::Rgb(f_rgb);
        }
        
        
    }
   
    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    if Path::new(&outfile).exists() {
        println!("fractal output file {} already exists!!!", outfile);
        return;
    }

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
