//! This is a simple(ish) graphics library that runs in the terminal.

use std::process::Command;
use rand::Rng;
use crossterm::{cursor, execute, style::{Color, Print, ResetColor, SetBackgroundColor}};
use crossterm::cursor::{DisableBlinking,Hide};
use terminal_size::{terminal_size, Height, Width};
use Color::Rgb;
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
    //! ```fill_row(5, Rgb { r: (255), g: (255), b: (255) });```
    //! 
    //! Utilizing the [pixel][pixel] function to draw a row of pixels.
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
    //! Fills row of terminal with a specific color
    //! -----
    //! ```fill_colum(5, Rgb { r: (255), g: (255), b: (255) });```
    //! 
    //! Utilizing the [pixel][pixel] function to draw a colum of pixels.
    
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
    //! Fills the screen of the terminal with a specific color
    //! -----
    //! ```fill_screen(Rgb { r: (255), g: (255), b: (255) });```
    //! 
    //! Utilizing the [pixel][pixel] function to draw a colum of pixels.
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
    //! Fills the screen with random colors.
    //! -----
    //! ```screen_test()```
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

pub fn row_test(y:u16){
    //! Fills row of terminal with random colors
    //! -----
    //! ```fill_row(5);```
    //! 
    //! Utilizing the [pixel][pixel] function to draw a row of pixels.

    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        let width = w;
        for x in 0..(width/2) {
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

/*
* NOTICE ABOUT THE CENTER TRAIT
* The center trait is currently under construction. It's intent is to center the rendered image.
*
* At this time, feel free to use .center() in your scripts, but keep in mind that it's functionality will change. 
*
*/
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
    //! clears the terminal
    //! -----
    //! Call ```clear()``` to wipe the terminal clear!
    let _ = Command::new("clear").status();
}

pub fn d2_map(cords: &[(f64, f64)]) {
    //! Maps out set values in a 2D space

    let count = cords.len();

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for &(first, second) in cords.iter() {
        first_numbers.push(first);
        second_numbers.push(second);
    }

    for i in 0..count {
        let x;
        let y;

        x = (first_numbers[i] + 1.0).floor() as u16;
        y = (second_numbers[i] + 1.0).floor() as u16;

        pixel(x, y, Rgb { r: 100, g: 100, b: 255 });
    }
}


pub fn d2_path(points: &[((f64,f64),(f64,f64))], color: Color) {
    //! Draws a 2d line on the terminal
    //! -----
    //! This draws a path from 2 points utilizing the pixel function
    //! 
    //! ```d2_path(points: &[((1,0,1.0),(5.0,5.0)),((5,0,5.0),(2.0,5.0))])```

    for points in points{
        let a_x = points.0.0;
        let a_y = points.0.1;
    
        let b_x = points.1.0;
        let b_y = points.1.1;
    
    
        let distance = ((b_x - a_x).powi(2) + (b_y - a_y).powi(2)).sqrt();
    
        let min_steps = 1000; 
        let max_steps = 100000; 
        let min_distance = 10.0; 
        let max_distance = 100.0; 
        
        let steps = ((distance - min_distance) / (max_distance - min_distance) * ((max_steps - min_steps) as f64) + (min_steps as f64)) as usize;
        
        
        let step_size = distance / steps as f64;
    
        let steps_x = (b_x - a_x) / distance * step_size;
        let steps_y = (b_y - a_y) / distance * step_size;
        
        
        pixel(a_x as u16, a_y as u16, color);
        
    
        let mut current_point = (a_x, a_y);
    
        for _ in 0..steps {
            current_point.0 += steps_x;
            current_point.1 += steps_y;
            pixel(current_point.0 as u16, current_point.1 as u16, color);
        }
    }
}


struct Camera {
    // This portion of Trender is still Underconstruction.
    position: (f64, f64, f64),
    direction: (f64, f64, f64),
    fov: f64, // Field of view in degrees
}

impl Camera {
    // This portion of Trender is still Underconstruction.
    fn project_point(&self, point: (f64, f64, f64)) -> (f64, f64) {
        let (x, y, z) = point;
        
        let relative_x = x - self.position.0;
        let relative_y = y - self.position.1;
        let relative_z = z - self.position.2;
        
        // Calculate the dot product of the direction vector and the vector from the camera to the point
        let dot_product = self.direction.0 * relative_x + self.direction.1 * relative_y + self.direction.2 * relative_z;
        
        // Apply perspective projection
        let projected_x = (self.fov / 2.0).tan() * relative_x / dot_product;
        let projected_y = (self.fov / 2.0).tan() * relative_y / dot_product;
        
        // Return the projected 2D coordinates
        (projected_x, projected_y)
    }
}
