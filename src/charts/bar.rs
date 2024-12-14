use dioxus::prelude::*;

use crate::grid::{Axis, Grid};
use crate::types::*;

/// The `BarChart` properties struct for the configuration of the bar chart.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, PartialEq, Props)]
pub struct BarChartProps {
    series: Series,
    #[props(optional)]
    labels: Option<Labels>,

    #[props(default = "100%".to_string(), into)]
    width: String,
    #[props(default = "100%".to_string(), into)]
    height: String,
    #[props(default = 600)]
    viewbox_width: i32,
    #[props(default = 400)]
    viewbox_height: i32,

    #[props(default)]
    padding_top: i32,
    #[props(default)]
    padding_bottom: i32,
    #[props(default)]
    padding_left: i32,
    #[props(default)]
    padding_right: i32,

    #[props(optional)]
    lowest: Option<f32>,
    #[props(optional)]
    highest: Option<f32>,
    #[props(default = 8)]
    max_ticks: i32,

    #[props(default = true)]
    show_grid: bool,
    #[props(default = true)]
    show_dotted_grid: bool,
    #[props(default = false)]
    show_grid_ticks: bool,
    #[props(default = true)]
    show_labels: bool,
    #[props(default = true)]
    show_series_labels: bool,

    #[props(default = 60)]
    label_size: i32,
    #[props(optional)]
    label_interpolation: Option<fn(f32) -> String>,

    #[props(default = "5%".to_string(), into)]
    bar_width: String,
    #[props(default = 30.0)]
    bar_distance: f32,
    #[props(default = false)]
    horizontal_bars: bool,
    #[props(default = false)]
    stacked_bars: bool,

    #[props(default = "dx-chart-bar".to_string(), into)]
    class_chart_bar: String,
    #[props(default = "dx-bar".to_string(), into)]
    class_bar: String,
    #[props(default = "dx-bar-group".to_string(), into)]
    class_bar_group: String,
    #[props(default = "dx-bar-label".to_string(), into)]
    class_bar_label: String,
    #[props(default = "dx-grid".to_string(), into)]
    class_grid: String,
    #[props(default = "dx-grid-line".to_string(), into)]
    class_grid_line: String,
    #[props(default = "dx-grid-label".to_string(), into)]
    class_grid_label: String,
    #[props(default = "dx-grid-labels".to_string(), into)]
    class_grid_labels: String,
}

