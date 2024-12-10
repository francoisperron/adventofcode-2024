#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: BlockId,
    pub size: usize,
}

impl Block {
    pub fn new_data_block(id: usize, size: usize) -> Block {
        Block { id: BlockId::Data(id), size }
    }

    pub fn new_free_block(size: usize) -> Block {
        Block { id: BlockId::Free, size }
    }

    pub fn is_free(&self) -> bool {
        self.id == BlockId::Free
    }

    pub fn checksum(&self, index: usize) -> usize {
        match self.id {
            BlockId::Free => 0,
            BlockId::Data(id) => id * index,
        }
    }

    pub fn print(&self) -> String {
        let value = match self.id {
            BlockId::Free => ".",
            BlockId::Data(id) => &id.to_string(),
        };
        vec![value; self.size].join("")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BlockId {
    Data(usize),
    Free,
}
