use crate::day12::price::Price;
use crate::toolbox::{Direction, Grid, Position};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    pub fn new(area: usize, perimeter: usize, sides: usize) -> Region {
        Region { area, perimeter, sides }
    }

    pub fn add(&mut self, position: &Position, plant: &char, grid: &Grid) {
        self.area += 1;
        self.perimeter += position.around_me_4().filter(|a| grid.element_at(a) != plant).count();
        self.sides += Direction::all()
            .iter()
            .filter(|direction| {
                let (corner_1, corner_2, corner_3) = position.corner(direction);
                (grid.element_at(&corner_1) != plant || grid.element_at(&corner_2) == plant) && grid.element_at(&corner_3) != plant
            })
            .count();
    }

    pub fn price(&self) -> Price {
        let total = self.area * self.perimeter;
        let total_with_discount = self.area * self.sides;
        Price::new(total, total_with_discount)
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::region::Region;
    use crate::toolbox::{Grid, Position};
    use std::sync::LazyLock;

    static GRID: LazyLock<Grid> = LazyLock::new(|| Grid::from("AB\nAC"));

    #[test]
    fn add_increments_area_by_one() {
        let mut region = Region::default();

        region.add(&Position::new(0, 0), &'A', &GRID);
        assert_eq!(region.area, 1);

        region.add(&Position::new(0, 1), &'A', &GRID);
        assert_eq!(region.area, 2);
    }

    #[test]
    fn add_increments_perimeter_by_different_plants_around_count() {
        let mut region = Region::default();

        region.add(&Position::new(0, 0), &'A', &GRID);
        assert_eq!(region.perimeter, 3);

        region.add(&Position::new(0, 1), &'A', &GRID);
        assert_eq!(region.perimeter, 6);
    }

    #[test]
    fn add_increments_sides_by_corners_count() {
        let mut region = Region::default();

        region.add(&Position::new(0, 0), &'A', &GRID);
        assert_eq!(region.sides, 2);

        region.add(&Position::new(0, 1), &'A', &GRID);
        assert_eq!(region.sides, 4);
    }

    #[test]
    fn calculates_price() {
        let region = Region::new(5, 10, 20);
        let price = region.price();

        assert_eq!(price.total, 5 * 10);
        assert_eq!(price.total_with_discount, 5 * 20);
    }
}
