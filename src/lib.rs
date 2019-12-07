extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

use std::error::Error;
use std::num::NonZeroU32;

pub use coord_2d::{Coord, Size};
pub use wfc::{ForbidNothing, ForbidInterface, ForbidPattern, Wrap, PatternId};
pub use wfc::wrap::WrapXY;
pub use wfc::orientation::{Orientation};

mod forbid_corner;
mod tile_pattern;
mod save_tiled;
mod save_image;
mod save_csv;

pub use tile_pattern::TilePattern;
pub use forbid_corner::ForceBorderForbid;
pub use save_tiled::{grid_to_tiled, TileSet};
pub use save_image::grid_to_image;
pub use save_csv::grid_to_csv;

fn main() -> Result<(), Box<dyn Error>> {
    let pattern_size = 2;
    let pattern = TilePattern::from_csv("house1.csv", NonZeroU32::new(pattern_size).unwrap(), &[Orientation::Original]).expect("Pattern Err");
    let forbid = ForceBorderForbid::new(&pattern, pattern_size);
    let grid = pattern.run_collapse(Size::new(128, 128), 1000, WrapXY, ForbidNothing, &mut rand::thread_rng()).expect("Err");
    let img = grid_to_image(&grid);
    img.save("outhouse.png").expect("Failed to save");
    let tset = TileSet {
        image_path: "../../../Documents/magecity_1.png".to_string(),
        image_size: Size::new(256, 1450),
        columns: 8,
        tile_count: 360,
        tile_size: Size::new(32, 32)
    };
    grid_to_tiled(&grid, "outhouse.tmx", tset)?;
    return Ok(());
}