/// This is the `BarChart` function used to render the bar chart `Element`.
/// In Dioxus, components are just functions; this is the main `BarChart`
/// component to be used inside `rsx!` macros in your code.
///
/// # Example
///
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_charts::BarChart;
///
/// fn app() -> Element {
///     rsx! {
///         BarChart {
///             padding_top: 30,
///             padding_left: 70,
///             padding_right: 50,
///             padding_bottom: 30,
///             bar_width: "10%",
///             horizontal_bars: true,
///             label_interpolation: (|v| format!("{v:.1}%")) as fn(f32) -> String,
///             series: vec![
///                 vec![63.0, 14.4, 8.0, 5.1, 1.8],
///             ],
///             labels: vec!["Chrome".into(), "Safari".into(), "IE/Edge".into(), "Firefox".into(), "Opera".into()]
///         }
///     }
/// }
/// ```
///
/// # Props
///
/// - `series`: [Vec]<[Vec]<[f32]>> (**required**): The series vector of vectors with the all series values.
/// - `labels`: [Vec]<[String]> (optional): Optional labels to show on the labels axis.
/// ---
/// - `width`: &[str] (default: `"100%"`): The SVG element width attribute. It also accepts any
/// other CSS style, i.e., "200px"
/// - `height`: &[str] (default: `"100%"`): The SVG height counter-part of the `width` prop above.
/// - `viewbox_width`: [i32] (default: `600`): The SVG viewbox width. Together with
/// `viewbox_height` it is useful for adjusting the aspect ratio for longer charts.
/// - `viewbox_height`: [i32] (default: `400`): The SVG viewbox height.
/// ---
/// - `padding_top`: [i32] (default: `0`): Padding for the top side of the view box.
/// - `padding_bottom`: [i32] (default: `0`): Padding for the bottom side of the view box.
/// - `padding_left`: [i32] (default: `0`): Padding for the left side of the view box.
/// - `padding_right`: [i32] (default: `0`): Padding for the right side of the view box.
/// ---
/// - `lowest`: [f32] (optional): The lowest number on the chart for the value axis.
/// - `highest`: [f32] (optional): The highest number on the chart for the value axis.
/// - `max_ticks`: [i32] (default: `8`): The maximum number of ticks on the generated value axis.
/// ---
/// - `show_grid`: [bool] (default: `true`): Show/hide the chart grid.
/// - `show_dotted_grid`: [bool] (default: `true`): Show the chart grid with dotted style or not.
/// - `show_grid_ticks`: [bool] (default: `false`): Show the chart grid ticks instead of drawing the
/// whole grid lines for a cleaner look.
/// - `show_labels`: [bool] (default: `true`): Show/hide the labels.
/// - `show_series_labels`: [bool] (default: `true`): Show/hide the values labels at the top of
/// bars.
/// ---
/// - `label_size`: [i32] (default: `60`): The maximum width or height of the label rect depending
/// on whether the chart shows horizontal or vertical bars.
/// - `label_interpolation`: fn([f32]) -> [String] (optional): Function for formatting the
/// generated labels for values.
/// ---
/// - `bar_width`: &[str] (default: `"5%"`): The width of each bar.
/// - `bar_distance`: [f32] (default: `30.0`): The distance between the bars for charts that have
/// multiple ones side by side.
/// - `horizontal_bars`: [bool] (default: `false`): Show horizontal bars.
/// - `stacked_bars`: [bool] (default: `false`): Build a Stacked Bars chart.
/// ---
/// - `class_chart_bar`: &[str] (default: `"dx-chart-line"`): The HTML element `class` of the
/// chart.
/// - `class_bar`: &[str] (default: `"dx-bar"`): The HTML element `class` of the whole line.
/// - `class_bar_group`: &[str] (default: `"dx-bar-group"`): The HTML element `class` of the line path.
/// - `class_bar_label`: &[str] (default: `"dx-bar-label"`): The HTML element `class` of the line
/// labels.
/// - `class_grid`: &[str] (default: `"dx-grid"`): The HTML element `class` of the grid.
/// - `class_grid_line`: &[str] (default: `"dx-grid-line"`): The HTML element `class` of every grid
/// line.
/// - `class_grid_label`: &[str] (default: `"dx-grid-label"`): The HTML element `class` of the grid
/// labels.
/// - `class_grid_labels`: &[str] (default: `"dx-grid-labels"`): The HTML element `class` of the
/// group of grid labels.
#[allow(non_snake_case)]
#[component]
pub fn BarChart(props: BarChartProps) -> Element {
    for series in props.series.iter() {
        if series.is_empty() {
            return rsx!("Bar chart error: empty series");
        }
    }

    let grid = {
        let view = Rect::new(
            props.padding_left as f32,
            props.padding_top as f32,
            (props.viewbox_width - props.padding_right) as f32,
            (props.viewbox_height - props.padding_bottom) as f32,
        );

        let lowest = if let Some(low) = props.lowest {
            low
        } else {
            0.0
        };

        let max_ticks = props.max_ticks.max(3);

        let axis_x = Axis::builder()
            .with_view(view)
            .with_grid_ticks(props.show_grid_ticks)
            .with_label_size(props.label_size)
            .with_centered_labels(props.labels.as_ref());

        let axis_y = Axis::builder()
            .with_view(view)
            .with_max_ticks(max_ticks)
            .with_grid_ticks(props.show_grid_ticks)
            .with_series(&props.series)
            .with_stacked_series(props.stacked_bars)
            .with_label_interpolation(props.label_interpolation)
            .with_highest(props.highest)
            .with_lowest(Some(lowest));

        if props.horizontal_bars {
            Grid::new(axis_y, axis_x)
        } else {
            Grid::new(axis_x, axis_y)
        }
    };

    let (axis_value, axis_label) = if props.horizontal_bars {
        (&grid.x, &grid.y)
    } else {
        (&grid.y, &grid.x)
    };

    let lines = grid.lines();

    let mut color_var = 255.0;
    let dotted_stroke = if props.show_dotted_grid {
        &"2px"
    } else {
        &"0px"
    };
    let generated_labels = axis_value.generated_labels();

    let grid_labels = if props.show_labels {
        Some(
            axis_value
                .text_data(generated_labels.len())
                .into_iter()
                .zip(generated_labels.iter())
                .collect::<Vec<(TextData, &String)>>(),
        )
    } else {
        None
    };

    let grid_centered_labels = if props.show_labels {
        let rects = axis_label
            .centered_text_rects(props.labels.as_ref().unwrap().len() as i32)
            .into_iter();

        let labels = if props.horizontal_bars {
            rects
                .zip(props.labels.as_ref().unwrap().iter().rev())
                .collect::<Vec<(Rect, &String)>>()
        } else {
            rects
                .zip(props.labels.as_ref().unwrap().iter())
                .collect::<Vec<(Rect, &String)>>()
        };

        Some(labels)
    } else {
        None
    };

    let stacked_bars_rects = if props.stacked_bars {
        let tick_centers = axis_label.tick_centers();
        let mut all_series_rects = Vec::<Vec<Rect>>::new();
        let mut last_bar_ends: Option<Vec<f32>> = None;

        for a in props.series.iter() {
            let mut rects = Vec::<Rect>::new();
            let mut view_bar_ends = Vec::<f32>::new();

            for (point, (i, v)) in tick_centers.iter().zip(a.iter().enumerate()) {
                let rect = if let Some(bar_ends) = &last_bar_ends {
                    let last_end = bar_ends[i];
                    let end = axis_value.world_to_view(v + last_end, 0.0);
                    view_bar_ends.push(v + last_end);

                    let last_end = axis_value.world_to_view(last_end, 0.0);

                    if props.horizontal_bars {
                        Rect::new(last_end, point.y, end, point.y)
                    } else {
                        Rect::new(point.x, last_end, point.x, end)
                    }
                } else {
                    let end = axis_value.world_to_view(*v, 0.0);
                    view_bar_ends.push(*v);

                    if props.horizontal_bars {
                        Rect::new(point.x, point.y, end, point.y)
                    } else {
                        Rect::new(point.x, point.y, point.x, end)
                    }
                };

                rects.push(rect);
            }

            all_series_rects.push(rects);
            last_bar_ends = Some(view_bar_ends);
        }

        Some(all_series_rects)
    } else {
        None
    };

    let stacked_bars_rects_rsx = stacked_bars_rects.map(|all_series_rects| {
        rsx! {
            //all_series_rects.iter().enumerate().map(|(i, series_rects)| {
            for (i, series_rects) in all_series_rects.iter().enumerate() {
                {color_var -= 75.0 * (1.0 / (i + 1) as f32);}

                g {
                    class: "{props.class_bar_group}-{i}",
                    {
                        series_rects.iter().map(|rect| {
                            rsx! {
                                line {
                                    x1: "{rect.min.x}",
                                    y1: "{rect.min.y}",
                                    x2: "{rect.max.x}",
                                    y2: "{rect.max.y}",
                                    class: "{props.class_bar}",
                                    stroke: "rgb({color_var}, 40, 40)",
                                    stroke_width: "{props.bar_width}",
                                }
                            }
                        })
                    }
                }
            }
        }
    });

    let series_rsx = props.series.iter().enumerate().map(|(i, a)| {
        color_var -= 75.0 * (1.0 / (i + 1) as f32);
        let offset = (i as f32 - (props.series.len() as f32 - 1.0) / 2.0) * props.bar_distance;
        let tick_centers = axis_label.tick_centers();

        let tick_centers_rsx = tick_centers.iter().zip(a.iter()).map(|(point, v)| {
            let end = axis_value.world_to_view(*v, 0.0);
            let (rect, text) = if props.horizontal_bars {
                (
                    Rect::new(point.x, point.y + offset, end, point.y + offset),
                    TextData {
                        x: end + 5.0,
                        y: point.y + offset,
                        anchor: "start",
                        baseline: "middle",
                    },
                )
            } else {
                (
                    Rect::new(point.x + offset, point.y, point.x + offset, end),
                    TextData {
                        x: point.x + offset,
                        y: end - 5.0,
                        anchor: "middle",
                        baseline: "text-bottom",
                    },
                )
            };

            let bar_label = {
                if !props.show_series_labels {
                    String::new()
                } else if let Some(func) = props.label_interpolation {
                    func(*v)
                } else {
                    format!("{}", *v)
                }
            };

            rsx! {
                line {
                    x1: "{rect.min.x}",
                    y1: "{rect.min.y}",
                    x2: "{rect.max.x}",
                    y2: "{rect.max.y}",
                    class: "{props.class_bar}",
                    stroke: "rgb({color_var}, 40, 40)",
                    stroke_width: "{props.bar_width}",
                },
                if props.show_series_labels {
                    text {
                        dx: "{text.x}",
                        dy: "{text.y}",
                        text_anchor: "{text.anchor}",
                        class: "{props.class_bar_label}",
                        alignment_baseline: "{text.baseline}",
                        "{bar_label}"
                    }
                },
            }
        });

        rsx! {
            g {
                class: "{props.class_bar_group}-{i}",
                {tick_centers_rsx}
            }
        }
    });

    rsx! {
        div {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{props.width}",
                height: "{props.height}",
                class: "{props.class_chart_bar}",
                preserve_aspect_ratio: "xMidYMid meet",
                view_box: "0 0 {props.viewbox_width} {props.viewbox_height}",

                if props.show_grid {
                    g {
                        class: "{props.class_grid}",
                        for line in lines {
                            line {
                                x1: "{line.min.x}",
                                y1: "{line.min.y}",
                                x2: "{line.max.x}",
                                y2: "{line.max.y}",
                                class: "{props.class_grid_line}",
                                stroke: "rgba(20, 20, 20, 0.8)",
                                stroke_dasharray: "{dotted_stroke}",
                            }
                        }
                    }
                },

                for labels in grid_labels {
                    g {
                        class: "{props.class_grid_labels}",
                        for (text, label) in labels {
                            text {
                                dx: "{text.x}",
                                dy: "{text.y}",
                                text_anchor: "{text.anchor}",
                                class: "{props.class_grid_label}",
                                alignment_baseline: "{text.baseline}",
                                "{label}"
                            }
                        }
                    }
                },

                for labels in grid_centered_labels {
                    g {
                        class: "{props.class_grid_labels}",
                        for (rect, label) in labels {
                            foreignObject {
                                x: "{rect.min.x}",
                                y: "{rect.min.y}",
                                width: "{rect.max.x}",
                                height: "{rect.max.y}",
                                if props.horizontal_bars {
                                    span {
                                        class: "{props.class_grid_label}",
                                        //width: "100%",
                                        height: "100%",
                                        display: "inline-flex",
                                        align_items: "center",
                                        line_height: "1",
                                        float: "right",
                                        text_align: "right",
                                        "{label}"
                                    }
                                } else {
                                    span {
                                        class: "{props.class_grid_label}",
                                        width: "100%",
                                        height: "100%",
                                        display: "inline-block",
                                        line_height: "1",
                                        text_align: "center",
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                },

                {stacked_bars_rects_rsx}


                if !props.stacked_bars {
                    {series_rsx}
                }
            }
        }
    }
}
