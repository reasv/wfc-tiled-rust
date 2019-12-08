//! Helper functions to use the Wave Function Collapse algorithm provided by the `wfc` crate on tile-based maps.
//! 
//! You can load layer CSV files like the ones exported from Tiled, and save the result as another CSV or as a Tiled .tmx file for previewing inside the software.
//! 
//! As the underlying library only works on two dimensions, multiple layers are not supported.
//! 
//! You should start with the [TilePattern](struct.TilePattern.html) struct.
//! 
//! Complete examples are provided in the [crate](https://crates.io/crates/wfc_tiled)'s `examples` folder.

extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

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
