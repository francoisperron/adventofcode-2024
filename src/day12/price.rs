use std::iter::Sum;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Price {
    pub total: usize,
    pub total_with_discount: usize,
}

impl Price {
    pub fn new(total: usize, total_with_discount: usize) -> Price {
        Price { total, total_with_discount }
    }
}

impl Sum for Price {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (total, total_with_discount) = iter.fold((0, 0), |(total, total_with_discount), price| (total + price.total, total_with_discount + price.total_with_discount));
        Price { total, total_with_discount }
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::price::Price;

    #[test]
    fn sums_prices() {
        let prices = vec![Price::new(1, 10), Price::new(2, 20)];
        let sum: Price = prices.into_iter().sum();

        assert_eq!(sum, Price::new(3, 30));
    }
}
