//! This is a simple(ish) graphics library that runs in the terminal.
//! Repository: https://github.com/Forged-Dyslexic/Trender
//! Author: https://github.com/QuantumLoopHole

#![allow(clippy::type_complexity)]

use crossterm::cursor::{DisableBlinking, Hide, Show};
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};
use rand::{rng, Rng};
use std::cmp::Ordering;
use std::process::Command;
use std::thread;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

/// Creates an RGB color.
///
/// # Examples
///
/// ``` e
/// let blue = trender::rgb(0, 0, 255);
/// ```
///
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}

/// Shows the cursor in the terminal.
pub fn show_curser() {
    let mut stdout = std::io::stdout();
    execute!(stdout, Show).unwrap();
}

/// Draws a "real" pixel in the terminal.
///
/// Because the terminal primarily uses text, modifying the "character slot"
/// results in a vertical rectangle. This function sets a background color in
/// a single terminal cell, effectively acting as a pixel.
///
/// # Examples
///
/// ```
/// trender::real_cell(1, 1, trender::rgb(0, 0, 255));
/// ```
pub fn real_cell(x: u16, y: u16, color: Color) {
    execute!(
        std::io::stdout(),
        cursor::MoveTo(x, y),
        SetBackgroundColor(color),
        Print(" "),
        DisableBlinking,
        ResetColor,
        Hide,
    )
    .unwrap();
}

/// Draws a square pixel on the terminal.
///
/// The terminal uses rectangular "pixels" which results in the need to draw two
/// adjacent cells to simulate a square.
///
/// # Examples
///
/// ```
/// // Draws a red square pixel at position (1,1)
/// trender::cell(1, 1, Color::Red);
/// ```
pub fn cell(x: u16, y: u16, color: Color) {
    let doubled_x = x.checked_mul(2).unwrap_or(u16::MAX);
    let first_pixel_x = doubled_x.saturating_sub(1);
    let second_pixel_x = doubled_x;

    real_cell(first_pixel_x, y, color);
    real_cell(second_pixel_x, y, color);
}

/// Fills an entire row of the terminal with a specific color.
///
/// # Examples
///
/// ```
/// trender::fill_row_cell(5, trender::rgb(255, 255, 255));
/// ```
pub fn fill_row_cell(y: u16, color: Color) {
    let size = terminal_size();
    if let Some((Width(w), Height(_))) = size {
        let width = w;
        for x in 0..(width / 2) {
            cell(x, y, color);
        }
    } else {
        println!("Unable to get terminal size");
    }
}

/// Fills a column of the terminal with a specific color using "real" cells.
///
/// # Examples
///
/// ```
/// trender::fill_colum_real_cell(5, trender::rgb(255, 255, 255));
/// ```
pub fn fill_colum_real_cell(x: u16, color: Color) {
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        for y in 0..h {
            real_cell(x, y, color);
        }
    } else {
        println!("Unable to get terminal size");
    }
}

/// Fills a column of the terminal with a specific color using square pixels.
///
/// # Examples
///
/// ```
/// trender::fill_colum_cell(5, trender::rgb(255, 255, 255));
/// ```
pub fn fill_colum_cell(x: u16, color: Color) {
    let doubled_x = x.checked_mul(2).unwrap_or(u16::MAX);
    let first_pixel_x = doubled_x.saturating_sub(1);
    let second_pixel_x = doubled_x;
    fill_colum_real_cell(first_pixel_x, color);
    fill_colum_real_cell(second_pixel_x, color);
}

/// Fills the entire terminal screen with a specific color.
///
/// # Examples
///
/// ```
/// trender::fill_screen_cell(trender::rgb(0, 0, 0));
/// ```
pub fn fill_screen_cell(color: Color) {
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        for y in 0..h {
            fill_row_cell(y, color);
        }
    } else {
        println!("Unable to get terminal size");
    }
}

/// Fills the screen with random colors.
///
/// # Examples
///
/// ```
/// trender::screen_test_cell();
/// ```
pub fn screen_test_cell() {
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        for y in 1..h {
            if let Some((Width(w), Height(_))) = size {
                for x in 1..(w / 2) {
                    let mut rng = rng();
                    let random_r: u8 = rng.random_range(0..=255);
                    let random_g: u8 = rng.random_range(0..=255);
                    let random_b: u8 = rng.random_range(0..=255);
                    cell(x, y, rgb(random_r, random_g, random_b));
                }
            } else {
                println!("Unable to get terminal size");
                return;
            }
        }
    } else {
        println!("Unable to get terminal size");
    }
}

