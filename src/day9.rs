use std::collections::VecDeque;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Debug)]
struct File {
    id: usize,
    size: u32,
}

impl File {
    fn new(id: usize, size: u32) -> Self {
        Self { id, size }
    }
}

struct Segments {
    files: VecDeque<File>,
    empty_space: VecDeque<u32>,
}

fn sum_range(start: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    start * len + ((len - 1) * len / 2)
}

impl Segments {
    fn new(s: String) -> Self {
        let mut files = VecDeque::new();
        let mut empty_space = VecDeque::new();
        for i in (0..s.len()).step_by(2) {
            files.push_back(File::new(
                i / 2,
                s.chars().nth(i).unwrap().to_digit(10).unwrap(),
            ));
            if i + 1 < s.len() {
                empty_space.push_back(s.chars().nth(i + 1).unwrap().to_digit(10).unwrap());
            }
        }
        Self { files, empty_space }
    }

    fn compute_checksum(&mut self) -> anyhow::Result<usize> {
        let mut checksum = 0;
        let mut curr_segment = 0;
        let mut final_segments = String::new();
        while !self.files.is_empty() {
            let curr_file = self
                .files
                .pop_front()
                .ok_or(anyhow::anyhow!("No files left"))?;
            final_segments.push_str(
                curr_file
                    .id
                    .to_string()
                    .repeat(curr_file.size as usize)
                    .as_str(),
            );

            checksum += curr_file.id * sum_range(curr_segment, curr_file.size as usize);
            curr_segment += curr_file.size as usize;

            let next_empty_space = self
                .empty_space
                .pop_front()
                .ok_or(anyhow::anyhow!("No empty space left"))?;
            if next_empty_space == 0 {
                continue;
            }
            if self.files.is_empty() {
                break;
            }
            let last_file = self
                .files
                .pop_back()
                .ok_or(anyhow::anyhow!("No files left"))?;
            if last_file.size > next_empty_space {
                self.files
                    .push_front(File::new(last_file.id, next_empty_space));
                self.files
                    .push_back(File::new(last_file.id, last_file.size - next_empty_space));
                self.empty_space.push_back(next_empty_space);
                self.empty_space.push_front(0)
            } else {
                let size = last_file.size;
                self.files.push_front(last_file);
                self.empty_space.push_back(size);
                self.empty_space.push_front(next_empty_space - size);
            }
        }
        println!("{}", final_segments);

        Ok(checksum)
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let mut segments = Segments::new(s);
    segments.compute_checksum()
}
