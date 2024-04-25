pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


use std::process::Command;
use rand::Rng;
use crossterm::{cursor, execute, style::{Color, Print, ResetColor, SetBackgroundColor}};
use crossterm::cursor::{DisableBlinking,Hide};
use terminal_size::{terminal_size, Height, Width};
use Color::Rgb;
use serde_json::Value;
use std::cmp::Ordering;

pub fn real_pixel(x: u16, y: u16, color: Color) {
    //! Draw a "real" pixel
    //! 
    //! Because the terminal primarily uses text, modifying the "character slot" (or whatever it is called) results in a vertical rectangle. I call this a real pixel.
    //! 
    //! To draw a blue real pixel: ```draw::real_pixel```
    //! 
    //! This is the base of the whole graphics system for now. I will be working on a way to get more definition in the terminal.
    //! I am looking into custom characters.
    //! 
    //! 
    execute!(
        std::io::stdout(),
        cursor::MoveTo(x, y),
        SetBackgroundColor(color),
        Print(" "),
        DisableBlinking,
        ResetColor,
        Hide,
    ).unwrap();
}

pub fn pixel(x: u16, y: u16, color: Color) {
    //! Draw a square pixel on the terminal
    //! -----
    //! The terminal uses rectangular "pixels" which results in the need to draw 2 rectangles side by side to create a square.
    //! To make a custom RGB value, use ```Rgb(r,g,b)```
    //! 
    //! for example: 
    //! 
    //! ```draw::pixel(1,1, Rgb {255, 0, 0});``` for a perfect **RED**
    
    let doubled_x = x.checked_mul(2).unwrap_or(u16::MAX); 
    let first_pixel_x = doubled_x.checked_sub(1).unwrap_or(0);
    let second_pixel_x = doubled_x; 

    real_pixel(first_pixel_x, y, color);
    real_pixel(second_pixel_x, y, color);
}

pub fn fill_row(y: u16, color: Color) {
    //! Fills row of terminal with a specific color
    //! -----
    //! Utilizing the [pixel][pixel] function
    let size = terminal_size();
    if let Some((Width(w), Height(_))) = size {
        let width = w;
        for x in 0..(width/2) {
            pixel(x, y, color);
        }
    } else {
        println!("Unable to get terminal size");
        return;
    }

    
    
}


pub fn fill_colum(x: u16, color: Color){
    
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        let height = h;
        for y in 0..height {
            real_pixel(x, y, color);
        }
    } else {
        println!("Unable to get terminal size");
        return;
    }

    
}

pub fn fill_screen(color: Color){
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        for y in 0..h {
            fill_row(y, color);
        }
    } else {
        println!("Unable to get terminal size");
        return;
    }
}


pub fn screen_test(){
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        let height = h;
        

        for y in 1..height {

            if let Some((Width(w), Height(_))) = size {
                let width = w;
                for x in 1..(width/2) {
                    let mut rng = rand::thread_rng();
                    let random_r: u8 = rng.gen_range(0..=255);
                    let random_g: u8 = rng.gen_range(0..=255);
                    let random_b: u8 = rng.gen_range(0..=255);
                    pixel(x, y, Rgb { r: (random_r), g: (random_g), b: (random_b) });
                }
            } else {
                println!("Unable to get terminal size");
                return;
            }
        }
    } else {
        println!("Unable to get terminal size");
        return;
    }
}

pub fn row_test(){

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        let height = h;
        let width = w;
        for x in 0..(width/2) {
            let mut rng = rand::thread_rng();
            let random_r: u8 = rng.gen_range(0..=255);
            let random_g: u8 = rng.gen_range(0..=255);
            let random_b: u8 = rng.gen_range(0..=255);
            pixel(x, 0, Rgb { r: (random_r), g: (random_g), b: (random_b) });
    }
    } else {
        println!("Unable to get terminal size");
        return;
    }

        

}

pub trait Center {
    fn center_x(&self, array: &[i64]) -> i64;
    fn center_y(&self, array: &[i64]) -> i64;
}

pub trait FCenter {
    fn fcenter_x(&self, array: &[f64]) -> f64;
    fn fcenter_y(&self, array: &[f64]) -> f64;
}

