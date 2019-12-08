extern crate grid_2d;
extern crate coord_2d;

use std::error::Error;
use std::path::Path;
use std::io::prelude::*;

use grid_2d::Grid;
use coord_2d::{Coord, Size};

pub struct TileSet {
    pub image_path: String,
    pub image_size: Size,
    pub columns: u32,
    pub tile_size: Size,
    pub tile_count: u32
}

fn format_tiled(map: String, size: Size, tileset: TileSet) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.1" orientation="orthogonal" renderorder="right-down" compressionlevel="-1"
     width="{width}" height="{height}" tilewidth="{twidth}" tileheight="{theight}" infinite="0" nextlayerid="3" nextobjectid="1">
     <tileset firstgid="1" name="default" tilewidth="{twidth}" tileheight="{theight}" tilecount="{tcount}" columns="{tcolumns}">
      <image source="{tpath}" width="{tsetwidth}" height="{tsetheight}"/>
     </tileset>
     <layer id="1" name="base" width="{width}" height="{height}">
      <data encoding="csv">
      {layer}
      </data>
    </layer>
    </map>"#,
        layer=map,
        width=size.width(), height=size.height(),
        twidth=tileset.tile_size.width(), theight=tileset.tile_size.height(),
        tsetwidth=tileset.image_size.width(), tsetheight=tileset.image_size.height(),
        tcount=tileset.tile_count,
        tcolumns=tileset.columns,
        tpath=tileset.image_path
    )
}
/// Save a [Grid](../grid_2d/grid/struct.Grid.html) to a Tiled .tmx file.
/// 
/// This is meant for previewing the output of [TilePattern::run_collapse()](struct.TilePattern.html#method.run_collapse) 
/// in Tiled, although generated tmx files can be edited normally in the application.
/// 
/// It is fairly limited in that it creates a file with only one layer, one
/// tileset and no extra settings.
/// 
/// # Examples
///
/// ```rust
/// use wfc_tiled::grid_to_tiled;
/// let tset = TileSet {
///     image_path: "examples\\tset.png", // tile set sprite sheet
///     image_size: Size::new(256, 1450),
///     columns: 8,
///     tile_count: 360,
///     tile_size: Size::new(32, 32)
/// };
/// grid_to_tiled(&grid, "output.tmx", tset)?;
/// ```
pub fn grid_to_tiled<P: AsRef<Path>>(grid: &Grid<u32>, path: P, tileset: TileSet) -> Result<(), Box<dyn Error>> {
    let mut csv_grid = String::new();
    let size = grid.size();
    for y in 0..(size.height() as i32) {
        for x in 0..(size.width() as i32) {
            let cell: u32 = *grid.get(Coord { x, y} ).unwrap() + 1;
            csv_grid = format!("{}{}{}", csv_grid, cell, ",");
        }
        csv_grid = format!("{}{}", csv_grid, "\n");
    }
    // remove trailing newline and last comma
    csv_grid.pop();
    csv_grid.pop();
    let output = format_tiled(csv_grid, size, tileset);
    let mut file = std::fs::File::create(path)?;
    file.write_all(&output.into_bytes())?;
    Ok(())
}