mod utils;

use regex::Regex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde::Serialize;
use wasm_bindgen::__rt::IntoJsResult;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct RGBValue {
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct ClosestColor {
    name: String,
    color: String,
}

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

pub static COLORS_DATA: [(&str, [u8; 3]); 148] = [
    ("Antiquewhite", [250, 235, 215]),
    ("Aliceblue", [240, 248, 255]),
    ("Aqua", [0, 255, 255]),
    ("Aquamarine", [127, 255, 212]),
    ("Azure", [240, 255, 255]),
    ("Beige", [245, 245, 220]),
    ("Bisque", [255, 228, 196]),
    ("Black", [0, 0, 0]),
    ("Blanchedalmond", [255, 235, 205]),
    ("Blue", [0, 0, 255]),
    ("Blueviolet", [138, 43, 226]),
    ("Brown", [165, 42, 42]),
    ("Burlywood", [222, 184, 135]),
    ("Cadetblue", [95, 158, 160]),
    ("Chartreuse", [127, 255, 0]),
    ("Chocolate", [210, 105, 30]),
    ("Coral", [255, 127, 80]),
    ("Cornflowerblue", [100, 149, 237]),
    ("Cornsilk", [255, 248, 220]),
    ("Crimson", [220, 20, 60]),
    ("Cyan", [0, 255, 255]),
    ("Darkblue", [0, 0, 139]),
    ("Darkcyan", [0, 139, 139]),
    ("Darkgoldenrod", [184, 134, 11]),
    ("Darkgray", [169, 169, 169]),
    ("Darkgreen", [0, 100, 0]),
    ("Darkgrey", [169, 169, 169]),
    ("Darkkhaki", [189, 183, 107]),
    ("Darkmagenta", [139, 0, 139]),
    ("Darkolivegreen", [85, 107, 47]),
    ("Darkorange", [255, 140, 0]),
    ("Darkorchid", [153, 50, 204]),
    ("Darkred", [139, 0, 0]),
    ("Darksalmon", [233, 150, 122]),
    ("Darkseagreen", [143, 188, 143]),
    ("Darkslateblue", [72, 61, 139]),
    ("Darkslategray", [47, 79, 79]),
    ("Darkslategrey", [47, 79, 79]),
    ("Darkturquoise", [0, 206, 209]),
    ("Darkviolet", [148, 0, 211]),
    ("Deeppink", [255, 20, 147]),
    ("Deepskyblue", [0, 191, 255]),
    ("Dimgray", [105, 105, 105]),
    ("Dimgrey", [105, 105, 105]),
    ("Dodgerblue", [30, 144, 255]),
    ("Firebrick", [178, 34, 34]),
    ("Floralwhite", [255, 250, 240]),
    ("Forestgreen", [34, 139, 34]),
    ("Fuchsia", [255, 0, 255]),
    ("Gainsboro", [220, 220, 220]),
    ("Ghostwhite", [248, 248, 255]),
    ("Gold", [255, 215, 0]),
    ("Goldenrod", [218, 165, 32]),
    ("Gray", [128, 128, 128]),
    ("Green", [0, 128, 0]),
    ("Greenyellow", [173, 255, 47]),
    ("Grey", [128, 128, 128]),
    ("Honeydew", [240, 255, 240]),
    ("Hotpink", [255, 105, 180]),
    ("Indianred", [205, 92, 92]),
    ("Indigo", [75, 0, 130]),
    ("Ivory", [255, 255, 240]),
    ("Khaki", [240, 230, 140]),
    ("Lavender", [230, 230, 250]),
    ("Lavenderblush", [255, 240, 245]),
    ("Lawngreen", [124, 252, 0]),
    ("Lemonchiffon", [255, 250, 205]),
    ("Lightblue", [173, 216, 230]),
    ("Lightcoral", [240, 128, 128]),
    ("Lightcyan", [224, 255, 255]),
    ("Lightgoldenrodyellow", [250, 250, 210]),
    ("Lightgray", [211, 211, 211]),
    ("Lightgreen", [144, 238, 144]),
    ("Lightgrey", [211, 211, 211]),
    ("Lightpink", [255, 182, 193]),
    ("Lightsalmon", [255, 160, 122]),
    ("Lightseagreen", [32, 178, 170]),
    ("Lightskyblue", [135, 206, 250]),
    ("Lightslategray", [119, 136, 153]),
    ("Lightslategrey", [119, 136, 153]),
    ("Lightsteelblue", [176, 196, 222]),
    ("Lightyellow", [255, 255, 224]),
    ("Lime", [0, 255, 0]),
    ("Limegreen", [50, 205, 50]),
    ("Linen", [250, 240, 230]),
    ("Magenta", [255, 0, 255]),
    ("Maroon", [128, 0, 0]),
    ("Mediumaquamarine", [102, 205, 170]),
    ("Mediumblue", [0, 0, 205]),
    ("Mediumorchid", [186, 85, 211]),
    ("Mediumpurple", [147, 112, 219]),
    ("Mediumseagreen", [60, 179, 113]),
    ("Mediumslateblue", [123, 104, 238]),
    ("Mediumspringgreen", [0, 250, 154]),
    ("Mediumturquoise", [72, 209, 204]),
    ("Mediumvioletred", [199, 21, 133]),
    ("Midnightblue", [25, 25, 112]),
    ("Mintcream", [245, 255, 250]),
    ("Mistyrose", [255, 228, 225]),
    ("Moccasin", [255, 228, 181]),
    ("Navajowhite", [255, 222, 173]),
    ("Navy", [0, 0, 128]),
    ("Oldlace", [253, 245, 230]),
    ("Olive", [128, 128, 0]),
    ("Olivedrab", [107, 142, 35]),
    ("Orange", [255, 165, 0]),
    ("Orangered", [255, 69, 0]),
    ("Orchid", [218, 112, 214]),
    ("Palegoldenrod", [238, 232, 170]),
    ("Palegreen", [152, 251, 152]),
    ("Paleturquoise", [175, 238, 238]),
    ("Palevioletred", [219, 112, 147]),
    ("Papayawhip", [255, 239, 213]),
    ("Peachpuff", [255, 218, 185]),
    ("Peru", [205, 133, 63]),
    ("Pink", [255, 192, 203]),
    ("Plum", [221, 160, 221]),
    ("Powderblue", [176, 224, 230]),
    ("Purple", [128, 0, 128]),
    ("Rebeccapurple", [102, 51, 153]),
    ("Red", [255, 0, 0]),
    ("Rosybrown", [188, 143, 143]),
    ("Royalblue", [65, 105, 225]),
    ("Saddlebrown", [139, 69, 19]),
    ("Salmon", [250, 128, 114]),
    ("Sandybrown", [244, 164, 96]),
    ("Seagreen", [46, 139, 87]),
    ("Seashell", [255, 245, 238]),
    ("Sienna", [160, 82, 45]),
    ("Silver", [192, 192, 192]),
    ("Skyblue", [135, 206, 235]),
    ("Slateblue", [106, 90, 205]),
    ("Slategray", [112, 128, 144]),
    ("Slategrey", [112, 128, 144]),
    ("Snow", [255, 250, 250]),
    ("Springgreen", [0, 255, 127]),
    ("Steelblue", [70, 130, 180]),
    ("Tan", [210, 180, 140]),
    ("Teal", [0, 128, 128]),
    ("Thistle", [216, 191, 216]),
    ("Tomato", [255, 99, 71]),
    ("Turquoise", [64, 224, 208]),
    ("Violet", [238, 130, 238]),
    ("Wheat", [245, 222, 179]),
    ("White", [255, 255, 255]),
    ("Whitesmoke", [245, 245, 245]),
    ("Yellow", [255, 255, 0]),
    ("yellowgreen", [154, 205, 50]),
];

/// CORE

#[wasm_bindgen]
pub fn closest(color_string: &str) -> JsValue {
    // Parse the color string
    let parsed_color = parse_color(color_string);

    // Find the closest predefined color
    let closest_color = find_color_name(parsed_color);

    // Convert the result to JsValue and return
    serde_wasm_bindgen::to_value(&closest_color).unwrap()
}

#[wasm_bindgen]
pub fn find_color_name(rgb: RGBValue) -> ClosestColor {
    let rgb_obj = RGBValue {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
    };

    let r = rgb_obj.r;
    let g = rgb_obj.g;
    let b = rgb_obj.b;

    let mut closest_gap = u32::MAX;
    let mut closest_color = ClosestColor {
        name: String::from("Unknown"),
        color: String::from("#F00"),
    };

    for (name, predefined) in COLORS_DATA.iter() {
        let gap = distance(&RGBValue { r, g, b }, &RGBValue {
            r: predefined[0],
            g: predefined[1],
            b: predefined[2],
        });

        if gap < closest_gap {
            closest_gap = gap;
            closest_color.name = name.to_string();
            closest_color.color = format!("rgb({},{},{})", predefined[0], predefined[1], predefined[2]);
        }

        if gap == 0 {
            break;
        }
    }

    closest_color
}

#[wasm_bindgen]
pub fn distance(rgb1: &RGBValue, rgb2: &RGBValue) -> u32 {
    let r_diff = (rgb2.r as i32 - rgb1.r as i32).abs() as u32;
    let g_diff = (rgb2.g as i32 - rgb1.g as i32).abs() as u32;
    let b_diff = (rgb2.b as i32 - rgb1.b as i32).abs() as u32;
    let dist = r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;
    dist
}

/// REGEX

fn hex_regex() -> Regex {
    // Regular expression pattern for hex color values
    // Matches hex values with or without the "#" symbol
    Regex::new(r#"^#?([A-Fa-f0-9]{6}(?:[A-Fa-f0-9]{2})?|[A-Fa-f0-9]{3,4})$"#).unwrap()
}

fn rgb_regex() -> Regex {
    // Regular expression pattern for RGB color values
    // Matches RGB values in the format "rgb(R, G, B)" or "rgba(R, G, B, A)"
    Regex::new(r#"^rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d+)\s*)?\)$"#).unwrap()
}

fn hsl_regex() -> Regex {
    // Regular expression pattern for HSL color values
    // Matches HSL values in the format "hsl(H, S%, L%)" or "hsla(H, S%, L%, A)"
    Regex::new(r#"^hsla?\(\s*(\d+)\s*,\s*(\d+%)\s*,\s*(\d+%)\s*(?:,\s*(\d+)\s*)?\)$"#).unwrap()
}

/// PARSE

#[wasm_bindgen]
pub fn parse_color(color_string: &str) -> RGBValue {
    let color = color_string.trim();

    // Check if the color string matches any of the regular expressions
    if hex_regex().is_match(color) {
        let hex_values = parse_hex(color);
        return hex_to_rgb(&hex_values);
    } else if rgb_regex().is_match(color) {
        let rgb_values = parse_rgb(color);
        return get_rgb_values(&rgb_values);
    } else if hsl_regex().is_match(color) {
        let hsl_values = parse_hsl(color);
        return hsl_to_rgb(hsl_values);
    } else {
        // If the color string does not match any of the regular expressions, return an error
        panic!("Invalid color: {}", color);
    }
}

#[wasm_bindgen]
pub fn parse_hex_color(color: &str) -> RGBValue {
    // Assuming color is in the format #RRGGBB or #RRGGBBAA
    let hex_values = &color[1..]; // Exclude the '#' character

    let (red, green, blue, _alpha) = match hex_values.len() {
        6 => (
            u8::from_str_radix(&hex_values[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&hex_values[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&hex_values[4..6], 16).unwrap_or(0),
            255, // Default alpha for 6-digit hex
        ),
        8 => (
            u8::from_str_radix(&hex_values[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&hex_values[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&hex_values[4..6], 16).unwrap_or(0),
            u8::from_str_radix(&hex_values[6..8], 16).unwrap_or(255),
        ),
        _ => return RGBValue { r: 0, g: 0, b: 0 }, // Invalid hex length
    };

    RGBValue { r: red, g: green, b: blue }
}

#[wasm_bindgen]
pub fn parse_rgb_color(color: &str) -> RGBValue {
    // Assuming color is in the format rgb(R, G, B) or rgba(R, G, B, A)
    let values: Vec<&str> = color
        .trim_matches(|c| c == ' ' || c == '(' || c == ')')
        .split(',')
        .collect();

    if values.len() == 3 || values.len() == 4 {
        let red = values[0].trim().parse::<u8>().unwrap_or(0);
        let green = values[1].trim().parse::<u8>().unwrap_or(0);
        let blue = values[2].trim().parse::<u8>().unwrap_or(0);
        let _alpha = if values.len() == 4 {
            values[3].trim().parse::<u8>().unwrap_or(255)
        } else {
            255
        };

        return RGBValue { r: red, g: green, b: blue };
    }

    RGBValue { r: 0, g: 0, b: 0 } // Invalid format
}

/// PARSE COLOR VALUES

pub fn parse_hex(value: &str) -> Vec<String> {
    let hex_pattern = Regex::new(r#"^#?([A-Fa-f0-9]{6}(?:[A-Fa-f0-9]{2})?|[A-Fa-f0-9]{3,4})$"#).unwrap();

    if let Some(captures) = hex_pattern.captures(value) {
        let mut hex = captures[1].to_owned();
        if hex.len() == 3 || hex.len() == 4 {
            // Expand shorthand hex to longhand
            hex = hex.chars().flat_map(|c| std::iter::repeat(c).take(2)).collect();
        }
        // Split the longhand hex into RGB components
        hex.chars().collect::<Vec<char>>().chunks(2)
            .map(|chunk| chunk.iter().collect())
            .collect::<Vec<String>>()
    } else {
        // Return an empty vector if the input value does not match the pattern
        Vec::new()
    }
}

pub fn parse_rgb(rgb_string: &str) -> Vec<String> {
    // Regular expression pattern for parsing RGB color values
    // Matches RGB values in the format "rgb(R, G, B)" or "rgba(R, G, B, A)"
    let rgb_pattern = Regex::new(r#"^rgba?\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})(?:\s*,\s*(\d*\.?\d+))?\s*\)$"#).unwrap();

    if let Some(captures) = rgb_pattern.captures(rgb_string) {
        let mut values = Vec::new();
        for i in 1..=3 {  // Capture R, G, B values
            if let Some(match_str) = captures.get(i).map(|m| m.as_str()) {
                values.push(match_str.to_owned());
            }
        }
        // Optional: Capture Alpha value if present
        if let Some(alpha_match) = captures.get(4) {
            values.push(alpha_match.as_str().to_owned());
        }
        values
    } else {
        Vec::new()
    }
}

pub fn parse_hsl(hsl_string: &str) -> Vec<(u16, u8, u8, u8)> {
    // Updated regular expression pattern for parsing HSL and HSLA color values
    let hsl_pattern = Regex::new(
        r#"hsla?\(\s*(\d+)(deg)?\s*,\s*([\d.]+)%\s*,\s*([\d.]+)%\s*(?:,\s*([\d.]+%?))?\)"#
    ).unwrap();

    if let Some(captures) = hsl_pattern.captures(hsl_string) {
        let hue = captures.get(1).and_then(|m| m.as_str().parse::<u16>().ok()).unwrap_or(0);
        let saturation = captures.get(3).and_then(|m| m.as_str().parse::<u8>().ok()).unwrap_or(0);
        let lightness = captures.get(4).and_then(|m| m.as_str().parse::<u8>().ok()).unwrap_or(0);
        let alpha = match captures.get(5) {
            Some(a) => {
                let alpha_str = a.as_str().replace('%', "");
                let alpha_val = alpha_str.parse::<f32>().unwrap_or(1.0);
                // Convert percentage to 0-255 range if needed
                if a.as_str().contains('%') { (alpha_val * 2.55).round() as u8 } else { (alpha_val * 255.0).round() as u8 }
            },
            None => 255 // Default alpha value
        };

        vec![(hue, saturation, lightness, alpha)]
    } else {
        Vec::new()
    }
}

/// CONVERT

pub fn split_values(raw_values: &str) -> Vec<String> {
    if raw_values.contains(",") {
        raw_values.split(|c| c == ',' || c == '/' || c == '\\')
            .map(|val| val.trim().to_string())
            .collect()
    } else {
        raw_values.split(|c| c == ' ' || c == '/' || c == '\\')
            .filter(|val| !val.is_empty())
            .map(|val| val.trim().to_string())
            .collect()
    }
}

pub fn normalize_degree(value: &str, multiplier: f64) -> f64 {
    let mut angle = value.parse::<f64>().unwrap_or(0.0);
    while angle < 0.0 {
        angle += 360.0;
    }
    while angle > 360.0 {
        angle -= 360.0;
    }
    (angle / 360.0) * multiplier
}

pub fn convert_to_int8(value: &str, multiplier: u8) -> u8 {
    let value = value.trim();
    if value.chars().all(char::is_numeric) {
        value.parse::<u8>().unwrap_or(0)
    } else if value.ends_with('%') {
        ((value.trim_matches('%').parse::<f64>().unwrap_or(0.0) / 100.0) * multiplier as f64) as u8
    } else if value.ends_with("deg") {
        normalize_degree(value.trim_start_matches("deg").trim_end_matches("deg"), multiplier as f64) as u8
    } else {
        0
    }
}

/// HEX

// Function to convert hex values to RGBValue
pub fn hex_to_rgb(hex_values: &[String]) -> RGBValue {
    // Extract RGB values from hex values
    let r = u8::from_str_radix(&hex_values[0], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_values[1], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_values[2], 16).unwrap_or(0);

    RGBValue { r, g, b }
}

/// RGB

// Function to convert RGB values from strings to RGBValue
pub fn get_rgb_values(hex_values: &[String]) -> RGBValue {
    let r = u8::from_str_radix(&hex_values[0], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_values[1], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_values[2], 16).unwrap_or(0);

    RGBValue { r, g, b }
}

/// HSL

// Function to convert HSL to RGB
pub fn hsl_to_rgb(hsl: Vec<(u16, u8, u8, u8)>) -> RGBValue {
    let ( hue, saturation, lightness, alpha ) = hsl[0];

    // Convert HSL to RGB (placeholder values, replace with actual conversion logic)
    let h = hue as f64 / 60.0;
    let s = saturation as f64 / 100.0;
    let l = lightness as f64 / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        0.0..=1.0 => (c, x, 0.0),
        1.0..=2.0 => (x, c, 0.0),
        2.0..=3.0 => (0.0, c, x),
        3.0..=4.0 => (0.0, x, c),
        4.0..=5.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    RGBValue { r, g, b }
}
