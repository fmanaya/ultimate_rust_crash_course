// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");


use final_project::{ blur, fractal, brighten, crop, rotate, invert, grayscale, generate };

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile);
        }

        "brighten" => {
            if args.len() != 3 {
                // cargo run brighten dyson.png dysonBr.png 40
                print_usage_and_exit();  // TODO: usage
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let bright: i32 = args.remove(0).trim().parse().expect("Wanted a number");
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            brighten(infile, outfile,bright);
        }
        "crop" => {
            if args.len() != 6 {
                //cargo run crop dyson.png dysonCr.png 50 50 95 455
                print_usage_and_exit();  // TODO: usage
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).trim().parse().expect("Wanted a number");
            let y: u32 = args.remove(0).trim().parse().expect("Wanted a number");
            let width: u32 = args.remove(0).trim().parse().expect("Wanted a number");
            let height: u32 = args.remove(0).trim().parse().expect("Wanted a number");
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            crop(infile, outfile,  x, y, width, height);
        }

        "rotate" => {
            if args.len() != 3 {
                // cargo run rotate dyson.png dysonBr.png 90
                print_usage_and_exit();  // TODO: usage
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let angle: i32 = args.remove(0).trim().parse().expect("Wanted a number");
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            rotate(infile, outfile, angle);
        }

        "invert" => {
            if args.len() != 2 {
                // cargo run rotate dyson.png dysonBr.png 90
                print_usage_and_exit();  // TODO: usage
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
           
            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                // cargo run grayscale dyson.png dysonGr.png
                print_usage_and_exit();  // TODO: usage
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
           
            grayscale(infile, outfile);
        }
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit_f();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }
        "generate" => {
            if args.len() != 4 {
                // cargo run generate 120 200 100 new.pn
                print_usage_and_exit_f();
            }
            let r = args.remove(0).trim().parse().expect("Wanted a number");
            let g = args.remove(0).trim().parse().expect("Wanted a number");
            let b = args.remove(0).trim().parse().expect("Wanted a number");
            let outfile = args.remove(0);
            let rgb: [u8; 3] = [r, g, b];   
            generate(outfile, rgb);
        }
        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            println!("command {} not found", subcommand);
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("command INFILE OUTFILE");
    println!("command => [blur|fractal]");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}
fn print_usage_and_exit_f() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}



// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!