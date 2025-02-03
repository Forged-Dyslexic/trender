# Trender

**Trender** is a terminal graphics library written in Rust. It provides a set of functions to draw “pixels” (or cells) on the terminal by using background colors and text-based techniques. Trender is designed for creating simple terminal-based graphics and animations. Because terminals are text-based, a “real” pixel is emulated by filling a character cell with a background color. The library also offers higher-level functions to fill rows, columns, and even the whole screen, along with some basic utilities for positioning and (in the future) 3D rendering.

> **Note:** This crate is currently under construction. You may encounter issues and incomplete features. Contributions and feedback are welcome!

---

## Table of Contents

- [Installation](#installation)
- [Usage Examples](#usage-examples)
  - [Drawing a Real Pixel](#drawing-a-real-pixel)
  - [Drawing a Square Pixel](#drawing-a-square-pixel)
  - [Filling Rows and Columns](#filling-rows-and-columns)
  - [Screen and Path Tests](#screen-and-path-tests)
- [API Reference](#api-reference)
  - [Basic Drawing Functions](#basic-drawing-functions)
  - [Utility Functions](#utility-functions)
  - [Centering Traits](#centering-traits)
- [Testing](#testing)

---

## Installation

Add Trender to your project by updating your `Cargo.toml`:

```toml
[dependencies]
trender = "0.0.2"
```
Or install via Cargo directly:
``` sh
cargo add trender
```
# Usage Examples
## Drawing a Real Pixel

The fundamental drawing primitive in Trender is the real pixel, drawn using real_cell(). Because terminals display characters as rectangles, this function sets the background color for a single character cell.
```
use crossterm::style::Color;
use trender::real_cell;

fn main() {
    // Draw a “real” pixel at column 10, row 5 in blue
    real_cell(10, 5, Color::Blue);
}
```
## Drawing a Square Pixel

Since terminal cells are rectangular, the cell() function draws two adjacent real cells so that the result approximates a square pixel.
```
use crossterm::style::Color;
use trender::cell;

fn main() {
    // Draw a square pixel at logical coordinates (1,1) in red.
    cell(1, 1, Color::Red);
}
```
## Filling Rows and Columns

Trender includes helper functions to fill entire rows, columns, or the whole screen with a specified color:

Fill a Row:
     ``` fill_row_cell(y, color) fills the given row by drawing square pixels across the terminal width.```
Fill a Column:
   ``` fill_colum_cell(x, color) fills the given column by drawing pixels vertically.```
    Fill the Screen:
```fill_screen_cell(color) uses the terminal size to cover the entire screen.```

Example:
```
use crossterm::style::Color;
use trender::{fill_row_cell, fill_screen_cell, clear};
use std::thread;
use std::time::Duration;

fn main() {
    // Clear the terminal, then fill row 5 with dark blue.
    clear();
    fill_row_cell(5, Color::DarkBlue);
    
    // Wait a bit and then fill the entire screen with a light color.
    thread::sleep(Duration::from_millis(500));
    fill_screen_cell(Color::Rgb { r: 230, g: 230, b: 250 });
}
```
## Screen and Path Tests

Trender also provides functions like screen_test_cell() to fill the screen with random colors and d2_path() to draw a line between points. These are useful for testing and demonstrating dynamic graphics.
```
use trender::screen_test_cell;

fn main() {
    // Fill the screen with random colors for a demo.
    screen_test_cell();
}
```
For drawing a 2D path between points:
```
use crossterm::style::Color;
use trender::d2_path;

fn main() {
    // Define a path between two points.
    let points = [((10.0, 5.0), (30.0, 15.0))];
    // Draw the path in green.
    d2_path(&points, Color::Green);
}
```
# API Reference
## Basic Drawing Functions
```real_cell(x: u16, y: u16, color: Color)```
Moves the cursor to the given (x, y) position and draws a “real” pixel (a terminal cell filled with the specified background color). It uses crossterm commands to change colors, disable blinking, and hide the cursor.

```cell(x: u16, y: u16, color: Color)```
Draws a square pixel by calculating two adjacent real cell positions. Use this for drawing “pixels” that are roughly square.

```fill_row_cell(y: u16, color: Color)```
Fills an entire row of the terminal with the given color by drawing square pixels across the terminal width.

```fill_colum_cell(x: u16, color: Color)```
Fills an entire column by drawing two vertical lines of pixels (via two calls to fill_colum_real_cell).

```fill_screen_cell(color: Color)```
Fills the entire terminal screen with the given color. It uses terminal_size() to determine the screen dimensions.

Utility Functions

```clear()```
Clears the terminal by executing the system’s clear command.

```show_curser()```
Re-enables the terminal cursor (previously hidden during drawing).

```thread_sleep_mil(mills: u64)```
A simple wrapper for sleeping a specified number of milliseconds. Useful for pausing between render updates.

## Centering Traits

Trender defines two traits—Center_cell and FCenter_cell—to help compute centered positions for drawing content:

```Center_cell for i64:```
    Provides methods to calculate centered x and y coordinates based on an array of integer values and the current terminal dimensions.

```FCenter_cell for f64:```
    Similar to Center_cell, but for floating-point calculations.

These traits allow you to adjust drawing coordinates so that your content is centered on the screen.
Testing

Trender includes several test functions (compiled only when testing via the #[cfg(test)] attribute):

    ```real_cell_test()```
    Tests drawing individual real cells at specific positions.
    ```cell_test()```
    Tests the square pixel drawing function.
    ``fill_row_cell_test()`` and ``fill_colum_cell_test()``
    Verify that entire rows or columns are properly filled.

These tests clear the terminal before drawing and pause briefly (using thread_sleep_mil) so that you can observe the output during development.
Future Work

    
License: MIT
