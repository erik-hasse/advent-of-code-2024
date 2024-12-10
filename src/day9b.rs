use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Debug)]
struct File {
    location: usize,
    id: usize,
    size: u32,
}

#[derive(Debug)]
struct OpenSpace {
    location: usize,
    size: u32,
}

impl File {
    fn new(location: usize, id: usize, size: u32) -> Self {
        Self { location, id, size }
    }

    fn checksum(&self) -> usize {
        if self.size == 0 {
            return 0;
        }
        let size = self.size as usize;
        let range_sum = self.location * size + ((size - 1) * size / 2);
        self.id * range_sum
    }
}

struct Segments {
    files: Vec<File>,
    empty_space: Vec<OpenSpace>,
}

impl Segments {
    fn new(s: String) -> Self {
        let mut files = Vec::new();
        let mut empty_space = Vec::new();
        let mut curr_location = 0;
        for i in (0..s.len()).step_by(2) {
            let size = s.chars().nth(i).unwrap().to_digit(10).unwrap();
            files.push(File::new(curr_location, i / 2, size));
            curr_location += size as usize;
            if i + 1 < s.len() {
                let size = s.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                if size > 0 {
                    empty_space.push(OpenSpace {
                        location: curr_location,
                        size,
                    });
                    curr_location += size as usize;
                }
            }
        }
        Self { files, empty_space }
    }

    fn process_b(&mut self) {
        for file in self.files.iter_mut().rev() {
            let open_space = self.empty_space.iter_mut().find(|s| s.size >= file.size);
            match open_space {
                Some(space) => {
                    if space.location < file.location {
                        println!("Moving file {} to location {}", file.id, space.location);
                        file.location = space.location;
                        space.size -= file.size;
                        space.location += file.size as usize;
                    }
                }
                None => {
                    println!("No space for file {}", file.id);
                }
            }
        }
    }

    fn compute_checksum(&self) -> usize {
        self.files.iter().map(|f| f.checksum()).sum()
    }
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let mut segments = Segments::new(s);
    segments.process_b();

    Ok(segments.compute_checksum())
}
