use std::collections::HashSet;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

struct Grid {
    letters: String,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(letters: String) -> Self {
        let width = letters.find("\n").unwrap_or(letters.len());
        let height = letters.split("\n").count();
        let letters = letters.replace("\n", "");

        Self {
            letters,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.letters.chars().nth(index)
    }

    fn to_point(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn check_str(&self, curr: (usize, usize), direction: (i8, i8), rest: &str) -> bool {
        let spot = self.get(curr);
        if rest.is_empty() || (rest.len() == 1 && spot == rest.chars().nth(0)) {
            return true;
        }
        if spot != Some(rest.chars().nth(0).unwrap()) {
            return false;
        };
        let next = (
            curr.0 as isize + direction.0 as isize,
            curr.1 as isize + direction.1 as isize,
        );
        if next.0 < 0
            || next.1 < 0
            || next.0 >= self.width as isize
            || next.1 >= self.height as isize
        {
            return false;
        }
        self.check_str((next.0 as usize, next.1 as usize), direction, &rest[1..])
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<u32> {
    let contents = read_to_string(input)?;
    let grid = Grid::new(contents);
    let mut count = 0;

    for i in 0..grid.letters.len() {
        println!("{:?}", grid.to_point(i));
        for xi in -1..=1 {
            for yi in -1..=1 {
                if xi == 0 && yi == 0 {
                    continue;
                }
                if grid.check_str(grid.to_point(i), (xi, yi), "XMAS") {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

pub fn part_b(input: &Path) -> anyhow::Result<u32> {
    let contents = read_to_string(input)?;
    let grid = Grid::new(contents);
    let mut count = 0;
    let goal = HashSet::from([Some('M'), Some('S')]);

    for i in 0..grid.letters.len() {
        let (x, y) = grid.to_point(i);

        println!("{:?}", (x, y));
        if x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1 {
            continue;
        }
        if grid.get((x, y)) != Some('A') {
            continue;
        }
        let ul_dr_set = HashSet::from([grid.get((x - 1, y - 1)), grid.get((x + 1, y + 1))]);
        let ur_dl_set = HashSet::from([grid.get((x + 1, y - 1)), grid.get((x - 1, y + 1))]);
        if ul_dr_set == goal && ur_dl_set == goal {
            count += 1;
        }
    }
    Ok(count)
}
