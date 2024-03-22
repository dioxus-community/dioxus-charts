use dioxus::prelude::*;

use dioxus_charts::charts::pie::LabelPosition;
use dioxus_charts::{BarChart, LineChart, PieChart};

pub fn demo_element() -> Element {
    rsx! {
        style {
            {include_str!("./custom.css")},
        },
        div {
            class: "bg-gray-600",
            div {
                class: "mx-auto max-w-2xl py-8 px-4 sm:py-10 sm:px-6 lg:max-w-7xl lg:px-8 space-y-4",
                h2 {
                    class: "text-2xl font-bold tracking-tight text-gray-200",
                    "Examples"
                }

                div {
                    class: "grid grid-cols-1 gap-y-12 gap-x-6 sm:grid-cols-2 lg:grid-cols-3 xl:gap-x-8",

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            BarChart {
                                padding_top: 30,
                                padding_left: 70,
                                padding_right: 50,
                                padding_bottom: 30,
                                bar_width: "10%",
                                horizontal_bars: true,
                                label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                                series: vec![
                                    vec![63.0, 14.4, 8.0, 5.1, 1.8],
                                ],
                                labels: vec!["Chrome".into(), "Safari".into(), "IE/Edge".into(), "Firefox".into(), "Opera".into()]
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Global desktop browser market share for 2022"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            BarChart {
                                padding_top: 30,
                                padding_left: 80,
                                padding_right: 60,
                                padding_bottom: 30,
                                bar_distance: 25.0,
                                horizontal_bars: true,
                                label_size: 70,
                                label_interpolation: (|v| format!("${v:.0}B")) as fn(f32) -> String,
                                series: vec![
                                    vec![2901.0, 2522.0, 1917.0, 1691.0, 1061.0],
                                    vec![2307.0, 1640.0, 1125.0, 939.8, 668.8],
                                ],
                                labels: vec!["Apple".into(), "Microsoft".into(), "Alphabet".into(), "Amazon".into(), "Tesla".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Market value of top tech companies [Jan-Nov 2022]"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            BarChart {
                                padding_top: 30,
                                padding_left: 50,
                                padding_right: 30,
                                padding_bottom: 40,
                                bar_width: "3.5%",
                                bar_distance: 20.0,
                                show_series_labels: false,
                                label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                                label_size: 70,
                                series: vec![
                                    vec![11.3, 4.3, 2.6, 1.7, 0.3],
                                    vec![13.7, 6.1, 4.1, 2.3, 0.7],
                                    vec![15.3, 6.9, 4.8, 3.2, 0.9],
                                ],
                                labels: vec!["Asia".into(), "Western Europe".into(), "USA".into(), "Eastern Europe".into(), "Latin America".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Ecommerce share of FMCG value, 2019-2021"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            BarChart {
                                padding_top: 30,
                                padding_left: 40,
                                padding_right: 30,
                                padding_bottom: 30,
                                label_interpolation: (|v| format!("${v}")) as fn(f32) -> String,
                                show_series_labels: false,
                                stacked_bars: true,
                                series: vec![
                                    vec![32.0, 24.0, 19.0, 18.5, 11.5],
                                    vec![9.5, 11.0, 17.0, 5.5, 2.5],
                                    vec![4.0, 3.5, 2.5, 5.0, 1.9],
                                    vec![38.5, 24.0, 19.0, 15.0, 2.3],
                                ],
                                labels: vec!["Miami".into(), "Barcelona".into(), "Tokyo".into(), "Rio de Janeiro".into(), "Mexico City".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Date night cost (2 long drinks, taxi, big mac and club entry)"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            LineChart {
                                padding_top: 30,
                                padding_left: 65,
                                padding_right: 80,
                                padding_bottom: 30,
                                label_interpolation: (|v| format!("${v}B")) as fn(f32) -> String,
                                series: vec![
                                    vec![29.0, 30.5, 32.6, 35.0, 37.5],
                                    vec![20.0, 25.1, 26.0, 25.2, 26.6],
                                    vec![18.0, 21.0, 22.5, 24.0, 25.1],
                                    vec![12.5, 17.0, 19.3, 20.1, 21.0],
                                ],
                                labels: vec!["2020A".into(), "2021E".into(), "2022E".into(), "2023E".into(), "2024E".into()],
                                series_labels: vec!["Disney".into(), "Comcast".into(), "Warner".into(), "Netflix".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Content spending at major media and tech companies"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            LineChart {
                                width: "100%",
                                height: "100%",
                                padding_top: 30,
                                padding_left: 50,
                                padding_right: 90,
                                padding_bottom: 30,
                                show_grid_ticks: true,
                                show_dotted_grid: false,
                                label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                                series: vec![
                                    vec![75.77, 73.95, 74.56, 78.25, 77.15, 62.64, 67.51],
                                    vec![57.17, 57.78, 54.69, 52.95, 51.78, 41.0, 47.25],
                                    vec![23.12, 26.5, 26.1, 29.84, 25.05, 20.41, 20.1],
                                    vec![26.02, 21.48, 21.05, 22.64, 20.64, 17.19, 16.31],
                                    vec![0.0, 13.65, 12.3, 13.35, 11.17, 9.87, 10.15],
                                ],
                                labels: vec!["2016".into(), "2017".into(), "2018".into(), "2019".into(), "2020".into(), "2021".into(), "2022".into()],
                                series_labels: vec!["Firefox".into(), "Chromium".into(), "Chrome".into(), "Epiphany".into(), "Konqueror".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Arch Linux browser package relative usage"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            PieChart {
                                width: "100%",
                                height: "100%",
                                start_angle: -60.0,
                                label_position: LabelPosition::Outside,
                                label_offset: 35.0,
                                padding: 20.0,
                                series: vec![59.54, 17.2, 9.59, 7.6, 5.53, 0.55],
                                labels: vec!["Asia".into(), "Africa".into(), "Europe".into(), "N. America".into(), "S. America".into(), "Oceania".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "World population share by continents 2022"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            PieChart {
                                width: "100%",
                                height: "100%",
                                start_angle: 50.0,
                                label_position: LabelPosition::Outside,
                                label_offset: 27.0,
                                donut: true,
                                padding: 20.0,
                                series: vec![5.0, 4.0, 4.0, 2.0, 2.0, 2.0, 1.0, 1.0],
                                labels: vec!["Brazil".into(), "Germany".into(), "Italy".into(), "France".into(), "Uruguay".into(), "Argentina".into(), "England".into(), "Spain".into()],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "World Cup titles at 2022"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            PieChart {
                                width: "100%",
                                height: "100%",
                                viewbox_width: 500,
                                viewbox_height: 300,
                                start_angle: 270.0,
                                show_ratio: 0.5,
                                donut: true,
                                label_position: LabelPosition::Center,
                                label_offset: 60.0,
                                label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                                series: vec![50.0, 25.0, 25.0],
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "Gauge Chart"
                        }
                    }

                    div {
                        class: "inline-grid grid-cols-1 gap-y-2",
                        div {
                            class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg bg-gray-300 xl:aspect-w-7 xl:aspect-h-8",
                            BarChart {
                                padding_top: 30,
                                padding_left: 100,
                                padding_right: 70,
                                padding_bottom: 30,
                                bar_width: "6%",
                                horizontal_bars: true,
                                viewbox_width: 600,
                                viewbox_height: 500,
                                show_grid_ticks: true,
                                show_dotted_grid: false,
                                label_size: 90,
                                label_interpolation: (|v| format!("{v}km²")) as fn(f32) -> String,
                                series: vec![
                                    vec![150.0, 22.3, 13.5, 12.7, 3.7, 2.78, 2.5, 1.47],
                                ],
                                labels: vec![
                                    "Walt Disney Orlando, USA".into(),
                                    "Disneyland Paris, France".into(),
                                    "Beto Carrero, Brazil".into(),
                                    "Disneyland Anaheim, USA".into(),
                                    "Alton Towers, UK".into(),
                                    "Universal Orlando, USA".into(),
                                    "Tokyo Disney, Japan".into(),
                                    "Cedar Point, USA".into(),
                                ]
                            }
                        }
                        h3 {
                            class: "truncate tracking-tight text-sm text-gray-200",
                            "8 biggest amusement parks in the world"
                        }
                    }
                }
            }
        }
    }
}