/// Fills a specific row of the terminal with random colors.
///
/// # Examples
///
/// ```
/// trender::row_test_cell(5);
/// ```
pub fn row_test_cell(y: u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        for x in 0..(w / 2) {
            let mut rng = rng();
            let random_r: u8 = rng.random_range(0..=255);
            let random_g: u8 = rng.random_range(0..=255);
            let random_b: u8 = rng.random_range(0..=255);
            cell(x, y, rgb(random_r, random_g, random_b));
        }
    } else {
        println!("Unable to get terminal size");
    }
}

/*
* NOTICE ABOUT THE CENTER TRAIT
* The center trait is currently under construction. Its intent is to center the rendered image.
*
* At this time, feel free to use .center_cell() in your scripts, but keep in mind that its functionality will change.
*/
/// Centers the given coordinates by adding offsets from the provided `array`.
///
/// # Arguments
///
/// * `xy` - A tuple-like array `[x, y]` representing the initial coordinates to be centered.
///
/// # Returns
///
/// Returns a new array `[centered_x, centered_y]` where the x and y coordinates are adjusted
/// by the corresponding values from the `array`.
///
/// # Example
///
/// ```
/// ```
pub fn center(xy: [i64; 2]) -> [i64; 2] {
    let x: i64 = 1; // Init the variables
    let y: i64 = 1;

    // Get centered coordinates
    let centered_x = x.center_x_cell(&xy);
    let centered_y = y.center_y_cell(&xy);
    // Return the new coordinates
    [centered_x, centered_y]
}

/// A trait to provide center functionality for integer values.
pub trait CenterCell {
    /// Centers a given array on the x-axis.
    fn center_x_cell(&self, array: &[i64]) -> i64;
    /// Centers a given array on the y-axis.
    fn center_y_cell(&self, array: &[i64]) -> i64;
}

/// A trait to provide center functionality for floating point values.
pub trait FCenterCell {
    /// Centers a given array on the x-axis.
    fn fcenter_x_cell(&self, array: &[f64]) -> f64;
    /// Centers a given array on the y-axis.
    fn fcenter_y_cell(&self, array: &[f64]) -> f64;
}

