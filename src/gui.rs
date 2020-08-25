use crate::config;
use crate::game;

use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text,
    TextInput,
};

pub struct Gui {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
    debug: bool,
    config_data: config::Data,
    game_state: game::Game,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Gui {
        Gui {
            steps: Steps::new(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
            debug: false,
            config_data: config::get(),
            game_state: game::Game::new(),
        }
    }

    fn title(&self) -> String {
        format!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps
                    .update(step_msg, &mut self.debug, &mut self.game_state);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Gui {
            steps,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(
                steps
                    .view(self.debug, &self.config_data)
                    .map(Message::StepMessage),
            )
            .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Radio { selection: None },
                Step::GameSetup,
                Step::Slider {
                    state: slider::State::new(),
                    value: 50,
                },
                Step::RowsAndColumns {
                    layout: Layout::Row,
                    spacing_slider: slider::State::new(),
                    spacing: 20,
                },
                Step::Text {
                    size_slider: slider::State::new(),
                    size: 30,
                    color_sliders: [slider::State::new(); 3],
                    color: Color::BLACK,
                },
                Step::Scrollable,
                Step::TextInput {
                    value: String::new(),
                    is_secure: false,
                    state: text_input::State::new(),
                },
                Step::Debugger,
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage, debug: &mut bool, game_state: &mut game::Game) {
        self.steps[self.current].update(msg, debug, game_state);
    }

    fn view(&mut self, debug: bool, config_data: &config::Data) -> Element<StepMessage> {
        self.steps[self.current].view(debug, &config_data)
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

pub enum Step {
    Welcome,
    Radio {
        selection: Option<GameType>,
    },
    GameSetup,
    Slider {
        state: slider::State,
        value: u8,
    },
    RowsAndColumns {
        layout: Layout,
        spacing_slider: slider::State,
        spacing: u16,
    },
    Text {
        size_slider: slider::State,
        size: u16,
        color_sliders: [slider::State; 3],
        color: Color,
    },
    Scrollable,
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Debugger,
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    LayoutChanged(Layout),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    GameTypeSelected(GameType),
    InputChanged(String),
    ToggleSecureInput(bool),
    DebugToggled(bool),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage, debug: &mut bool, game_state: &mut game::Game) {
        match msg {
            StepMessage::DebugToggled(value) => {
                if let Step::Debugger = self {
                    *debug = value;
                }
            }
            StepMessage::GameTypeSelected(game_type) => {
                if let Step::Radio { selection } = self {
                    *selection = Some(game_type);
                    Self::game_start(game_state);
                }
            }
            StepMessage::SliderChanged(new_value) => {
                if let Step::Slider { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::TextSizeChanged(new_size) => {
                if let Step::Text { size, .. } = self {
                    *size = new_size;
                }
            }
            StepMessage::TextColorChanged(new_color) => {
                if let Step::Text { color, .. } = self {
                    *color = new_color;
                }
            }
            StepMessage::LayoutChanged(new_layout) => {
                if let Step::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            StepMessage::SpacingChanged(new_spacing) => {
                if let Step::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing;
                }
            }
            StepMessage::InputChanged(new_value) => {
                if let Step::TextInput { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::ToggleSecureInput(toggle) => {
                if let Step::TextInput { is_secure, .. } = self {
                    *is_secure = toggle;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::Radio { .. } => "Radio button",
            Step::GameSetup => "Start Game",
            Step::Slider { .. } => "Slider",
            Step::Text { .. } => "Text",
            Step::RowsAndColumns { .. } => "Rows and columns",
            Step::Scrollable => "Scrollable",
            Step::TextInput { .. } => "Text input",
            Step::Debugger => "Debugger",
            Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Radio { selection } => *selection == Some(GameType::FiveCardDraw),
            Step::GameSetup => true,
            Step::Slider { .. } => true,
            Step::Text { .. } => true,
            Step::RowsAndColumns { .. } => true,
            Step::Scrollable => true,
            Step::TextInput { value, .. } => !value.is_empty(),
            Step::Debugger => true,
            Step::End => false,
        }
    }

    fn view(&mut self, debug: bool, config_data: &config::Data) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(&config_data),
            Step::Radio { selection } => Self::radio(*selection),
            Step::GameSetup => Self::view_hand(),
            Step::Slider { state, value } => Self::slider(state, *value),
            Step::Text {
                size_slider,
                size,
                color_sliders,
                color,
            } => Self::text(size_slider, *size, color_sliders, *color),
            Step::RowsAndColumns {
                layout,
                spacing_slider,
                spacing,
            } => Self::rows_and_columns(*layout, spacing_slider, *spacing),
            Step::Scrollable => Self::scrollable(),
            Step::TextInput {
                value,
                is_secure,
                state,
            } => Self::text_input(value, *is_secure, state),
            Step::Debugger => Self::debugger(debug),
            Step::End => Self::end(),
        }
        .into()
    }

    pub fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn welcome(config_data: &config::Data) -> Column<'a, StepMessage> {
        Self::container("Welcome!")
            .push(Text::new(format!("Hello, {}", config_data.player_nick)))
            .push(Text::new("Server configuration:"))
            .push(Text::new(format!(
                "Maximum players allowed per game: {}",
                config_data.max_players
            )))
            .push(Text::new(format!(
                "Maximum bet per betting round: {}",
                config_data.max_bet
            )))
            .push(Text::new(format!(
                "Maximum raises per betting round: {}",
                config_data.max_raises
            )))
    }

    fn game_start(new_game: &mut game::Game) {
        game::start(new_game)
    }

    fn view_hand() -> Column<'a, StepMessage> {
        game::view_hand()
    }

    fn slider(state: &'a mut slider::State, value: u8) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(Text::new(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            ))
            .push(Text::new(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            ))
            .push(Slider::new(
                state,
                0..=100,
                value,
                StepMessage::SliderChanged,
            ))
            .push(
                Text::new(&value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn rows_and_columns(
        layout: Layout,
        spacing_slider: &'a mut slider::State,
        spacing: u16,
    ) -> Column<'a, StepMessage> {
        let row_radio = Radio::new(Layout::Row, "Row", Some(layout), StepMessage::LayoutChanged);

        let column_radio = Radio::new(
            Layout::Column,
            "Column",
            Some(layout),
            StepMessage::LayoutChanged,
        );

        let layout_section: Element<_> = match layout {
            Layout::Row => Row::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
            Layout::Column => Column::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
        };

        let spacing_section = Column::new()
            .spacing(10)
            .push(Slider::new(
                spacing_slider,
                0..=80,
                spacing,
                StepMessage::SpacingChanged,
            ))
            .push(
                Text::new(&format!("{} px", spacing))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            );

        Self::container("Rows and columns")
            .spacing(spacing)
            .push(Text::new(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            ))
            .push(Text::new(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            ))
            .push(layout_section)
            .push(Text::new(
                "You can also easily change the spacing between elements:",
            ))
            .push(spacing_section)
    }

    fn text(
        size_slider: &'a mut slider::State,
        size: u16,
        color_sliders: &'a mut [slider::State; 3],
        color: Color,
    ) -> Column<'a, StepMessage> {
        let size_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("You can change its size:"))
            .push(Text::new(&format!("This text is {} pixels", size)).size(size))
            .push(Slider::new(
                size_slider,
                10..=70,
                size,
                StepMessage::TextSizeChanged,
            ));

        let [red, green, blue] = color_sliders;

        let color_sliders = Row::new()
            .spacing(10)
            .push(color_slider(red, color.r, move |r| Color { r, ..color }))
            .push(color_slider(green, color.g, move |g| Color { g, ..color }))
            .push(color_slider(blue, color.b, move |b| Color { b, ..color }));

        let color_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("And its color:"))
            .push(Text::new(&format!("{:?}", color)).color(color))
            .push(color_sliders);

        Self::container("Text")
            .push(Text::new(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            ))
            .push(size_section)
            .push(color_section)
    }

    fn radio(selection: Option<GameType>) -> Column<'a, StepMessage> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Select Game Type").size(24))
            .push(GameType::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, game_type| {
                    choices.push(Radio::new(
                        game_type,
                        game_type,
                        selection,
                        StepMessage::GameTypeSelected,
                    ))
                },
            ));

        Self::container("Game Type").push(question)
        // "Iced works very well with iterators! The list above is \
        // basically created by folding a column over the different \
        // choices, creating a radio button for each one of them!", */
    }

    fn scrollable() -> Column<'a, StepMessage> {
        Self::container("Scrollable")
            .push(Text::new(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            ))
            .push(Text::new("Tip: You can use the scrollbar to scroll down faster!").size(16))
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You are halfway there!")
                    .width(Length::Fill)
                    .size(30)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You made it!")
                    .width(Length::Fill)
                    .size(50)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn text_input(
        value: &str,
        is_secure: bool,
        state: &'a mut text_input::State,
    ) -> Column<'a, StepMessage> {
        let text_input = TextInput::new(
            state,
            "Type something to continue...",
            value,
            StepMessage::InputChanged,
        )
        .padding(10)
        .size(30);
        Self::container("Text input")
            .push(Text::new(
                "Use a text input to ask for different kinds of information.",
            ))
            .push(if is_secure {
                text_input.password()
            } else {
                text_input
            })
            .push(Checkbox::new(
                is_secure,
                "Enable password mode",
                StepMessage::ToggleSecureInput,
            ))
            .push(Text::new(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            ))
            .push(
                Text::new(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn debugger(debug: bool) -> Column<'a, StepMessage> {
        Self::container("Debugger")
            .push(Text::new(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            ))
            .push(Text::new(
                "Give it a shot! Check the following checkbox to be able to \
                 see element boundaries.",
            ))
            .push(if cfg!(target_arch = "wasm32") {
                Element::new(
                    Text::new("Not available on web yet!")
                        .color([0.7, 0.7, 0.7])
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
            } else {
                Element::new(Checkbox::new(
                    debug,
                    "Explain layout",
                    StepMessage::DebugToggled,
                ))
            })
            .push(Text::new("Feel free to go back and take a look."))
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be updated as more features are added.",
            ))
            .push(Text::new("Make sure to keep an eye on it!"))
    }
}

fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

fn color_slider(
    state: &mut slider::State,
    component: f32,
    update: impl Fn(f32) -> Color + 'static,
) -> Slider<f64, StepMessage> {
    Slider::new(state, 0.0..=1.0, f64::from(component), move |c| {
        StepMessage::TextColorChanged(update(c as f32))
    })
    .step(0.01)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    FiveCardDraw,
    FiveCardDoubleDraw,
    SevenCardStud,
}

impl GameType {
    fn all() -> [GameType; 3] {
        [
            GameType::FiveCardDraw,
            GameType::FiveCardDoubleDraw,
            GameType::SevenCardStud,
        ]
    }
}

impl From<GameType> for String {
    fn from(game_type: GameType) -> String {
        String::from(match game_type {
            GameType::FiveCardDraw => "Five Card Draw",
            GameType::FiveCardDoubleDraw => "Five Card Double Draw",
            GameType::SevenCardStud => "Seven Card Stud",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
