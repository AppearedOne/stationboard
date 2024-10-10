use iced::Color;

pub fn line_color(num: &str) -> Color {
    match num.trim() {
        "31" => from_rgb(164.0, 162.0, 198.0),
        "704" => from_rgb(141.0, 34.0, 78.0),
        "701" => from_rgb(0.0, 141.0, 197.0),
        "703" => from_rgb(255.0, 193.0, 3.0),
        "91" => from_rgb(255.0, 255.0, 255.0),
        _ => from_rgb(255.0, 0.0, 0.0),
    }
}

fn from_c_value(c: f32) -> f32 {
    c / 255.0
}

pub fn from_rgb(r: f32, g: f32, b: f32) -> iced::Color {
    Color::from_rgb(from_c_value(r), from_c_value(g), from_c_value(b))
}
