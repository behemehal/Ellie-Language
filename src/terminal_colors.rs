#[allow(dead_code)]

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

pub fn get_color(selected: Colors) -> String {
    let color_id = match selected {
        Colors::Black => "[30m",
        Colors::Red => "[31m",
        Colors::Green => "[32m",
        Colors::Yellow => "[33m",
        Colors::Blue => "[34m",
        Colors::Magenta => "[35m",
        Colors::Cyan => "[36m",
        Colors::White => "[37m",
        Colors::Reset => "[0m",
    };
    return format!("{}{}", '\u{001b}', color_id);
}
