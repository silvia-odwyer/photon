extern crate photon_rs;
use photon_rs::native::{open_image, save_image};
use photon_rs::multiple::{blend};
use photon_rs::transform::{resize, SamplingFilter};
use time::{PreciseTime};

fn main() {
        // Start time
        let start = PreciseTime::now();
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("fruit_1920_1080.jpg");

    // let distance_vec: Vec<f32> = photon_rs::channels::distance_runtime_select(row, row2, row3, row4);
    // println!("Distance: {:?}", distance_vec);

    // Increment the red channel by 40
    // let resized_img = resize(&mut img, 800, 600, SamplingFilter::Triangle);
    photon_rs::channels::alter_red_channel(&mut img, 30);

    let output_img_path = "output.jpg";

    save_image(img, output_img_path);
    // Output time taken.
    let end = PreciseTime::now();
    println!("Took {:?} seconds to process image.", start.to(end));
    

    // Write file to filesystem.
    // save_image(img, output_img_path);    
	
	// TEST
	
	// let mut og_img_mut = open_image("examples/input_images/nine_yards.JPG");
	// img = resize(&mut img, og_img_mut.get_width(), og_img_mut.get_height(), SamplingFilter::Nearest);
	
	// blend(&mut og_img_mut, &img, "overlay");
	
	//save_image(og_img_mut, output_img_path);
	

    println!("Saved image: {}. Please check this directory for the image.", output_img_path);
}

