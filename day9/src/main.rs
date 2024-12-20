use aoc24_util::fetch_input;

enum Block {
    File(usize),
    Free,
}

impl Block {
    fn file_id(&self) -> usize {
        match self {
            Self::File(v) => *v,
            _ => 0,
        }
    }

    fn is_file(&self) -> bool {
        matches!(self, Block::File(_))
    }
}

fn blocks(disk_map: &[usize]) -> impl DoubleEndedIterator<Item = Block> + use<'_> {
    disk_map
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (0..*v).map(move |_| {
                if i % 2 == 0 {
                    Block::File(i / 2)
                } else {
                    Block::Free
                }
            })
        })
        .flatten()
}

fn resolve(disk_map: &[usize]) -> usize {
    let map_size: usize = disk_map.iter().sum();
    let mut rightmost_files = blocks(disk_map)
        .rev()
        .enumerate()
        .map(|(i, v)| (map_size - i - 1, v))
        .filter(|(_, v)| v.is_file())
        .peekable();
    let blocks = blocks(disk_map).enumerate();

    let mut checksum = 0;

    for (block_index, block) in blocks {
        if let Some((index, _)) = rightmost_files.peek() {
            if *index < block_index {
                break;
            }
        }

        if block.is_file() {
            print!("{}", block.file_id());

            checksum += block_index * block.file_id();
        } else {
            let (_, moved_file) = match rightmost_files.next() {
                Some(v) => v,
                _ => break,
            };

            print!("{}", moved_file.file_id());

            checksum += block_index * moved_file.file_id();
        }
    }

    println!();

    checksum
}

fn main() {
    let text = fetch_input(9);

    let disk_map: Vec<usize> = text
        .trim()
        .chars()
        .map(|v| v.to_string().parse().unwrap())
        .collect();

    println!("{}", resolve(&disk_map));
}
