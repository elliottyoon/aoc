use std::ops::{Add, Index, IndexMut};

type Input<'a> = (usize, usize);

struct Grid<T> {
    rows: i32,
    cols: i32,
    inner: Vec<T>,
}

impl Grid<u8> {
    #[inline]
    fn parse(input: &str) -> Self {
        let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let rows = lines.len() as i32;
        let cols = lines[0].len() as i32;

        let mut inner = Vec::with_capacity((rows * cols) as usize);
        lines
            .iter()
            .for_each(|slice| inner.extend_from_slice(slice));

        Self { rows, cols, inner }
    }
}

impl<T> Grid<T> {
    #[inline]
    fn same_dimension_with<P: Copy>(&self, value: P) -> Grid<P> {
        Grid {
            rows: self.rows,
            cols: self.cols,
            inner: vec![value; (self.rows * self.cols) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.cols && point.y >= 0 && point.y < self.rows
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.inner[(self.cols * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.inner[(self.cols * index.y + index.x) as usize]
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut part1 = 0;
    let mut part2 = 0;

    let mut todo = Vec::new();
    let mut edges = Vec::new();
    let mut seen = grid.same_dimension_with(false);

    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            let plant_type = grid[point];
            let check = |point| grid.contains(point) && grid[point] == plant_type;

            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            todo.push(point);
            seen[point] = true;

            while area < todo.len() {
                let point = todo[area];
                area += 1;

                for direction in [UP, DOWN, LEFT, RIGHT] {
                    let next = point + direction;
                    // Bounds check
                    if check(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        edges.push((point, direction));
                        perimeter += 1;
                    }
                }
            }

            for &(point, direction) in &edges {
                let right = direction.clockwise();
                let left = direction.counter_clockwise();
                sides += (!check(point + left) || check(point + left + direction)) as usize;
                sides += (!check(point + right) || check(point + right + direction)) as usize;
            }

            todo.clear();
            edges.clear();

            part1 += area * perimeter;
            part2 += area * (sides / 2);
        }
    }

    (part1, part2)
}

pub fn part1(input: &Input) -> usize {
    input.0
}
pub fn part2(input: &Input) -> usize {
    input.1
}
