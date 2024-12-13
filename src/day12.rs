use std::collections::HashSet;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

type Coord = (isize, isize);

struct Region {
    plots: HashSet<Coord>,
    perimeter: usize,
}

impl Region {
    fn cost(&self) -> usize {
        let cost = self.plots.len() * self.perimeter;
        println!("{} * {} = {}", self.plots.len(), self.perimeter, cost);
        cost
    }

    fn count_corners(&self) -> usize {
        let mut corners = 0;
        for p in &self.plots {
            for x in [-1, 1] {
                for y in [-1, 1] {
                    let dx = (p.0 + x, p.1);
                    let dy = (p.0, p.1 + y);
                    let dboth = (p.0 + x, p.1 + y);
                    if !self.plots.contains(&dx) && !self.plots.contains(&dy) {
                        // Interior corner
                        corners += 1
                    }
                    if self.plots.contains(&dx)
                        && self.plots.contains(&dy)
                        && !self.plots.contains(&dboth)
                    {
                        // Exterior corner
                        corners += 1
                    }
                }
            }
        }
        corners
    }

    fn side_cost(&self) -> usize {
        let corners = self.count_corners();
        let cost = self.plots.len() * corners;
        println!("{} * {} = {}", self.plots.len(), corners, cost);
        cost
    }
}

struct Map {
    land: Vec<Vec<char>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let land: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self { land }
    }

    fn build_region(&self, coord: Coord) -> Region {
        let mut to_visit = vec![coord];
        let mut plots = HashSet::new();
        plots.insert(coord);
        let mut visited = vec![];
        let mut perimeter = 0;
        while let Some(point) = to_visit.pop() {
            visited.push(point);
            let curr = self.get(point).unwrap();
            for dir in DIRECTIONS {
                let spot = (point.0 + dir.0, point.1 + dir.1);
                if !visited.contains(&spot) && !to_visit.contains(&spot) {
                    if let Some(val) = self.get(spot) {
                        if val == curr {
                            to_visit.push(spot);
                            plots.insert(spot);
                        } else {
                            perimeter += 1
                        }
                    } else {
                        perimeter += 1
                    }
                }
            }
        }
        println!("{:?}", plots);
        Region { plots, perimeter }
    }

    fn get(&self, coord: Coord) -> Option<char> {
        if coord.0 < 0 || coord.1 < 0 {
            return None;
        }
        self.land
            .get(coord.1 as usize)?
            .get(coord.0 as usize)
            .copied()
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::new();
        for y in 0..self.land.len() {
            for x in 0..self.land.first().unwrap().len() {
                let p = (x as isize, y as isize);
                if !regions.iter().any(|r| r.plots.contains(&p)) {
                    regions.push(self.build_region(p))
                }
            }
        }
        regions
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let map = Map::new(&s);
    let regions = map.find_regions();

    Ok(regions.into_iter().map(|r| r.cost()).sum())
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let map = Map::new(&s);
    let regions = map.find_regions();

    Ok(regions.into_iter().map(|r| r.side_cost()).sum())
}
