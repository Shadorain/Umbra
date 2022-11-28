#![allow(dead_code)]

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use hecs::Bundle;

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
pub enum ElementMode {
    /// Fullscreen mode, only one main element fullscreened at a time
    #[default]
    Monocle,
    /// An element not bound by tile/alignment
    Floating,
    /// Elements bound to a tiling layout
    Tiled,
    /// Holds a static position, cannot move
    Static,
    /// Aligned to a section of the screen horizontally or vertically
    Aligned(VerticalAlign, HorizontalAlign),
}

#[derive(Default)]
pub enum Visibility {
    #[default]
    Visible,
    Invisible,
}

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
pub enum Offset {
    #[default]
    /// (0, 0)
    Zero,
    /// Relative based on parent/layout
    Relative(crate::screen::Point),
    /// Absolute off top left corner of screen
    Absolute(crate::screen::Point),
}

#[derive(Default)]
pub struct Direction {
    /// Top position
    pub t: u16,
    /// Bottom position
    pub b: u16,
    /// Left position
    pub l: u16,
    /// Right position
    pub r: u16,
}

#[derive(Default)]
pub enum Sizing {
    #[default]
    /// Sized by the tiling layout
    Dynamic,
    /// Statically sized, will not change by contraints
    Static(crate::screen::Size),
    /// Resizeable and bound by sizing constraints
    Flexible(crate::screen::Size),
}

#[derive(Default)]
/// A marker indicating an element is interactible.
/// Essentially is included in certain events that non-interactive
/// elements would not be.
pub struct Interactable;

#[derive(Default)]
/// A marker indicating an element can gain focus
pub struct Focusable;

#[derive(Default)]
pub struct Visible(Visibility);

#[derive(Default)]
pub struct Size(Sizing);

#[derive(Default)]
/// Title Component
/// Holds a copy-on-write string for good ol' optimization.
/// Credits for Cow idea: [Ivy Base](https://docs.rs/ivy-base/0.10.3/ivy_base/components/index.html)
pub struct Title(Cow<'static, str>);

impl Title {
    /// Allows any conversion Into Cow<'static, str>
    pub fn new<S: Into<Cow<'static, str>>>(title: S) -> Self {
        Self(title.into())
    }
}
impl Deref for Title {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for Title {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.to_mut()
    }
}

pub type Padding = Direction;
pub type Margin = Direction;

#[derive(Default)]
/// Whether the element will be drawn with a border or not
pub struct Border(BorderStyle);

#[derive(Bundle, Default)]
/// A bundle of typical components styling for an `Element` would utilize
pub struct ElementStyleBundle {
    pub border: Border,
    pub padding: Padding,
    pub margin: Margin,
}

#[derive(Bundle, Default)]
/// A bundle of typical components an `Element` would utilize
pub struct ElementBundle {
    pub visible: Visible,
    pub mode: ElementMode,
    pub offset: Offset,
    pub size: Size,
}
