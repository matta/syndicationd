use std::{str::FromStr, sync::OnceLock};

use ratatui::style::Color;
use synd_feed::types::{Category, Requirement};

use crate::{
    application::InFlight,
    config::{Categories, Icon, IconColor},
    ui::theme::Theme,
};

pub mod components;
pub mod extension;
pub mod theme;
pub mod widgets;

mod icon;
pub(crate) use icon::icon;

pub const UNKNOWN_SYMBOL: &str = "-";
pub const TABLE_HIGHLIGHT_SYMBOL: &str = " ";
pub const DEFAULT_REQUIREMNET: Requirement = Requirement::Should;

pub fn default_category() -> &'static Category<'static> {
    static DEFAULT_CATEGORY: OnceLock<Category<'static>> = OnceLock::new();

    DEFAULT_CATEGORY.get_or_init(|| Category::new("default").unwrap())
}

pub fn default_icon() -> &'static Icon {
    static DEFAULT_ICON: OnceLock<Icon> = OnceLock::new();

    DEFAULT_ICON.get_or_init(|| {
        Icon::new("󰎞").with_color(IconColor::new(Color::from_str("dark gray").unwrap()))
    })
}

pub struct Context<'a> {
    pub theme: &'a Theme,
    pub in_flight: &'a InFlight,
    pub categories: &'a Categories,
}
