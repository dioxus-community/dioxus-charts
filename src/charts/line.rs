use dioxus::prelude::*;

use crate::grid::{Axis, Grid};
use crate::types::*;

/// The `LineChart` properties struct for the configuration of the line chart.
#[allow(clippy::struct_excessive_bools)]
#[derive(PartialEq, Props)]
pub struct LineChartProps<'a> {
    series: &'a Series,
    #[props(optional)]
    labels: Option<Labels>,
    #[props(optional)]
    series_labels: Option<Labels>,

    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "100%")]
    height: &'a str,
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

    #[props(default = true)]
    show_grid: bool,
    #[props(default = true)]
    show_dotted_grid: bool,
    #[props(default = false)]
    show_grid_ticks: bool,
    #[props(default = true)]
    show_labels: bool,
    #[props(default = true)]
    show_dots: bool,
    #[props(default = true)]
    show_lines: bool,
    #[props(default = true)]
    show_line_labels: bool,

    #[props(default = "1%")]
    line_width: &'a str,
    #[props(default = "3%")]
    dot_size: &'a str,
    #[props(optional)]
    label_interpolation: Option<fn(f32) -> String>,

    #[props(optional)]
    lowest: Option<f32>,
    #[props(optional)]
    highest: Option<f32>,
    #[props(default = 8)]
    max_ticks: i32,

    #[props(default = "dx-chart-line")]
    class_chart_line: &'a str,
    #[props(default = "dx-line")]
    class_line: &'a str,
    #[props(default = "dx-line-path")]
    class_line_path: &'a str,
    #[props(default = "dx-line-dot")]
    class_line_dot: &'a str,
    #[props(default = "dx-line-label")]
    class_line_label: &'a str,
    #[props(default = "dx-grid")]
    class_grid: &'a str,
    #[props(default = "dx-grid-line")]
    class_grid_line: &'a str,
    #[props(default = "dx-grid-label")]
    class_grid_label: &'a str,
    #[props(default = "dx-grid-labels")]
    class_grid_labels: &'a str,
}

