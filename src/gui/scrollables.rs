
use iced::scrollable;

pub struct Variant {
    pub title: &'static str,
    pub state: scrollable::State,
    pub scrollbar_width: Option<u16>,
    pub scrollbar_margin: Option<u16>,
    pub scroller_width: Option<u16>,
}

impl Variant {
    pub fn default() -> Self {
        Self {
            title: "Default Scrollbar",
            state: scrollable::State::new(),
            scrollbar_width: None,
            scrollbar_margin: None,
            scroller_width: None,
        }
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            Variant::default(),
            Self {
                title: "Slimmed & Margin",
                state: scrollable::State::new(),
                scrollbar_width: Some(4),
                scrollbar_margin: Some(3),
                scroller_width: Some(4),
            },
            Self {
                title: "Wide Scroller",
                state: scrollable::State::new(),
                scrollbar_width: Some(4),
                scrollbar_margin: None,
                scroller_width: Some(10),
            },
            Self {
                title: "Narrow Scroller",
                state: scrollable::State::new(),
                scrollbar_width: Some(10),
                scrollbar_margin: None,
                scroller_width: Some(4),
            },
        ]
    }
}