use crate::config;
use crate::game::{Game, HandToImgs, start};
use crate::player;
use ionic_deckhandler::Card;
mod style;

use iced::{
    button, container, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, pane_grid,
    Element, HorizontalAlignment, Length, Radio, Row, Sandbox, Scrollable, Slider, Space, Text,
    TextInput, Vector, Align
};

use style::{ ButtonStyle, ContainerStyle };

enum Page {
    Menu { start_button: button::State },
    Setup { five_player: button::State, seven_player: button::State },
    Game,
    Finish,
}

impl<'a> Page {
    fn title(&self) -> String {
        // match self {
        //     Page::Menu { .. } => "Menu".to_owned(),
        //     Page::Setup { .. } => "Setup".to_owned(),
        //     Page::Game => "Game".to_owned(),
        //     Page::Finish => "Finish".to_owned(),
        // }
        "TelluricDeckay".to_owned()
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

    fn setup(five_player_state: &'a mut button::State, seven_player_state: &'a mut button::State) -> Column<'a, Message> {
        Self::container("Game Setup")
        .push(
            Button::new(
                five_player_state,
                Text::new("5 Player").horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::FivePlayerGamePressed)
            .style(ButtonStyle)
        )
        .push(
            Button::new(
                seven_player_state,
                Text::new("7 Player").horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::SevenPlayerGamePressed)
            .style(ButtonStyle)
        )
    }

    fn game(game: &Game) -> Column<'a, Message> {
        Self::container("Table")
        .push({
            let mut player_row = Row::new().spacing(2);
            for p in game.players.iter() {
                player_row = player_row.push({
                    let mut card_row = Row::new();
                    for c_img in p.hand.get_hand_imgs().iter() {
                        card_row = card_row.push(c_img.clone());
                    }
                    card_row
                })
            }
            player_row
        })
        .push({
            Row::new()
                .push({
                    let mut card_row = Row::new();
                    for c_img in game.players[0].hand.get_hand_imgs().iter() {
                        card_row = card_row.push(c_img.clone());
                    }
                    card_row
                })
                .push(Self::container("Options"))
        })
    }

    fn view(&mut self, game: &Game) -> Element<Message> {
        match *self {
            Page::Menu {
                ref mut start_button,
            } => Self::menu(start_button),
            Page::Setup { ref mut five_player, ref mut seven_player } => Self::setup(five_player, seven_player),
            Page::Game => Self::game(game),
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
    FivePlayerGamePressed,
    SevenPlayerGamePressed,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            pages: [
                Page::Menu {
                    start_button: button::State::new(),
                },
                Page::Setup {
                    five_player: button::State::new(),
                    seven_player: button::State::new(),
                },
                Page::Game,
                Page::Finish,
            ],
            current: 0,
        }
    }

    fn title(&self) -> String {
        self.pages[self.current].title()
    }

    fn view(&mut self, game: &Game) -> Element<Message> {
        self.pages[self.current].view(game)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::NewGamePressed => self.current = 1,
            Message::FivePlayerGamePressed => self.current = 2,
            _ => panic!("not implemented this message yet."),
        }
    }
}

pub struct Gui {
    pages: Pages,
    game: Game,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Gui {
        let mut g = Game::new();
        start(&mut g);
        Gui {
            pages: Pages::new(),
            game: g,
        }
    }

    fn title(&self) -> String {
        self.pages.title()
    }

    fn update(&mut self, event: Self::Message) {
        self.pages.update(event)
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(self.pages.view(&self.game))
            .style(ContainerStyle)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}