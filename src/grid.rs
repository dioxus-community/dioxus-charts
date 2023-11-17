use log::debug;

use crate::types::*;
use crate::utils::magnitude;

const LABEL_OFFSET: f32 = 6.0;
const TICK_SIZE: f32 = 10.0;

#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
pub(crate) struct Axis {
    view: Rect,
    step_len: f32,
    steps: i32,
    world_start: f32,
    world: f32,
    grid_ticks: bool,
    label_interpolation: Option<fn(f32) -> String>,
    label_size: i32,
    direction: Direction,
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            view: Rect::default(),
            step_len: 0.0,
            steps: 0,
            world_start: 0.0,
            world: 0.0,
            grid_ticks: false,
            label_interpolation: None,
            label_size: 60,
            direction: Direction::Horizontal,
        }
    }
}

impl Axis {
    pub fn builder<'a>() -> AxisBuilder<'a> {
        AxisBuilder::default()
    }

    pub fn world_to_view(&self, v: f32, start_offset: f32) -> f32 {
        if self.world > 0.0 {
            match self.direction {
                Direction::Vertical => v / self.world * self.view.width() + self.view.min.x,
                Direction::Horizontal => {
                    let c = (v - start_offset) / self.world * self.view.height() + self.view.min.y;
                    self.view.min.y - c + self.view.max.y
                }
            }
        } else {
            0.0
        }
    }

    pub fn step_to_world(&self, v: f32) -> f32 {
        self.world / (self.steps as f32 - 1.0) * v
    }

    pub fn lines(&self) -> Vec<Rect> {
        let mut lines = Vec::<Rect>::new();

        for i in 0..self.steps {
            let w = self.step_to_world(i as f32);
            let v = self.world_to_view(w, 0.0);

            match self.direction {
                Direction::Vertical => {
                    let end = if self.grid_ticks && i != 0 {
                        self.view.max.y - TICK_SIZE
                    } else {
                        self.view.min.y
                    };

                    lines.push(Rect::new(v, self.view.max.y, v, end));
                }
                Direction::Horizontal => {
                    let end = if self.grid_ticks && i != 0 {
                        self.view.min.x + TICK_SIZE
                    } else {
                        self.view.max.x
                    };

                    lines.push(Rect::new(self.view.min.x, v, end, v));
                }
            }
        }

        lines
    }

    pub fn tick_centers(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        for i in 1..self.steps {
            let p = i - 1;
            let w1 = self.step_to_world(p as f32);
            let w2 = self.step_to_world(i as f32);
            let v1 = self.world_to_view(w1, 0.0);
            let v2 = self.world_to_view(w2, 0.0);
            let center = (v1 + v2) / 2.0;

            match self.direction {
                Direction::Vertical => {
                    let center = (v1 + v2) / 2.0;
                    points.push(Point::new(center, self.view.max.y));
                }
                Direction::Horizontal => {
                    points.push(Point::new(self.view.min.x, center));
                }
            }
        }

        match self.direction {
            Direction::Vertical => points,
            Direction::Horizontal => points.into_iter().rev().collect(),
        }
    }

    pub fn centered_text_rects(&self, n_labels: i32) -> Vec<Rect> {
        let mut texts = Vec::<Rect>::new();
        let n_labels = self.steps.min(n_labels + 1);

        for i in 1..n_labels {
            let p = i - 1;
            let w1 = self.step_to_world(p as f32);
            let w2 = self.step_to_world(i as f32);
            let v1 = self.world_to_view(w1, 0.0);
            let v2 = self.world_to_view(w2, 0.0);

            match self.direction {
                Direction::Vertical => {
                    let width = v2 - v1;
                    let height = self.label_size as f32;
                    texts.push(Rect::new(v1, self.view.max.y + LABEL_OFFSET, width, height));
                }
                Direction::Horizontal => {
                    let height = v1 - v2;
                    let width = self.label_size as f32;
                    texts.push(Rect::new(
                        self.view.min.x - LABEL_OFFSET - width,
                        v2,
                        width,
                        height,
                    ));
                }
            }
        }

        texts
    }

