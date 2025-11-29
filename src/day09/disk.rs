use crate::day09::block::Block;

pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn from_blocks(input: &str) -> Disk {
        let blocks = input
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let size = c.to_digit(10).unwrap() as usize;
                (0..size)
                    .map(|_| if i % 2 == 0 { Block::new_data_block(i / 2, 1) } else { Block::new_free_block(1) })
                    .collect::<Vec<Block>>()
            })
            .collect();
        Disk { blocks }
    }

    pub fn from_files(input: &str) -> Disk {
        let blocks = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 { Block::new_data_block(i / 2, size) } else { Block::new_free_block(size) }
            })
            .collect();
        Disk { blocks }
    }

    pub fn defrag(&self) -> Disk {
        let mut blocks = self.blocks.clone();

        for block_index in (1..blocks.len()).rev() {
            let block = blocks[block_index];
            if block.is_free() {
                continue;
            }

            if let Some(free_space_index) = blocks[..block_index].iter().position(|b| b.is_free() && b.size >= block.size) {
                let free_block = blocks[free_space_index];
                blocks[free_space_index] = block;
                blocks[block_index] = Block::new_free_block(block.size);
                if block.size < free_block.size {
                    blocks.insert(free_space_index + 1, Block::new_free_block(free_block.size - block.size));
                }
            }
        }

        Disk { blocks }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .flat_map(|block| (0..block.size).map(move |_| block))
            .enumerate()
            .map(|(index, block)| block.checksum(index))
            .sum()
    }

    pub fn print(&self) -> String {
        self.blocks.iter().map(|block| block.print()).collect::<Vec<String>>().join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_blocks() {
        let disk = Disk::from_blocks("12345");

        assert_eq!(disk.print(), "0..111....22222");
    }

    #[test]
    fn parses_example() {
        let disk = Disk::from_blocks("2333133121414131402");

        assert_eq!(disk.print(), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn defrags() {
        let disk = Disk::from_blocks("12345");

        assert_eq!(disk.defrag().print(), "022111222......");
    }

    #[test]
    fn defrags_example() {
        let disk = Disk::from_blocks("2333133121414131402");

        assert_eq!(disk.defrag().print(), "0099811188827773336446555566..............");
    }

    #[test]
    fn defrags_whole_file_example() {
        let disk = Disk::from_files("2333133121414131402");

        assert_eq!(disk.defrag().print(), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn calculates_checksum() {
        let disk = Disk::from_blocks("2333133121414131402");

        assert_eq!(disk.defrag().checksum(), 1928);
    }
}
