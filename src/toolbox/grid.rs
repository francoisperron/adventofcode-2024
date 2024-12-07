use crate::toolbox::position::Position;
use std::collections::HashMap;

pub struct Grid {
    pub elements: HashMap<Position, char>,
    pub max_x: isize,
    pub max_y: isize,
}

impl Grid {
    pub fn from(input: &str) -> Grid {
        let elements = input.lines().enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, element)| (Position::new(x as isize, y as isize), element)))
            .collect();
        let max_y = input.lines().count() as isize;
        let max_x = input.lines().next().unwrap().chars().count() as isize;
        Grid { elements, max_x, max_y }
    }

    pub fn element_at(&self, position: &Position) -> &char {
        self.elements.get(position).unwrap_or(&' ')
    }
    
    pub fn position_of(&self, element: &char) -> Option<Position> {
        self.elements.iter().find(|(_, v)| *v == element).map(|(position, _)| *position)
    }

    pub fn is_inbound(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.max_x && position.y >= 0 && position.y < self.max_y
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::toolbox::grid::Grid;
    use crate::toolbox::position::Position;

    #[test]
    fn creates_grid() {
        let grid = Grid::from("ab\ncd");

        assert_eq!(grid.elements, HashMap::from([
            (Position::new(0, 0), 'a'),
            (Position::new(1, 0), 'b'),
            (Position::new(0, 1), 'c'),
            (Position::new(1, 1), 'd')
        ]));         
    }
    
    #[test]
    fn gets_element_at_position() {
        let grid = Grid::from("ab\ncd");
        
        assert_eq!(grid.element_at(&Position::new(0, 0)), &'a');
        assert_eq!(grid.element_at(&Position::new(1, 0)), &'b');
        assert_eq!(grid.element_at(&Position::new(0, 1)), &'c');
        assert_eq!(grid.element_at(&Position::new(1, 1)), &'d');
    }
    
    #[test]
    fn gets_empty_when_element_not_found() {
        let grid = Grid::from("ab\ncd");
        
        assert_eq!(grid.element_at(&Position::new(2, 2)), &' ');
    }
    
    #[test]
    fn determines_if_position_is_inbound() {
        let grid = Grid::from("ab\ncd");
        
        assert!(grid.is_inbound(&Position::new(0, 0)));
        assert!(grid.is_inbound(&Position::new(1, 0)));
        assert!(grid.is_inbound(&Position::new(0, 1)));
        assert!(grid.is_inbound(&Position::new(1, 1)));
        
        assert!(!grid.is_inbound(&Position::new(-1, 0)));
        assert!(!grid.is_inbound(&Position::new(0, -1)));
        assert!(!grid.is_inbound(&Position::new(2, 0)));
        assert!(!grid.is_inbound(&Position::new(0, 2)));
    }
}