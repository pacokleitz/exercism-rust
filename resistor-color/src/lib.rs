use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, IntEnum, IntoEnumIterator)]
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

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut values = ResistorColor::into_enum_iter()
        .map(|x| x.int_value())
        .collect::<Vec<usize>>();
    values.sort();
    values
        .into_iter()
        .map(|x| ResistorColor::from_int(x).unwrap())
        .collect()
}
