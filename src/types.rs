use std::fmt;

pub(crate) type Series = Vec<Vec<f32>>;
pub(crate) type Labels = Vec<String>;

#[derive(Clone, Copy, Default)]
pub(crate) struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// Used for cleaner SVG path formatting
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            min: Point::new(x1, y1),
            max: Point::new(x2, y2),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[derive(Clone)]
pub(crate) struct TextData {
    pub x: f32,
    pub y: f32,
    pub anchor: &'static str,
    pub baseline: &'static str,
}

impl Default for TextData {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            anchor: "start",
            baseline: "text-bottom",
        }
    }
}

pub(crate) struct MultiZip<T>(pub Vec<T>);

//pub(crate) struct MultiZip<'a, T> {
//    vec: &'a Vec<T>
//}
//
//pub(crate) struct MultiZipIter<'a, T> {
//    values: &'a Vec<T>,
//    index: usize
//}
//
//impl<'a, T> IntoIterator for MultiZip<'a, T>
//where
//    T: Iterator
//{
//    type Item = Vec<T::Item>;
//    type IntoIter = MultiZipIter<'a, T>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        MultiZipIter {
//            values: self.vec,
//            index: 0
//        }
//    }
//}

//impl<'a, T> Iterator for MultiZipIter<'a, T>
impl<T> Iterator for MultiZip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}