    pub fn text_data(&self, n_labels: usize) -> Vec<TextData> {
        let mut texts = Vec::<TextData>::new();
        let n_labels = self.steps.min(n_labels as i32);

        for i in 0..n_labels {
            let w = self.step_to_world(i as f32);
            let v = self.world_to_view(w, 0.0);

            match self.direction {
                Direction::Vertical => {
                    texts.push(TextData {
                        x: v,
                        y: self.view.max.y + LABEL_OFFSET,
                        anchor: "start",
                        baseline: "hanging",
                    });
                }
                Direction::Horizontal => {
                    texts.push(TextData {
                        x: self.view.min.x - LABEL_OFFSET,
                        y: v,
                        anchor: "end",
                        baseline: "text-bottom",
                    });
                }
            }
        }

        texts
    }

    pub fn generated_labels(&self) -> Labels {
        let mut labels = Labels::new();

        for i in 0..=self.steps {
            if let Some(func) = self.label_interpolation {
                labels.push(func(self.world_start + i as f32 * self.step_len));
            } else {
                labels.push(format!("{}", self.world_start + i as f32 * self.step_len));
            }
        }

        labels
    }
}

pub(crate) struct AxisBuilder<'a> {
    view: Rect,
    lowest: Option<f32>,
    highest: Option<f32>,
    direction: Direction,
    label_interpolation: Option<fn(f32) -> String>,
    labels_centered: bool,
    label_size: i32,
    grid_ticks: bool,
    max_ticks: i32,
    stacked_series: bool,
    series: Option<&'a Series>,
    labels: Option<&'a Labels>,
}

impl<'a> Default for AxisBuilder<'a> {
    fn default() -> Self {
        Self {
            view: Rect::default(),
            highest: None,
            lowest: None,
            direction: Direction::Horizontal,
            label_interpolation: None,
            labels_centered: false,
            label_size: 60,
            grid_ticks: false,
            max_ticks: 8,
            stacked_series: false,
            series: None,
            labels: None,
        }
    }
}

impl<'a> AxisBuilder<'a> {
    pub fn with_view(mut self, view: Rect) -> Self {
        self.view = view;
        self
    }

    pub fn with_lowest(mut self, lowest: Option<f32>) -> Self {
        self.lowest = lowest;
        self
    }

    pub fn with_highest(mut self, highest: Option<f32>) -> Self {
        self.highest = highest;
        self
    }

    pub fn with_series(mut self, series: &'a Series) -> Self {
        self.series = Some(series);
        self
    }

    pub fn with_stacked_series(mut self, stacked: bool) -> Self {
        self.stacked_series = stacked;
        self
    }

    pub fn with_labels(mut self, labels: Option<&'a Labels>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_label_size(mut self, size: i32) -> Self {
        self.label_size = size;
        self
    }

    pub fn with_max_ticks(mut self, n_ticks: i32) -> Self {
        self.max_ticks = n_ticks;
        self
    }

