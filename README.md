# Dioxux Charts

A simple chart components library for Dioxus

This crate provides some basic SVG-based chart components, customizable with
CSS, to be used with the [Dioxus](https://dioxuslabs.com/) GUI library. The
components configuration was designed to be similar to what one would find
in JavaScript chart libraries.

The components available currently are:

- `PieChart`: for Pie, Donut and Gauge charts
- `BarChart`: for Bar and Stacked Bar charts, vertical or horizontal
- `LineChart`

## Usage

This crate is [on crates.io](https://crates.io/crates/dioxus-charts) and can be
used by adding `dioxus_charts` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
dioxus_charts = "0.1.0"
```

## Example

```rust
use dioxus::prelude::*;
use dioxus_charts::BarChart;

fn app(cx: Scope) -> Element {
	cx.render(rsx! {
		BarChart {
			padding_top: 30,
			padding_left: 70,
			padding_right: 50,
			padding_bottom: 30,
			bar_width: "10%",
			horizontal_bars: true,
			label_interpolation: |v| format!("{v}%"),
			series: vec![
				vec![63.0, 14.4, 8.0, 5.1, 1.8],
			],
			labels: vec!["Chrome".into(), "Safari".into(), "IE/Edge".into(), "Firefox".into(), "Opera".into()]
		}
	})
}
 ```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Dioxus-Charts by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
