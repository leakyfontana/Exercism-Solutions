use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::fmt;

#[repr(usize)]
#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Copy, Clone)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow"),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    let num = 0..10;

    if !num.contains(&value) {
        return "value out of range".to_string()
    }

    ResistorColor::from_int(value).unwrap().to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    let mut color: Vec<ResistorColor> = vec![ResistorColor::Black; ResistorColor::VARIANT_COUNT];
    
    for r in ResistorColor::into_enum_iter() {
        let idx = color_to_value(r);
        color[idx] = r;
    }

    color
}
