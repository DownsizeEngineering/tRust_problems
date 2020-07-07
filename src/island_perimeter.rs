

pub fn run() -> i32 {
 //    island_perimeter(vec![vec![0,1,0,0],
 // vec![1,1,1,0],
 // vec![0,1,0,0],
 // vec![1,1,0,0]])
 island_perimeter(vec![vec![0,1]])
}



pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashSet, HashMap};

    type Coord = (usize, usize);

    #[derive(Copy, Clone)]
    struct Land {
        west: bool,
        south: bool,
        north: bool,
        east: bool,
    }


    fn visit(coord: Coord, 
        grid: &Vec<Vec<i32>>,
        map: &mut HashMap<Coord, Land>, 
        coast: &mut Vec<Land>
    ) {
        let (x, y) = coord;
        let west = if x == 0 || grid[x - 1][y] == 0 {false} 
            else {true};
        let east = if x == grid.len() - 1 || grid[x + 1][y] == 0 {false}
            else {true};
        let north = if y == 0 || grid[x][y - 1] == 0 {false} 
            else {true};
        let south = if y == grid[0].len() - 1 || grid[x][y + 1] == 0 {false} 
            else {true};

        let land = Land {west, south, north, east};
        if west && south && north && east {/*landlocked*/} else {coast.push(land);}
        map.insert((x,y), land);

        if west && map.contains_key(&(x - 1, y)) == false {
            visit((x - 1, y), grid, map, coast);
        }
        if east && map.contains_key(&(x + 1, y)) == false {
            visit((x + 1, y), grid, map, coast);
        }
        if south && map.contains_key(&(x, y + 1)) == false {
            visit((x, y + 1), grid, map, coast);
        }
        if north && map.contains_key(&(x, y - 1)) == false {
            visit((x, y - 1), grid, map, coast);
        }
    }

    fn north_shore(grid: &Vec<Vec<i32>>) -> Coord {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return (i, j);
                }
            }
        }
        (0,0)
    }

    let mut map: HashMap<Coord, Land> = HashMap::new();
    let mut coast: Vec<Land> = Vec::new();

    visit(north_shore(&grid), &grid, &mut map, &mut coast);
    let mut perimiter = 0;
    for shore in coast {
        if shore.west == false { perimiter += 1; }
        if shore.south == false { perimiter += 1; }
        if shore.north == false { perimiter += 1; }
        if shore.east == false { perimiter += 1; }
    }

    perimiter
}

