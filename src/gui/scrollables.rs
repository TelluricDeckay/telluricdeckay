use iced::{scrollable, Point};

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
            state: {
                let mut s = scrollable::State::new();
                // Horrible hack to make the scrollbar scroll to bottom.
                // TODO: fix this when proper patch is added to iced.
                // See: https://github.com/hecrj/iced/pull/607#issuecomment-742818666
                // s = unsafe {
                //     let mut tmp = std::mem::transmute::<_, (Option<f32>, Option<Point>, f32)>(s);
                //     tmp.2 = 999999.0;
                //     std::mem::transmute::<_, scrollable::State>(tmp)
                // };
                s.snap_to(1.0);
                s
            },
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
