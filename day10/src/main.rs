use std::{collections::HashMap, error::Error, fs};

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Northeast,
    Northsouth,
    Northwest,
    Southeast,
    Southwest,
    Eastwest,
    Empty,
    Start,
}

impl Orientation {
    fn points_north(&self) -> bool {
        *self == Orientation::Northeast
            || *self == Orientation::Northsouth
            || *self == Orientation::Northwest
    }

    fn points_south(&self) -> bool {
        *self == Orientation::Northsouth
            || *self == Orientation::Southeast
            || *self == Orientation::Southwest
    }

    fn points_east(&self) -> bool {
        *self == Orientation::Eastwest
            || *self == Orientation::Northeast
            || *self == Orientation::Southeast
    }

    fn points_west(&self) -> bool {
        *self == Orientation::Eastwest
            || *self == Orientation::Northwest
            || *self == Orientation::Southwest
    }
}

impl TryFrom<char> for Orientation {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self::Northsouth),
            '-' => Ok(Self::Eastwest),
            'L' => Ok(Self::Northeast),
            'J' => Ok(Self::Northwest),
            '7' => Ok(Self::Southwest),
            'F' => Ok(Self::Southeast),
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Start),
            x => Err(String::from("Character could not be parsed: {x}")),
        }
    }
}

#[derive(Debug)]
struct Tile {
    row: usize,
    col: usize,
    orientation: Orientation,
}

impl Tile {
    fn get_adjacent_locs(&self) -> Vec<(usize, usize)> {
        let mut adjacent_locs = Vec::new();
        if self.orientation.points_north() {
            adjacent_locs.push((self.row - 1, self.col));
        }
        if self.orientation.points_south() {
            adjacent_locs.push((self.row + 1, self.col));
        }
        if self.orientation.points_east() {
            adjacent_locs.push((self.row, self.col + 1));
        }
        if self.orientation.points_west() {
            adjacent_locs.push((self.row, self.col - 1));
        }
        adjacent_locs
    }
}

#[derive(Debug)]
struct TileGrid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl TileGrid {
    fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        if row < self.height && col < self.width {
            self.tiles.get(row * self.width + col)
        } else {
            None
        }
    }

    fn checked_get(&self, row: Option<usize>, col: Option<usize>) -> Option<&Tile> {
        if let (Some(r), Some(c)) = (row, col) {
            self.get(r, c)
        } else {
            None
        }
    }
}

fn get_tile_loop(tile_grid: &TileGrid) -> Vec<&Tile> {
    let start_tile = tile_grid
        .tiles
        .iter()
        .find(|tile| tile.orientation == Orientation::Start)
        .expect("No start tile found");
    let (start_row, start_col) = (start_tile.row, start_tile.col);
    let mut tile_loop = vec![start_tile];

    println!("Start tile: {:?}", tile_grid.get(start_row, start_col));
    // Find a tile with a pipe connected to the start
    let adjacent_to_start_tile = match (
        tile_grid.checked_get(start_row.checked_sub(1), start_col.into()),
        tile_grid.checked_get(start_row.checked_add(1), start_col.into()),
        tile_grid.checked_get(start_row.into(), start_col.checked_sub(1)),
        tile_grid.checked_get(start_row.into(), start_col.checked_add(1)),
    ) {
        (Some(north_of_start), _, _, _) if north_of_start.orientation.points_south() => {
            north_of_start
        }
        (_, Some(south_of_start), _, _) if south_of_start.orientation.points_north() => {
            south_of_start
        }
        (_, _, Some(west_of_start), _) if west_of_start.orientation.points_east() => west_of_start,
        (_, _, _, Some(east_of_start)) if east_of_start.orientation.points_west() => east_of_start,
        _ => {
            panic!("No tiles found adjacent to start tile {start_tile:?}");
        }
    };

    tile_loop.insert(0, adjacent_to_start_tile);

    while (tile_loop[0].row, tile_loop[0].col) != (start_row, start_col) {
        let tile = tile_grid.get(tile_loop[0].row, tile_loop[0].col).unwrap();
        // println!("Current Tile: {tile:?}");
        let adjacent_locs = tile
            .get_adjacent_locs()
            .into_iter()
            .filter(|(row, col)| (row, col) != (&tile_loop[1].row, &tile_loop[1].col))
            .collect::<Vec<(usize, usize)>>();
        if adjacent_locs.len() != 1 {
            panic!("Error with adjacent location length: {adjacent_locs:?}");
        }
        tile_loop.insert(
            0,
            tile_grid
                .get(adjacent_locs[0].0, adjacent_locs[0].1)
                .unwrap_or_else(|| panic!("Could not find location for {:?}", adjacent_locs[0])),
        );
    }

    // println!("{tile_loop:?}");
    tile_loop
}

