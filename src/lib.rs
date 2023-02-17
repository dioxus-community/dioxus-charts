/*!
A simple chart components library for Dioxus

This crate provides some basic SVG-based chart components, customizable with
CSS, to be used with the [Dioxus] GUI library. The components configuration
was designed to be similar to what one would find in JavaScript chart libraries.

The components available are:

- [PieChart](crate::charts::PieChart): for Pie, Donut and Gauge charts
- [BarChart](crate::charts::BarChart): for Bar and Stacked Bar charts, vertical
or horizontal
- [LineChart](crate::charts::LineChart)

# Usage
This crate is [on crates.io](https://crates.io/crates/dioxus-charts) and can be
used by adding `dioxus-charts` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
dioxus-charts = "0.1.1"
```

[Dioxus]: https://dioxuslabs.com/
*/

#![deny(missing_docs)]

mod grid;
mod types;
mod utils;

pub mod charts {
    //! Chart components
    //!
    //! This module contains all the charts available:
    //! - [PieChart](crate::charts::PieChart)
    //! - [BarChart](crate::charts::BarChart)
    //! - [LineChart](crate::charts::LineChart)

    /// Module for the [BarChart](pie::PieChart) component and its configuration types
    pub mod bar;
    /// Module for the [LineChart](pie::PieChart) component and its configuration types
    pub mod line;
    /// Module for the [PieChart](pie::PieChart) component and its configuration types
    pub mod pie;

    pub use bar::BarChart;
    pub use line::LineChart;
    pub use pie::PieChart;
}

pub use crate::charts::{BarChart, LineChart, PieChart};
