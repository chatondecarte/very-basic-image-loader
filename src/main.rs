extern crate image;
extern crate minifb;


use minifb::{Key, Window, WindowOptions};
use image::GenericImageView;
use std::env;


fn main() {

	// get path from first argument
	let args :Vec<String> = env::args().collect(); 
	let path = match args.len() {
		1 => {
			   println!("i don't know which image you want i draw");
			   return;
			},
		2 => &args[1],
		_ => {
			   println!("i'm drawing the first one");
			   &args[1]
			}
	};

	// load image
    let img = match image::open(path) {
    	Ok(img) => img,
    	Err(_) => {
    				println!("i can't read that");
    				return;
    			}
    };

    // get width & height from image
    let width :usize = img.dimensions().0 as usize;
    let height :usize = img.dimensions().1 as usize;

    //create empty buffer
    let  mut buffer: Vec<u32> = vec![];

    //get pixels : return tuple (x, y, rgba[r, g, b, a])
    let px = img.pixels();
    for p in px {
		let r = p.2[0] as u32;
    	let g = p.2[1] as u32;
    	let b = p.2[2] as u32;

    	// convert rgb to decimal
    	let parsed = r*256*256 + g*256 + b;
		buffer.push(parsed as u32);
    }

// open window
let mut window = Window::new(
        path,
        width,
        height,
        WindowOptions::default(),
    ).unwrap();

	// main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

    	// update window
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }

}
