
extern crate grid_2d;
extern crate coord_2d;

use std::error::Error;
use std::path::Path;
use grid_2d::Grid;

use coord_2d::Coord;



/// Save a [Grid](../grid_2d/grid/struct.Grid.html) to a CSV file.
/// 
/// This is meant for tile grids returned by the [TilePattern::run_collapse()](struct.TilePattern.html#method.run_collapse) method.
/// 
/// # Examples
///
/// ```
/// use wfc_tiled::grid_to_csv;
/// 
/// grid_to_csv(&grid, "output.csv")?;
/// ```
pub fn grid_to_csv<P: AsRef<Path>>(grid: &Grid<u32>, path: P) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new().from_path(path)?;
    let size = grid.size();
    for y in 0..(size.height() as i32) {
        let mut row = Vec::new();
        for x in 0..(size.width() as i32) {
            let cell: u32 = *grid.get(Coord {x, y} ).unwrap();
            row.push(cell.to_string());

        }
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}