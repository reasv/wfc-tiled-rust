extern crate wfc_tiled;

use std::num::NonZeroU32;
use std::error::Error;

use wfc_tiled::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ::std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("usage: {} INPUT_CSV TILEMAP_FILE", args[0]);
        ::std::process::exit(1);
    }
    let input_path = &args[1];
    let tileset_path = args[2].clone();
    let attempts = 1000;
    let pattern_size = 2;
    let output_size = Size::new(32, 32);

    // Extract patterns from input
    let pattern = TilePattern::from_csv(input_path, NonZeroU32::new(pattern_size).unwrap(), &[Orientation::Original]).expect("Error while creating pattern");

    // Create forbid to force the border of the result to be equal to the contents 
    // of the bottom right corner of the input.
    // This is useful because it can effectively prevent the output from wrapping.
    let forbid = ForceBorderForbid::new(&pattern, pattern_size);

    // Run Wave Function Collapse
    let grid = pattern.run_collapse(output_size, attempts, WrapXY, forbid, &mut rand::thread_rng()).expect("Error in WFC");

    // Save as image (for preview purposes)
    let img = grid_to_image(&grid);
    img.save("out.png").expect("Failed to save");

    // Save as CSV
    grid_to_csv(&grid, "out.csv")?;

    // Save as Tiled .tmx file
    let tset = TileSet {
        image_path: tileset_path,
        image_size: Size::new(256, 1450),
        columns: 8,
        tile_count: 360,
        tile_size: Size::new(32, 32)
    };
    grid_to_tiled(&grid, "out.tmx", tset)?;
    return Ok(());
}