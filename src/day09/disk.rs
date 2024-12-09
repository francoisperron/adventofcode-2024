pub struct Disk {
    blocks: Vec<(String, usize)>,
}

impl Disk {
    pub fn from_blocks(input: &str) -> Disk {
        let blocks = input
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let size = c.to_digit(10).unwrap() as usize;
                let value = if i % 2 == 0 { (i / 2).to_string() } else { ".".to_string() };
                (0..size).map(|_| (value.to_string(), 1)).collect::<Vec<(String, usize)>>()
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
                let value = if i % 2 == 0 { (i / 2).to_string() } else { ".".to_string() };
                (value, size)
            })
            .collect();
        Disk { blocks }
    }

    pub fn defrag(&self) -> Disk {
        let mut blocks = self.blocks.clone();
        let mut block_index = blocks.len();

        while block_index > 1 {
            block_index -= 1;
            let (id, size) = blocks[block_index].clone();
            if id == "." {
                continue;
            }

            if let Some(free_space_index) = blocks[..block_index].iter().position(|(id_free, space_free)| id_free == "." && *space_free >= size) {
                let free_space = blocks[free_space_index].1;
                blocks[free_space_index] = (id.clone(), size);
                blocks[block_index] = (".".to_string(), size);

                if size < free_space {
                    blocks.insert(free_space_index + 1, (".".to_string(), free_space - size));
                }
            }
        }

        Disk { blocks }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .flat_map(|(id, size)| (0..*size).map(move |_| id))
            .enumerate()
            .map(|(index, id)| if id == "." { 0 } else { index * id.parse::<usize>().unwrap() })
            .sum()
    }

    pub fn print(&self) -> String {
        self.blocks
            .iter()
            .flat_map(|(v, size)| vec![v.to_string(); *size])
            .collect::<Vec<String>>()
            .join("")
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