fn part_one(tile_grid: &TileGrid) -> usize {
    get_tile_loop(tile_grid).len() / 2
}

fn part_two(tile_grid: &TileGrid) -> usize {
    let tile_loop = get_tile_loop(tile_grid);

    // Fix Start orientation
    let n = tile_loop.len();
    let starting_direction = (
        tile_loop[1].row as isize - tile_loop[0].row as isize,
        tile_loop[1].col as isize - tile_loop[0].col as isize,
    );
    let ending_direction = (
        tile_loop[n - 2].row as isize - tile_loop[n - 1].row as isize,
        tile_loop[n - 2].col as isize - tile_loop[n - 1].col as isize,
    );
    let start_orientation = match (starting_direction, ending_direction) {
        ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => Orientation::Southeast,
        ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => Orientation::Southwest,
        ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => Orientation::Northeast,
        ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => Orientation::Northwest,
        ((1, 0), (-1, 0)) | ((-1, 0), (1, 0)) => Orientation::Northsouth,
        ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => Orientation::Eastwest,
        _ => Orientation::Empty,
    };
    println!("Start orientation was: {start_orientation:?}");

    let tile_loop_hashmap = tile_loop
        .into_iter()
        .map(|tile| {
            (
                (tile.row, tile.col),
                if tile.orientation == Orientation::Start {
                    &start_orientation
                } else {
                    &tile.orientation
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut inside_count = 0usize;

    for row in 0..tile_grid.height {
        let mut inside_loop_status = false;
        for col in 0..tile_grid.width {
            match tile_loop_hashmap
                .get(&(row, col))
                .unwrap_or(&&Orientation::Empty)
            {
                Orientation::Northsouth | Orientation::Northeast | Orientation::Northwest => {
                    inside_loop_status = !inside_loop_status;
                }
                Orientation::Empty if inside_loop_status => {
                    inside_count += 1;
                }
                _ => {}
            }
        }
    }

    inside_count
}

fn parse_file_contents(file_contents: &str) -> Result<TileGrid, Box<dyn Error>> {
    let tile_vec_vec = file_contents
        .lines()
        .zip(0usize..)
        .map(|(line, row)| {
            line.chars()
                .zip(0usize..)
                .map(|(c, col)| match c.try_into() {
                    Ok(orientation) => Ok(Tile {
                        row,
                        col,
                        orientation,
                    }),
                    Err(err) => Err(err.into()),
                })
                .collect::<Result<Vec<Tile>, String>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, _>>()?;

    Ok(TileGrid {
        width: tile_vec_vec[0].len(),
        height: tile_vec_vec.len(),
        tiles: tile_vec_vec.into_iter().flatten().collect(),
    })
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let tile_grid = match parse_file_contents(&file_contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Error parsing tile grid: {err}");
            return;
        }
    };

    // println!("{tile_grid:?}");
    // let half_loop_length = part_one(&tile_grid);
    // println!("Part One -- Half Loop Length: {half_loop_length}");

    let inside_count = part_two(&tile_grid);
    println!("Part Two -- Inside Count: {inside_count}");
}