impl CenterCell for i64 {
    fn center_x_cell(&self, array: &[i64]) -> i64 {
        let size = terminal_size();
        if let Some((Width(w), _)) = size {
            let range = match (array.iter().max(), array.iter().min()) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0,
            };
            (*self + (w / 4) as i64) - (range / 2) // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }

    fn center_y_cell(&self, array: &[i64]) -> i64 {
        let size = terminal_size();
        if let Some((_, Height(h))) = size {
            let range = match (array.iter().max(), array.iter().min()) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0,
            };
            (*self + (h / 2) as i64) - (range / 2) // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }
}

impl FCenterCell for f64 {
    fn fcenter_x_cell(&self, array: &[f64]) -> f64 {
        let size = terminal_size();
        if let Some((Width(w), _)) = size {
            let range = match (
                array
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                array
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
            ) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0.0,
            };
            (*self + (w / 4) as f64) - (range / 2.0) // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }

    fn fcenter_y_cell(&self, array: &[f64]) -> f64 {
        let size = terminal_size();
        if let Some((_, Height(h))) = size {
            let range = match (
                array
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                array
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
            ) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0.0,
            };
            (*self + (h / 2) as f64) - (range / 2.0) // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }
}

/// Clears the terminal screen.
///
/// # Examples
///
/// ```
/// trender::clear();
/// ```
pub fn clear() {
    let _ = Command::new("clear").status();
}

/// Maps out 2D coordinates by converting floating-point values to terminal positions.
///
/// # Examples
///
/// ```
/// let coords = vec![(0.0, 0.0), (1.5, 2.5)];
/// trender::d2_map_cell(&coords);
/// ```
pub fn d2_map_cell(cords: &[(f64, f64)]) {
    let count = cords.len();

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for &(first, second) in cords.iter() {
        first_numbers.push(first);
        second_numbers.push(second);
    }

    for i in 0..count {
        let x = (first_numbers[i] + 1.0).floor() as u16;
        let y = (second_numbers[i] + 1.0).floor() as u16;

        cell(x, y, rgb(100, 100, 255));
    }
}

/// Draws a 2D line (path) between pairs of points on the terminal.
///
/// The function interpolates points between two endpoints and draws them using the
/// `cell` function.
///
/// # Examples
///
/// ```
/// let points = &[((1.0, 1.0), (10.0, 10.0))];
/// trender::d2_path(points, trender::rgb(255, 0, 0));
/// ```
pub fn d2_path(points: &[((f64, f64), (f64, f64))], color: Color) {
    for points in points {
        let a_x = points.0 .0;
        let a_y = points.0 .1;

        let b_x = points.1 .0;
        let b_y = points.1 .1;

        let distance = ((b_x - a_x).powi(2) + (b_y - a_y).powi(2)).sqrt();

        let min_steps = 1000;
        let max_steps = 100000;
        let min_distance = 10.0;
        let max_distance = 100.0;

        let steps = ((distance - min_distance) / (max_distance - min_distance)
            * ((max_steps - min_steps) as f64)
            + (min_steps as f64)) as usize;

        let step_size = distance / steps as f64;

        let steps_x = (b_x - a_x) / distance * step_size;
        let steps_y = (b_y - a_y) / distance * step_size;

        cell(a_x as u16, a_y as u16, color);

        let mut current_point = (a_x, a_y);

        for _ in 0..steps {
            current_point.0 += steps_x;
            current_point.1 += steps_y;
            cell(current_point.0 as u16, current_point.1 as u16, color);
        }
    }
}

// Shapes

/// Draws a square given bottom-left and top-right coordinates with specified line and fill colors.
///
/// **Note:** This function is a placeholder and currently does not implement any drawing logic.
pub fn square(bl: [i32; 2], rt: [i32; 2], line_color: Color, fill_color: Color) {
    let _a = bl;
    let _b = rt;
    let _c = line_color;
    let _d = fill_color;
}

// Just some ease of use functionality

/// Sleeps the current thread for a specified number of milliseconds.
///
/// # Examples
///
/// ```
/// trender::thread_sleep_mil(100);
/// ```
pub fn thread_sleep_mil(mills: u64) {
    thread::sleep(Duration::from_millis(mills));
}

/// Testing module: Contains tests for the terminal graphics functions.
#[cfg(test)]
mod tests {

    use super::*;

    /// Test for real_cell and fill_colum_cell functions.
    #[test]
    fn real_cell_test() {
        clear();

        real_cell(1, 1, Color::DarkBlue);
        real_cell(0, 1, Color::Red);
        thread_sleep_mil(100);
        clear();

        fill_colum_cell(1, Color::DarkBlue);
        fill_colum_cell(2, Color::Red);
        thread_sleep_mil(500);
        clear();

        show_curser();
    }

    /// Test for the cell function.
    #[test]
    fn cell_test() {
        clear();
        cell(1, 1, Color::DarkBlue);
        cell(2, 1, Color::Red);
        cell(2, 2, Color::DarkBlue);
        cell(3, 2, Color::Red);
        thread_sleep_mil(1000);
        clear();
        show_curser();
    }

    /// Test for the fill_row_cell function.
    #[test]
    fn fill_row_cell_test() {
        clear();
        fill_row_cell(1, Color::DarkBlue);
        fill_row_cell(2, Color::Red);
        thread_sleep_mil(500);
        clear();
        show_curser();
    }

    /// Test for the fill_colum_real_cell function.
    #[test]
    fn fill_colum_real_cell_test() {
        clear();
        fill_colum_real_cell(1, Color::DarkBlue);
        fill_colum_real_cell(2, Color::Red);
        thread_sleep_mil(500);
        clear();
        show_curser();
    }

    /// Test for the fill_colum_cell function.
    #[test]
    fn fill_colum_cell_test() {
        clear();
        fill_colum_cell(1, Color::DarkBlue);
        fill_colum_cell(2, Color::Red);
        thread_sleep_mil(500);
        clear();
        show_curser();
    }

    #[test]
    fn fill_screen_test() {
        clear();
        fill_screen_cell(rgb(255, 255, 255));
        thread_sleep_mil(500);
        clear();
        show_curser();
    }

    #[test]
    fn center_test() {
        println!("{:?}", center([1, 1]));
    }

    /// Test the centering script
    #[test]
    fn center_cell_test() {
        clear();
        let centered = center([1, 1]);
        println!("{:?}", centered);
        cell(centered[0] as u16, centered[1] as u16, rgb(255, 255, 255));
        thread_sleep_mil(5000);
        clear();
        show_curser();
    }
}
