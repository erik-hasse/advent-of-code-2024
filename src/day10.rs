use std::collections::HashSet;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

type Map = Vec<Vec<u8>>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_coords(map: &Map, x: isize, y: isize) -> Option<u8> {
    if x < 0 || y < 0 {
        return None;
    }
    map.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

fn find_summits_from(map: &Map, x: isize, y: isize) -> HashSet<(isize, isize)> {
    let current = get_coords(map, x, y);
    match current {
        None => HashSet::new(),
        Some(9) => HashSet::from([(x, y)]),
        Some(current) => {
            let mut summits = HashSet::new();
            let directions: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions {
                let (nx, ny) = (x + dx, y + dy);
                if let Some(next) = get_coords(map, nx, ny) {
                    if next == current + 1 {
                        summits.extend(find_summits_from(map, nx, ny));
                    }
                }
            }
            summits
        }
    }
}

fn find_paths_from(map: &Map, x: isize, y: isize) -> Vec<Vec<(isize, isize)>> {
    let current = get_coords(map, x, y);
    match current {
        None => Vec::new(),
        Some(9) => vec![vec![(x, y)]],
        Some(current) => {
            let mut paths = Vec::new();
            let directions: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in directions {
                let (nx, ny) = (x + dx, y + dy);
                if let Some(next) = get_coords(map, nx, ny) {
                    if next == current + 1 {
                        let mut next_paths = find_paths_from(map, nx, ny);
                        for path in next_paths.iter_mut() {
                            path.push((x, y));
                        }
                        paths.extend(next_paths);
                    }
                }
            }
            paths
        }
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let input = read_to_string(input)?;
    let map = parse_map(&input);
    let mut score = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if get_coords(&map, x as isize, y as isize) == Some(0) {
                score += find_summits_from(&map, x as isize, y as isize).len();
            }
        }
    }
    Ok(score)
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let input = read_to_string(input)?;
    let map = parse_map(&input);
    let mut score = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if get_coords(&map, x as isize, y as isize) == Some(0) {
                score += find_paths_from(&map, x as isize, y as isize).len();
            }
        }
    }
    Ok(score)
}
