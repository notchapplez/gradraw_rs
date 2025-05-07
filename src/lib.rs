use colors_transform::{Color, Rgb};

pub fn gradient_print(start_color: String, end_color: String) {
    let mut correct_color_start: String = String::new();
    let mut correct_color_end: String = String::new();
    
    if start_color.starts_with("#") {
        correct_color_start = start_color[1..].to_string();
        
        if end_color.starts_with("#") {
            correct_color_end = end_color[1..].to_string();
        }
    }

    let starts_with_valid_hex = correct_color_start.chars().next().map_or(false, |c| {
        matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
    });
    
    println!("{}", starts_with_valid_hex);
}