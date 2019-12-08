extern crate wfc_tiled;

use std::num::NonZeroU32;
use std::error::Error;

use wfc_tiled::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ::std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("usage: {} INPUT_CSV", args[0]);
        ::std::process::exit(1);
    }
    let input_path = &args[1];
    let attempts = 1000;
    let pattern_size = 2;
    let output_size = Size::new(128, 128);

    // Extract patterns from input
    let pattern = TilePattern::from_csv(input_path, NonZeroU32::new(pattern_size).unwrap(), &[Orientation::Original]).expect("Error while creating pattern");

    // Run Wave Function Collapse
    let grid = pattern.run_collapse(output_size, attempts, WrapXY, ForbidNothing, &mut rand::thread_rng()).expect("Error in WFC");

    // save as image (for preview purposes)
    let img = grid_to_image(&grid);
    img.save("out.png").expect("Failed to save");

    // save as CSV
    grid_to_csv(&grid, "out.csv")?;

    // save as Tiled .tmx file
    let tset = TileSet {
        image_path: "examples\\tilemap.png".to_string(),
        image_size: Size::new(256, 1450),
        columns: 8,
        tile_count: 360,
        tile_size: Size::new(32, 32)
    };
    grid_to_tiled(&grid, "out.tmx", tset)?;
    return Ok(());
}