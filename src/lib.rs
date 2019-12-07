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
