use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
#[repr(u32)]
#[derive(Debug, PartialEq, Sequence, IntEnum, Clone, Copy)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let cast: Result<ResistorColor, int_enum::IntEnumError<ResistorColor>> =
        ResistorColor::from_int(value);
    match cast {
        Ok(x) => format!("{:?}", x),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
