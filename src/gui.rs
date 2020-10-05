use crate::config;
use crate::game;
use crate::player;
use ionic_deckhandler::Card;

use iced::{
    button, container, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container,
    Element, HorizontalAlignment, Length, Radio, Row, Sandbox, Scrollable, Slider, Space, Text,
    TextInput, Vector, Align
};

enum Page {
    Menu { start_button: button::State },
    Setup,
    Game,
    Finish,
}

impl<'a> Page {
    fn title(&self) -> String {
        match self {
            Page::Menu { .. } => "Menu".to_owned(),
            Page::Setup => "Setup".to_owned(),
            Page::Game => "Game".to_owned(),
            Page::Finish => "Finish".to_owned(),
        }
    }

    fn container(title: &str) -> Column<'a, Message> {
        Column::new().spacing(20).push(
            Text::new(title)
                .size(50)
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .align_items(Align::Center)
    }

    fn menu(button_state: &'a mut button::State) -> Column<'a, Message> {
        Self::container("Welcome to TelluricDeckay!")
            .push(
                Text::new("A Poker Game")
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(
                Button::new(
                    button_state,
                    Text::new("Start").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::NewGamePressed)
                .style(ButtonStyle),
            )
    }

    fn setup() -> Column<'a, Message> {
        Self::container("Game Setup").push(Text::new("Nothing to see here."))
    }

    fn view(&mut self) -> Element<Message> {
        match *self {
            Page::Menu {
                ref mut start_button,
            } => Self::menu(start_button),
            Page::Setup => Self::setup(),
            _ => panic!("state not supported"),
        }
        .into()
    }
}

struct Pages {
    pages: [Page; 4],
    current: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    NewGamePressed,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            pages: [
                Page::Menu {
                    start_button: button::State::new(),
                },
                Page::Setup,
                Page::Game,
                Page::Finish,
            ],
            current: 0,
        }
    }

    fn title(&self) -> String {
        self.pages[self.current].title()
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::NewGamePressed => self.current = 1,
        }
    }
}

pub struct Gui {
    pages: Pages,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Gui {
        Gui {
            pages: Pages::new(),
        }
    }

    fn title(&self) -> String {
        self.pages.title()
    }

    fn update(&mut self, event: Self::Message) {
        self.pages.update(event)
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(self.pages.view())
            .style(ContainerStyle)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
            border_radius: 2,
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

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(0x36, 0x30, 0x3F).into()),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}
