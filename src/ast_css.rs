pub struct StyleSheet {
    pub blocks: Vec<Block>,
}
#[derive(Debug)]
pub struct Block {
    pub selector: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            selector: vec![],
            declarations: vec![],
        }
    }
    pub fn set_selector(self: &mut Self, input: &str) {
        // todo
    }
}
#[derive(Debug)]
pub struct Selector {
    element: Option<String>,
    class: Option<String>,
    id: Option<String>,
    attribute: Option<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub property: CSSProperty,
    pub value: Value,
}

#[derive(Debug)]
pub enum CSSProperty {
    Color,
    BackgroundColor,
    Margin,
    Padding,
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Color,
    Length(f32, Unit),
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
