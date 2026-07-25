//Get average Brightness
//C. Wyatt Polasek + Zach Breene

/*For help solving this program, we used the following resources:
GitHub Copilot
The pnmdata lab
Notes from 9/12 Lecture
https://docs.rs/csc411_image/latest/csc411_image/
*/

use csc411_image::{Read, GrayImage};
use std::env;

fn main() {
let input = env::args().nth(1);
assert!(env::args().len() == 2);

let img = GrayImage::read(input.as_deref()).unwrap();
let denom = img.denominator;

//Initiate variables
let mut total_grayscale = 0;
let mut pixelcount = 0;

//Iterate over pixels
for pixel in img.pixels {
    //Adds each pixel to the total
    let grayscale = pixel.value as u32;
    total_grayscale += grayscale;

    //Counts each iteration
    pixelcount += 1;
}

//Calculations for Average Grayscale and Brightness
let average_grayscale = total_grayscale as f32 / pixelcount as f32; 
let brightness = average_grayscale as f32 / denom as f32;


//Printig output: Brightness (avg_grayscale / denominator)
println!("{}", format!("{:.3}", brightness));
}