    pub fn with_grid_ticks(mut self, show_ticks: bool) -> Self {
        self.grid_ticks = show_ticks;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_centered_labels(mut self, labels: Option<&'a Labels>) -> Self {
        self.labels = labels;
        self.labels_centered = true;
        self
    }

    pub fn with_label_interpolation(mut self, func: Option<fn(f32) -> String>) -> Self {
        self.label_interpolation = func;
        self
    }

    pub fn build(self) -> Axis {
        if let Some(series) = self.series {
            let highest = if let Some(high) = self.highest {
                high
            } else if self.stacked_series {
                MultiZip(series.iter().map(|a| a.iter().copied()).collect())
                    .map(|t| t.iter().sum())
                    .reduce(f32::max)
                    .unwrap()
            } else {
                series
                    .iter()
                    .map(|a| a.iter().copied().reduce(f32::max).unwrap())
                    .reduce(f32::max)
                    .unwrap()
            };

            //if self.stacked_series {
            //    let mzip = MultiZip(series.iter().map(|a| a.iter()).collect());

            //    for z in mzip {
            //        debug!("{z:?}");
            //    }
            //}

            let lowest = if let Some(low) = self.lowest {
                low
            } else {
                series
                    .iter()
                    .map(|a| a.iter().copied().reduce(f32::min).unwrap())
                    .reduce(f32::min)
                    .unwrap()
            };

            debug!("highest: {}", highest);
            debug!("lowest: {}", lowest);
            let value_range = highest - lowest;
            let minimum_tick = value_range / (self.max_ticks as f32 - 2.0);
            let magnitude = magnitude(minimum_tick);
            let residual = minimum_tick / magnitude;
            debug!("magnitude: {}", magnitude);

            let step = match residual {
                n if n > 9.0 => 10.0,
                n if n > 8.0 => 9.0,
                n if n > 7.0 => 8.0,
                n if n > 6.0 => 7.0,
                n if n > 5.0 => 6.0,
                n if n > 4.0 => 5.0,
                n if n > 3.0 => 4.0,
                n if n > 2.5 => 3.0,
                n if n > 2.0 => 2.5,
                n if n > 1.5 => 2.0,
                n if n > 1.0 => 1.5,
                _ => 1.0,
            } * magnitude;

            debug!("step_len: {}", step);

            let max = if self.highest.is_some() {
                highest
            } else {
                let max = (highest / step).ceil() * step;
                if max < highest {
                    debug!("step added to max");
                    max + step
                } else {
                    max
                }
            };

            let min = if self.lowest.is_some() {
                lowest
            } else {
                let min = (lowest / step).floor() * step;
                if min > lowest {
                    debug!("step added to min");
                    min - step
                } else {
                    min
                }
            };

            let range = max - min;
            debug!("range: {} min: {}, max: {}", range, min, max);
            let steps = unsafe { (range / step).round().to_int_unchecked::<i32>() + 1 };
            debug!("steps: {}", steps);

            Axis {
                view: self.view,
                step_len: step,
                steps,
                world_start: min,
                world: range,
                label_interpolation: self.label_interpolation,
                grid_ticks: self.grid_ticks,
                label_size: self.label_size,
                direction: self.direction,
            }
        } else if let Some(labels) = self.labels {
            let len = labels.len();
            let steps = if self.labels_centered {
                len as i32 + 1
            } else {
                len as i32
            };

            Axis {
                view: self.view,
                step_len: steps as f32 / (steps as f32 - 1.0),
                steps,
                world: steps as f32,
                grid_ticks: self.grid_ticks,
                label_size: self.label_size,
                direction: self.direction,
                ..Axis::default()
            }
        } else {
            Axis::default()
        }
    }
}

pub(crate) struct Grid {
    pub x: Axis,
    pub y: Axis,
}

impl Grid {
    pub fn new<'a>(x: AxisBuilder<'a>, y: AxisBuilder<'a>) -> Grid {
        Grid {
            x: x.with_direction(Direction::Vertical).build(),
            y: y.with_direction(Direction::Horizontal).build(),
        }
    }

    pub fn world_to_view(&self, cx: f32, cy: f32, inverted: bool) -> Point {
        if inverted {
            Point {
                x: self.x.world_to_view(cx, self.x.world_start),
                y: self.y.world_to_view(self.y.step_to_world(cy), 0.0),
            }
        } else {
            Point {
                x: self.x.world_to_view(self.x.step_to_world(cx), 0.0),
                y: self.y.world_to_view(cy, self.y.world_start),
            }
        }
    }

    pub fn lines(&self) -> Vec<Rect> {
        [self.x.lines().as_slice(), self.y.lines().as_slice()].concat()
    }

    pub fn text_data(&self, x_n_labels: Option<usize>, y_n_labels: Option<usize>) -> Vec<TextData> {
        [
            self.x.text_data(x_n_labels.unwrap_or(0)).as_slice(),
            self.y.text_data(y_n_labels.unwrap_or(0)).as_slice(),
        ]
        .concat()
    }
}
