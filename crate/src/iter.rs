pub struct ImageIterator {
    width: u32,
    height: u32,
    item: u32,
}

impl ImageIterator {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            item: 0_u32,
        }
    }
    pub fn with_dimension(dimension: &(u32, u32)) -> Self {
        Self {
            width: dimension.0,
            height: dimension.1,
            item: 0_u32,
        }
    }
}

impl Iterator for ImageIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.item;
        self.item += 1;
        if n < (self.width * self.height) {
            Some((n / self.height, n % self.height))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::iter::ImageIterator;

    const WIDTH: u32 = 8;
    const HEIGHT: u32 = 6;
    const DATA: [(u32, u32); 48] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (2, 0),
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (3, 4),
        (3, 5),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
        (5, 0),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (6, 0),
        (6, 1),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
        (7, 0),
        (7, 1),
        (7, 2),
        (7, 3),
        (7, 4),
        (7, 5),
    ];

    #[test]
    fn test_image_iter_with_dimension() {
        let mut item = DATA.iter();
        for i in ImageIterator::with_dimension(&(WIDTH, HEIGHT)) {
            assert_eq!(i, *item.next().unwrap());
        }
    }

    #[test]
    fn test_image_iter_new() {
        let mut item = DATA.iter();
        for (x, y) in ImageIterator::new(WIDTH, HEIGHT) {
            assert_eq!((x, y), *item.next().unwrap());
        }
    }

    #[test]
    fn test_n2_iter() {
        let mut item = DATA.iter();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                assert_eq!((x, y), *item.next().unwrap());
            }
        }
    }
}
