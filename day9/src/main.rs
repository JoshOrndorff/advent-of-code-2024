// 24803636232561 is too high

#[derive(Debug)]
/// Allows iterating the non-empty block of a disk snapshot
/// from both the front and back
struct DoubleEndedBlockIterator {
    /// Sizes of all files in the snapshot.
    /// In the language of the original problem, this is the odd digits in the original sequence
    file_sizes: Vec<usize>,
    file_index_front: usize,
    /// Blocks consumed from the current file
    blocks_consumed_front: usize,
    file_index_back: usize,
    blocks_consumed_back: usize,
}

impl DoubleEndedBlockIterator {
    fn new(file_sizes: Vec<usize>) -> Self {
        // println!("Making a new de iterator. File sizes are {:?}", file_sizes);
        Self {
            file_index_front: 0,
            blocks_consumed_front: 0,
            file_index_back: file_sizes.len() - 1,
            blocks_consumed_back: 0,
            file_sizes,
        }
    }

    fn has_met_in_middle(&self) -> bool {
        self.file_index_front == self.file_index_back
            && self.blocks_consumed_front + self.blocks_consumed_back
                == self.file_sizes[self.file_index_front]
    }
}

impl Iterator for DoubleEndedBlockIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // First order of business is to see if we are done.
        if self.has_met_in_middle() {
            return None;
        }

        let return_value = self.file_index_front;
        self.blocks_consumed_front += 1;

        // If we're done with this file, move the pointer and reset for next time
        if self.blocks_consumed_front == self.file_sizes[self.file_index_front] {
            self.file_index_front += 1;
            self.blocks_consumed_front = 0;
        }

        Some(return_value)
    }
}

impl DoubleEndedIterator for DoubleEndedBlockIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        // First order of business is to see if we are done.
        if self.has_met_in_middle() {
            return None;
        }

        let return_value = self.file_index_back;
        self.blocks_consumed_back += 1;

        // If we're done with this file, move the pointer and reset for next time
        if self.blocks_consumed_back == self.file_sizes[self.file_index_back] {
            self.file_index_back -= 1;
            self.blocks_consumed_back = 0;
        }

        Some(return_value)
    }
}

struct FinishedDiskIterator {
    de_block_iter: DoubleEndedBlockIterator,
    pulling_from_front: bool,
    blocks_to_pull: Vec<usize>,
}

impl FinishedDiskIterator {
    fn new(starting_disk_map: Vec<usize>) -> Self {
        // Make a double ended block iterator for the starting disk by
        // extracting the file sizes from the odd positioned digits.
        let file_sizes = starting_disk_map
            .iter()
            .enumerate()
            .filter_map(|(i, &size)| if i % 2 == 0 { Some(size) } else { None })
            .collect();
        let de_block_iter = DoubleEndedBlockIterator::new(file_sizes);
        Self {
            de_block_iter,
            pulling_from_front: true,
            blocks_to_pull: starting_disk_map,
        }
    }
}

impl Iterator for FinishedDiskIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.blocks_to_pull[0] == 0 {
            self.blocks_to_pull.remove(0); //TODO prolly should be a vec dequeue or else reverse it an pop off the end.
            self.pulling_from_front = !self.pulling_from_front;

            if self.blocks_to_pull.is_empty() {
                println!("HIT THE SHORTCUTTTTTTTTTTTTTTTTTTTTTTTT");
                return None;
            }
        }

        if self.pulling_from_front {
            println!(
                "about to pull from front with {} remaining.",
                self.blocks_to_pull[0]
            );
        }
        self.blocks_to_pull[0] -= 1;

        if self.pulling_from_front {
            self.de_block_iter.next()
        } else {
            self.de_block_iter.next_back()
        }
    }
}
fn rmain() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");
    let input = "12345";
    let starting_disk_map = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("digit should parse") as usize)
        .collect();
    let finished_disk_iterator = FinishedDiskIterator::new(starting_disk_map);

    // for x in finished_disk_iterator {
    //     print!("{x}");
    // }

    let checksum = finished_disk_iterator
        .enumerate()
        .map(|(i, id)| {
            let product = i * id;
            println!("{i} times {id} is {product}");
            product
        })
        .sum::<usize>();
    println!("{checksum}");
}

fn main() {
    let mut dei = DoubleEndedBlockIterator::new(vec![1, 3, 5]);
    dei.next();
    while let Some(x) = dei.next_back() {
        println!("{:?}", dei);
        println!("{x}\n\n");
    }
}