impl Center for i64 {
    fn center_x(&self, array: &[i64]) -> i64 {
        let size = terminal_size();
        if let Some((Width(w), _)) = size {
            let range = match (array.iter().max(), array.iter().min()) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0,
            };
            (*self + (w / 4) as i64) - (range / 2) as i64 // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }

    fn center_y(&self, array: &[i64]) -> i64 {
        let size = terminal_size();
        if let Some((_, Height(h))) = size {
            let range = match (array.iter().max(), array.iter().min()) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0,
            };
            (*self + (h / 2) as i64) - (range / 2) as i64 // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }
}


impl FCenter for f64 {
    fn fcenter_x(&self, array: &[f64]) -> f64 {
        let size = terminal_size();
        if let Some((Width(w), _)) = size {
            let range = match (array.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)), array.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))) {
                (Some(max_val), Some(min_val)) => max_val - min_val,
                _ => 0.0,
            };
            (*self + (w / 4) as f64) - (range / 2.0) // Adjusted for range
        } else {
            println!("Unable to get terminal width");
            *self
        }
    }

    fn fcenter_y(&self, array: &[f64]) -> f64 {
        let size = terminal_size();
        if let Some((_, Height(h))) = size {
            let range = match (array.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)), array.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))) {
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




pub fn clear(){
    let _ = Command::new("clear").status();
}

pub fn d2_map(json_str: &str, centered: bool) {
    //! Maps out set values in a 
    if let Ok(v) = serde_json::from_str::<Value>(json_str) {
        if let Some(vert) = v["object1"]["Vert"].as_object() {
            let count = vert.len();

            let mut first_numbers = Vec::new();
            let mut second_numbers = Vec::new();

            for (_, arr) in vert.iter() {
                if let Some(numbers) = arr.as_array() {
                    if let (Some(first), Some(second)) = (
                        numbers.get(0).and_then(|v| v.as_i64()),
                        numbers.get(1).and_then(|v| v.as_i64()),
                    ) {
                        first_numbers.push(first);
                        second_numbers.push(second);
                    }
                }
            }

            for i in 0..count {
                let mut x = (first_numbers[i]).try_into().unwrap();
                let mut y = (second_numbers[i]).try_into().unwrap();

                if centered {
                    x = first_numbers[i].center_x(&first_numbers).try_into().unwrap();
                    y = second_numbers[i].center_y(&second_numbers).try_into().unwrap();
                }
                else{
                    x = (first_numbers[i] +1).try_into().unwrap();
                    y = (second_numbers[i] +1).try_into().unwrap();
                }

                pixel(x, y, Rgb { r: (100), g: (100), b: (255) });
            }
        }
        else{
            clear();
            print!("object1 Vert is not an object");
        }
    }

}

pub fn d2_path(a_x: f64, a_y: f64, b_x: f64, b_y: f64, color: Color, centered: bool) {
    let distance = ((b_x - a_x).powi(2) + (b_y - a_y).powi(2)).sqrt();

    let steps = 10000; // Adjust this value for smoother or rougher drawing

    let step_size = distance / steps as f64;

    let steps_x = (b_x - a_x) / distance * step_size;
    let steps_y = (b_y - a_y) / distance * step_size;

    if centered {
        pixel(a_x.fcenter_x(&[a_x, b_x]) as u16, a_y.fcenter_y(&[a_y, b_y]) as u16, color);
    } else {
        pixel(a_x as u16, a_y as u16, color);
    }

    let mut current_point = (a_x, a_y);

    for _ in 0..steps {
        current_point.0 += steps_x;
        current_point.1 += steps_y;

        if centered {
            pixel(current_point.0.fcenter_x(&[a_x, b_x]) as u16, current_point.1.fcenter_y(&[a_y, b_y]) as u16, color);
        } else {
            pixel(current_point.0 as u16, current_point.1 as u16, color);
        }
    }
    
}

fn calculate_angle(start: (f64, f64), end: (f64, f64)) -> f64 {
    let delta_x = end.0 - start.0;
    let delta_y = end.1 - start.1;
    let angle_radians = delta_y.atan2(delta_x);
    let angle_degrees = angle_radians.to_degrees();
    let rounded_angle_degrees = (angle_degrees / 45.0).round() * 45.0;
    rounded_angle_degrees
}

fn find_closest_point(target_point: (f64, f64), points: [(f64, f64); 8]) -> (f64, f64) {
    let mut min_distance = f64::INFINITY;
    let mut closest_point = points[0];

    for &point in points.iter() {
        let distance = ((point.0 - target_point.0).powi(2) + (point.1 - target_point.1).powi(2)).sqrt();
        if distance < min_distance {
            min_distance = distance;
            closest_point = point;
        }
    }

    closest_point
}

