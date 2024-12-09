use itertools::repeat_n;
use winnow::{combinator::repeat, token::any, PResult, Parser as _};

use crate::days::Day;

pub struct Day09;

/// The map for a disk, comprised of alternating file sizes and empty spaces sizes
#[derive(Debug, Clone)]
pub struct DiskMap(Vec<u8>);

/// An iterator for efficient fragmentation in part 1
pub struct MapIterator {
    /// A copy of the original map
    map: Vec<u8>,
    /// An index to iterate over the files and empty spaces from the map
    pos_head: usize,
    /// An index pointing to the last unmoved file in the map
    pos_tail: usize,
    /// Remaining sectors in the head file or empty space
    remaining_head: u8,
}

impl MapIterator {
    fn is_file(&self, pos: usize) -> bool {
        pos % 2 == 0
    }
}

impl Iterator for MapIterator {
    type Item = usize; // block ID

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.is_file(self.pos_head),
            self.remaining_head,
            self.map[self.pos_tail],
        ) {
            (true, 0, _) => unreachable!("empty file, should not happen"),
            (true, 1.., _) => {
                // iterating over a file which we must return as-is (not moved)
                if self.pos_head > self.pos_tail + 1 {
                    return None;
                }
                // mark the sector as visited
                self.remaining_head -= 1;
                if self.remaining_head == 0 {
                    // all the sectors of this file will have been processed
                    // move to next empty space
                    self.pos_head += 1;
                    self.remaining_head = self.map[self.pos_head]; // empty space size
                }
                Some(self.pos_head / 2) // file ID
            }
            (false, 0, _) => {
                // no empty space, means we go the next file directly
                if self.pos_head > self.pos_tail {
                    return None;
                }
                self.pos_head += 1; // now pointing to a file
                self.remaining_head = self.map[self.pos_head] - 1; // we are yielding the first item directly
                if self.remaining_head == 0 {
                    // there was only 1 item to yield in the next block, so we go directly to the next empty space
                    self.pos_head += 1; // now pointing to an empty space
                    self.remaining_head = self.map[self.pos_head]; // size of the empty space
                } else if self.pos_head == self.pos_tail {
                    // we are moving into the same file as the tail, so we can just finish iteration with the unmoved
                    // sectors and then we'll be done
                    self.remaining_head = self.map[self.pos_tail] - 1; // we are yielding one item directly
                }
                Some(self.pos_head / 2) // file ID
            }
            (false, 1.., 0) => unreachable!("empty file, should not happen"),
            (false, 1.., 1..) => {
                // currently on an empty sector
                // copy a part of the file at the tail to this empty slot
                if self.pos_head > self.pos_tail {
                    return None;
                }
                self.map[self.pos_tail] -= 1; // register that the tail file is one sector shorter
                self.remaining_head -= 1; // advance in the empty slot
                if self.remaining_head == 0 {
                    // empty slot has been filled, move to next file
                    self.pos_head += 1; // now points to a file
                    self.remaining_head = self.map[self.pos_head]; // next file size
                }
                let id = self.pos_tail / 2; // value to return (tail file ID)
                if self.map[self.pos_tail] == 0 {
                    // tail file has been copied entirely
                    self.pos_tail -= 2; // move to previous tail file
                }
                if self.pos_head == self.pos_tail {
                    // we are moving into the same file as the tail, so we can just finish iteration with the
                    // unmoved parts and then we'll be done
                    self.remaining_head = self.map[self.pos_tail];
                }
                Some(id) // yield tail file ID
            }
        }
    }
}

impl IntoIterator for DiskMap {
    type Item = usize;

    type IntoIter = MapIterator;

    fn into_iter(self) -> Self::IntoIter {
        let first = *self.0.first().unwrap();
        let len = self.0.len();
        assert!(len % 2 == 1); // disk map should end with a file
        Self::IntoIter {
            map: self.0,
            pos_head: 0,
            pos_tail: len - 1,
            remaining_head: first,
        }
    }
}

impl Day for Day09 {
    type Input = DiskMap;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let chars: Vec<char> = repeat(1.., any).parse_next(input)?;
        Ok(DiskMap(
            chars
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        ))
    }

    type Output1 = usize;

    /// Part 1 took 136.8us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, id)| i * id)
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 197.7ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // create the actual sectors list for the map
        // `None` means the sector is empty, `Some(id)` means it contains a part of file ID
        let mut out: Vec<_> = input
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                if i % 2 == 0 {
                    repeat_n(Some(i / 2), *v as usize)
                } else {
                    repeat_n(None, *v as usize)
                }
            })
            .collect();
        let len = out.len();
        let mut last_id = usize::MAX;
        let mut i = len - 1; // iterate over the disk sectors starting from the end
        while i > 0 {
            // skip until we encounter a non-empty sector
            if out[i].is_none() {
                i -= 1;
                continue;
            }
            let id = out[i].unwrap();
            if id >= last_id {
                // this file was already moved, skip it
                i -= 1;
                continue;
            }
            last_id = id; // register last processed file ID to make sure we don't move a file twice

            // we know the file size from the original disk map
            let mut file_size = input.0[id * 2] as usize;
            // try to find a suitable hole
            let mut j = 0;
            while j < len {
                // skip full sectors
                if out[j].is_some() {
                    j += 1;
                    continue;
                }
                // if we didn't find an empty sector before reaching the position of the file to move, then we can't
                // move it
                if j > i.saturating_sub(file_size) {
                    // no suitable hole size
                    break;
                }
                // check how big the hole is
                let hole_size = out.iter().skip(j).take_while(|v| v.is_none()).count();
                if hole_size >= file_size {
                    // the hole is large enough, so we move the file parts by swapping with the empty sectors
                    while file_size > 0 {
                        out.swap(j + file_size - 1, i - file_size + 1);
                        file_size -= 1;
                    }
                    break; // file was moved
                } else {
                    // the hole is not large enough, let's keep looking
                    j += hole_size;
                }
            }
            // check the next file in descending order
            i = i.saturating_sub(file_size); // avoid underflow
        }
        // checksum
        out.into_iter()
            .enumerate()
            .map(|(i, id)| i * id.unwrap_or_default())
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let parsed = Day09::parser(&mut INPUT).unwrap();
        assert_eq!(Day09::part_1(&parsed), 1928);
    }

    #[test]
    fn test_part2() {
        let parsed = Day09::parser(&mut INPUT).unwrap();
        assert_eq!(Day09::part_2(&parsed), 2858);
    }
}