/// This is the `LineChart` function used to render the line chart `Element`.
/// In Dioxus, components are just functions, so this is the main `PieChart`
/// component to be used inside `rsx!` macros in your code.
///
/// # Example
///
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_charts::LineChart;
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         LineChart {
///             padding_top: 30,
///             padding_left: 65,
///             padding_right: 80,
///             padding_bottom: 30,
///             label_interpolation: |v| format!("${v:.0}B"),
///             series: vec![
///                 vec![29.0, 30.5, 32.6, 35.0, 37.5],
///                 vec![20.0, 25.1, 26.0, 25.2, 26.6],
///                 vec![18.0, 21.0, 22.5, 24.0, 25.1],
///                 vec![12.5, 17.0, 19.3, 20.1, 21.0],
///             ],
///             labels: vec!["2020A".into(), "2021E".into(), "2022E".into(), "2023E".into(), "2024E".into()],
///             series_labels: vec!["Disney".into(), "Comcast".into(), "Warner".into(), "Netflix".into()],
///         }
///     })
/// }
/// ```
///
/// # Props
///
/// - `series`: [Vec]<[Vec]<[f32]>> (**required**): The series vector of vectors with the series values.
/// - `labels`: [Vec]<[String]> (optional): Optional labels to show on the labels axis.
/// - `series_labels`: [Vec]<[String]> (optional): Optional labels to show for each generated line.
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
/// - `show_dots`: [bool] (default: `true`): Show/hide the line dots.
/// - `show_lines`: [bool] (default: `true`): Show/hide the series lines.
/// - `show_line_labels`: [bool] (default: `true`): Show/hide the labels for the lines.
/// ---
/// - `line_width`: &[str] (default: `"1%"`): The width of the series lines.
/// - `dot_size`: &[str] (default: `"3%"`): The size of the line dots.
/// - `label_interpolation`: fn([f32]) -> [String] (optional): Function for formatting the
/// generated labels.
/// ---
/// - `class_chart_line`: &[str] (default: `"dx-chart-line"`): The HTML element `class` of the
/// chart.
/// - `class_line`: &[str] (default: `"dx-line"`): The HTML element `class` of the whole line.
/// - `class_line_path`: &[str] (default: `"dx-line"`): The HTML element `class` of the line path.
/// - `class_line_dot`: &[str] (default: `"dx-line-dot"`): The HTML element `class` of the line dot.
/// - `class_line_label`: &[str] (default: `"dx-line-label"`): The HTML element `class` of the line
/// labels.
/// - `class_grid`: &[str] (default: `"dx-grid"`): The HTML element `class` of the grid.
/// - `class_grid_line`: &[str] (default: `"dx-grid-line"`): The HTML element `class` of every grid
/// line.
/// - `class_grid_label`: &[str] (default: `"dx-grid-label"`): The HTML element `class` of the grid
/// labels.
/// - `class_grid_labels`: &[str] (default: `"dx-grid-labels"`): The HTML element `class` of the
/// group of grid labels.
#[allow(non_snake_case)]
pub fn LineChart<'a>(cx: Scope<'a, LineChartProps<'a>>) -> Element {
    for series in cx.props.series.iter() {
        if series.is_empty() {
            return cx.render(rsx!("Pie chart error: empty series"));
        }
    }

    let view = Rect::new(
        cx.props.padding_left as f32,
        cx.props.padding_top as f32,
        (cx.props.viewbox_width - cx.props.padding_right) as f32,
        (cx.props.viewbox_height - cx.props.padding_bottom) as f32,
    );

    let max_ticks = cx.props.max_ticks.max(3);

    let axis_x = Axis::builder()
        .with_view(view)
        .with_grid_ticks(cx.props.show_grid_ticks)
        .with_labels(cx.props.labels.as_ref());

    let axis_y = Axis::builder()
        .with_view(view)
        .with_max_ticks(max_ticks)
        .with_grid_ticks(cx.props.show_grid_ticks)
        .with_series(&cx.props.series)
        .with_label_interpolation(cx.props.label_interpolation)
        .with_highest(cx.props.highest)
        .with_lowest(cx.props.lowest);

    let grid = Grid::new(axis_x, axis_y);
    let lines = grid.lines();
    let generated_labels = grid.y.generated_labels();

    let grid_labels = if cx.props.show_labels {
        if let Some(labels) = cx.props.labels.as_ref() {
            Some(
                grid.text_data(Some(labels.len()), Some(generated_labels.len()))
                    .into_iter()
                    .zip(labels.iter().chain(generated_labels.iter()))
                    .collect::<Vec<(TextData, &String)>>(),
            )
        } else {
            Some(
                grid.y
                    .text_data(generated_labels.len())
                    .into_iter()
                    .zip(generated_labels.iter())
                    .collect::<Vec<(TextData, &String)>>(),
            )
        }
    } else {
        None
    };

    let mut color_var = 255.0;
    let dotted_stroke = if cx.props.show_dotted_grid {
        &"2px"
    } else {
        &"0px"
    };

    cx.render(rsx! {
        div {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{cx.props.width}",
                height: "{cx.props.height}",
                class: "{cx.props.class_chart_line}",
                preserve_aspect_ratio: "xMidYMid meet",
                view_box: "0 0 {cx.props.viewbox_width} {cx.props.viewbox_height}",
                cx.props.show_grid.then(|| rsx! {
                    g {
                        class: "{cx.props.class_grid}",
                        lines.iter().map(|line| {
                            rsx! {
                                line {
                                    x1: "{line.min.x}",
                                    y1: "{line.min.y}",
                                    x2: "{line.max.x}",
                                    y2: "{line.max.y}",
                                    class: "{cx.props.class_grid_line}",
                                    stroke: "rgba(20, 20, 20, 0.8)",
                                    stroke_dasharray: "{dotted_stroke}",
                                }
                            }
                        }),
                    }
                }),
                grid_labels.map(|labels| rsx! {
                    g {
                        class: "{cx.props.class_grid_labels}",
                        labels.iter().map(|(text, label)| rsx! {
                            text {
                                dx: "{text.x}",
                                dy: "{text.y}",
                                text_anchor: "{text.anchor}",
                                class: "{cx.props.class_grid_label}",
                                alignment_baseline: "{text.baseline}",
                                label.as_str()
                            }
                        })
                    }
                }),
                cx.props.series
                    .iter()
                    .enumerate()
                    .zip(cx.props.series_labels
                        .as_ref()
                        .unwrap_or(&vec!())
                        .iter()
                        .chain(std::iter::repeat(&"".to_owned())))
                    .map(|((i, a), label)| {

                    let mut commands = Vec::<String>::with_capacity(a.len());
                    let mut dots = Vec::<Rect>::with_capacity(a.len());
                    let mut text_point: Option<Point> = None;

                    color_var -= 75.0 * (1.0 / (i + 1) as f32);

                    for (index, v) in a.iter().enumerate() {
                        let point = grid.world_to_view(index as f32, *v, false);

                        if index == 0 {
                            commands.push(format!("M{},{}", point.x, point.y));
                        } else {
                            commands.push(format!("L{},{}", point.x, point.y));
                        }

                        if cx.props.show_dots {
                            dots.push(Rect::new(point.x, point.y, point.x + 0.1, point.y));
                        }

                        if !label.is_empty() && index == (a.len() - 1) {
                            text_point = Some(point);
                        }
                    }

                    let commands = commands.join(" ");

                    rsx! {
                        g {
                            class: "{cx.props.class_line}-{i}",
                            path {
                                d: "{commands}",
                                class: "{cx.props.class_line_path}",
                                stroke: "rgb({color_var}, 40, 40)",
                                stroke_width: "{cx.props.line_width}",
                                stroke_linecap: "round",
                                fill: "transparent",
                            },
                            dots.iter().map(|d| {
                                rsx! {
                                    line {
                                        x1: "{d.min.x}",
                                        y1: "{d.min.y}",
                                        x2: "{d.max.x}",
                                        y2: "{d.max.y}",
                                        class: "{cx.props.class_line_dot}",
                                        stroke: "rgb({color_var}, 40, 40)",
                                        stroke_width: "{cx.props.dot_size}",
                                        stroke_linecap: "round",
                                    }
                                }
                            }),
                            text_point.map(|point| {
                                rsx! {
                                    text {
                                        dx: format_args!("{}", point.x + 10.0),
                                        dy: "{point.y}",
                                        text_anchor: "start",
                                        color: "rgb({color_var}, 40, 40)",
                                        class: "{cx.props.class_line_label}",
                                        label.as_str()
                                    }
                                }
                            }),
                        }
                    }
                }),
            }
        }
    })
}
