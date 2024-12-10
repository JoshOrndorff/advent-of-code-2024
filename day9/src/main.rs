// 8800549860065 is too high for part 2.
// It solves the example input though.


/// A segment of consecutive blocks on disk.
/// Could represent a file or a chunk of free space.
struct DiskSegment {
    start: usize,
    length: usize,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");
    // let input = "12345";
    let starting_disk_map: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("digit should parse") as usize)
        .collect();

    // Populate the file records and free spacefrom the puzzle input
    let mut files = Vec::new();
    let mut free_space = Vec::new();
    let mut total_length = 0usize;

    for (i, &length) in starting_disk_map.iter().enumerate() {
        if i % 2 == 0 {
            files.push(DiskSegment {
                start: total_length,
                length,
            })
        } else {
            free_space.push(DiskSegment {
                start: total_length,
                length,
            })
        }
        total_length += length;
    }

    // Iterate the files from right to left
    for file_id in (0..files.len()).rev() {
        let file = &mut files[file_id];

        // Iterate the free space from left to right looking for a new home
        for free in free_space.iter_mut() {
            // Design thought. If we entirely fill a free space, should we remove the record?
            // Or is setting its length to zero enough?
            if free.length >= file.length {
                file.start = free.start;

                free.start += file.length;
                free.length -= file.length;

                break;
            }
        }
    }

    // Now calculate the checksum by iterating the files.
    // The files will be iterated in order of id (aka the order they originally appeared on disk)
    // NOT in the order they now appear on disk.
    // This is okay.
    let mut checksum = 0usize;
    for (id, DiskSegment { start, length }) in files.iter().enumerate() {
        // We could optimize by uses gauss's trick, but wtf.
        for i in *start..(start + length) {
            checksum += i * id;
        }
    }

    println!("{checksum}");
}
