extern crate image;
extern crate grid_2d;
extern crate coord_2d;

use image::{DynamicImage, Rgba, RgbaImage};
use grid_2d::Grid;
use coord_2d::Coord;

fn u32conv(x: u32) -> [u8;4] {
    let _b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, 255]
}
/// Convert a [Grid](../grid_2d/grid/struct.Grid.html) to a [DynamicImage](../image/enum.DynamicImage.html).
/// 
/// This is only meant for preview purposes, if you don't want to use
/// external software like Tiled.
/// 
/// The function simply creates an image where each pixel's color is 
/// based on the corresponding tile's ID in the grid.
/// 
/// For example, tile ID 22 is converted to RGBA(22, 0, 0, 255).
/// 
/// # Examples
///
/// ```
/// use wfc_tiled::grid_to_image;
/// 
/// let img = grid_to_image(&grid);
/// img.save("out.png").expect("Failed to save");
/// 
/// ```
pub fn grid_to_image(grid: &Grid<u32>) -> DynamicImage {
    let size = grid.size();
    let mut rgba_image = RgbaImage::new(size.width(), size.height());
    grid.enumerate().for_each(|(Coord { x, y }, cell)| {
        rgba_image.put_pixel(x as u32, y as u32, Rgba { data: u32conv(*cell)});
    });
    return DynamicImage::ImageRgba8(rgba_image);
}