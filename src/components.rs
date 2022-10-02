#![allow(dead_code)]

use hecs::Bundle;

use crate::screen::Point;

struct Renderable;
struct Tileable;
struct Resizable;

pub enum Interactability {
    Clickable,
    Focusable,
}

#[derive(Default)]
pub enum Visibility {
    #[default]
    Visible,
    Invisible,
}

#[derive(Default)]
struct Visible(Visibility);

#[derive(Default)]
struct Geometry(Point);

#[derive(Default)]
struct Title(&'static str);

#[derive(Default)]
pub enum VerticalAlign {
    Top,
    #[default]
    Center,
    Bottom,
}

#[derive(Default)]
pub enum HorizontalAlign {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Default)]
struct Direction {
    /// Top position
    t: u16,
    /// Bottom position
    b: u16,
    /// Left position
    l: u16,
    /// Right position
    r: u16,
}

type Padding = Direction;
type Margin = Direction;

#[derive(Default)]
pub enum BorderStyle {
    #[default]
    None,
    Single,
    Rounded,
    Double,
    Thick,
}

#[derive(Default)]
struct Alignment(VerticalAlign, HorizontalAlign);

#[derive(Default)]
struct Border(BorderStyle);

#[derive(Bundle, Default)]
pub struct ElementStyleBundle {
    border: Border,
    padding: Padding,
    margin: Margin,
}

#[derive(Bundle, Default)]
struct ElementBundle {
    visible: Visibility,
    geometry: Point,
}

