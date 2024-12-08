pub mod mylib {
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Polyomino {
        blocks: HashSet<(usize, usize)>,
        height: usize,
        width: usize,
    }

    impl Polyomino {
        pub fn from_chars(chars: Vec<Vec<char>>) -> Self {
            assert!(
                chars.iter().flatten().any(|&ch| ch == '#' || ch == '.'),
                "`chars` must contain at least one '.' or '#'"
            );

            let mut blocks = HashSet::new();
            let height = chars.len();
            let width = chars.first().map_or(0, |line| line.len());

            assert!(
                chars.iter().all(|line| line.len() == width),
                "All rows must have the same length"
            );

            for (x, line) in chars.iter().enumerate() {
                for (y, &ch) in line.iter().enumerate() {
                    if ch == '#' {
                        blocks.insert((x, y));
                    }
                }
            }
            Polyomino {
                blocks,
                height,
                width,
            }
        }

        pub fn from_blocks(blocks: HashSet<(usize, usize)>) -> Self {
            assert!(
                !blocks.is_empty(),
                "`blocks` must contain at least one element"
            );

            let mut max_x = 0;
            let mut max_y = 0;

            for &(x, y) in &blocks {
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }

            let height = max_x + 1;
            let width = max_y + 1;

            Polyomino {
                blocks,
                height,
                width,
            }
        }

        pub fn empty(height: usize, width: usize) -> Self {
            Polyomino {
                blocks: HashSet::new(),
                height,
                width,
            }
        }

        pub fn blocks(&self) -> &HashSet<(usize, usize)> {
            &self.blocks
        }

        pub fn height(&self) -> usize {
            self.height
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn is_filled(&self) -> bool {
            self.blocks.len() == self.height * self.width
        }

        pub fn matches(&self, x: usize, y: usize, other: &Self) -> bool {
            for i in 0..other.height {
                for j in 0..other.width {
                    let new_x = x + i;
                    let new_y = y + j;
                    if new_x >= self.height || new_y >= self.width {
                        return false;
                    }
                    let self_has_block = self.blocks.contains(&(new_x, new_y));
                    let other_has_block = other.blocks.contains(&(i, j));
                    if self_has_block != other_has_block {
                        return false;
                    }
                }
            }
            true
        }

        pub fn collide(&self, x: usize, y: usize, other: &Self) -> bool {
            for &(ox, oy) in &other.blocks {
                let new_x = x + ox;
                let new_y = y + oy;
                if new_x < self.height
                    && new_y < self.width
                    && self.blocks.contains(&(new_x, new_y))
                {
                    return true;
                }
            }
            false
        }

        pub fn put(&mut self, x: usize, y: usize, other: &Self) {
            for &(ox, oy) in &other.blocks {
                let new_x = x + ox;
                let new_y = y + oy;
                if new_x < self.height && new_y < self.width {
                    self.blocks.insert((new_x, new_y));
                }
            }
        }

        pub fn fit(&mut self) {
            if self.blocks.is_empty() {
                self.height = 0;
                self.width = 0;
                return;
            }

            let mut min_x = usize::MAX;
            let mut max_x = 0;
            let mut min_y = usize::MAX;
            let mut max_y = 0;

            for &(x, y) in &self.blocks {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }

            let mut trimmed_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                trimmed_blocks.insert((x - min_x, y - min_y));
            }

            self.blocks = trimmed_blocks;
            self.height = max_x - min_x + 1;
            self.width = max_y - min_y + 1;
        }

        pub fn rotate_right(&mut self) {
            let mut new_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                let new_x = y;
                let new_y = self.height - 1 - x;
                new_blocks.insert((new_x, new_y));
            }
            std::mem::swap(&mut self.height, &mut self.width);
            self.blocks = new_blocks;
        }

        pub fn rotate_left(&mut self) {
            let mut new_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                let new_x = self.width - 1 - y;
                let new_y = x;
                new_blocks.insert((new_x, new_y));
            }
            std::mem::swap(&mut self.height, &mut self.width);
            self.blocks = new_blocks;
        }

        pub fn rotate_180(&mut self) {
            let mut new_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                let new_x = self.height - 1 - x;
                let new_y = self.width - 1 - y;
                new_blocks.insert((new_x, new_y));
            }
            self.blocks = new_blocks;
        }

        pub fn flip_horizontal(&mut self) {
            let mut new_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                let new_y = self.width - 1 - y;
                new_blocks.insert((x, new_y));
            }
            self.blocks = new_blocks;
        }

        pub fn flip_vertical(&mut self) {
            let mut new_blocks = HashSet::new();
            for &(x, y) in &self.blocks {
                let new_x = self.height - 1 - x;
                new_blocks.insert((new_x, y));
            }
            self.blocks = new_blocks;
        }
    }

    impl std::fmt::Display for Polyomino {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut result = vec![vec!['.'; self.width]; self.height];
            for &(x, y) in &self.blocks {
                result[x][y] = '#';
            }
            let string_representation = result
                .into_iter()
                .map(|line| line.into_iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");
            write!(f, "{}", string_representation)
        }
    }
}
