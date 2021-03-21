mod style;
mod scrollables;
pub mod asset_manager;

use crate::config;
use crate::game::{start, Game};
use crate::player;
use ionic_deckhandler::Card;
use asset_manager::HandToImgs;

use iced::{
    button, container, pane_grid, scrollable, slider, text_input, Align, Button, Checkbox, Color,
    Column, Container, Element, HorizontalAlignment, Length, Radio, Row, Sandbox, Scrollable,
    Slider, Space, Text, TextInput, Vector,
};


use style::{ButtonStyle, ContainerStyle};

enum Page {
    Menu {
        start_button: button::State,
    },
    Setup {
        five_player: button::State,
        seven_player: button::State,
    },
    Game {
        bet: button::State,
        bet_amount: slider::State,
        call: button::State,
        fold: button::State,
        scroll: scrollables::Variant,
        game: Game,
    },
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

    fn container(title: &str, title_sz: u16) -> Column<'a, Message> {
        Column::new()
            .spacing(20)
            .push(
                Text::new(title)
                    .size(title_sz)
                    // .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .align_items(Align::Center)
            .max_width(500)
    }

    fn menu(button_state: &'a mut button::State) -> Column<'a, Message> {
        Self::container("Welcome to TelluricDeckay!", 50)
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

    fn setup(
        five_player_state: &'a mut button::State,
        seven_player_state: &'a mut button::State,
    ) -> Column<'a, Message> {
        Self::container("Game Setup", 30)
            .push(
                Button::new(
                    five_player_state,
                    Text::new("5 Player").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::FivePlayerGamePressed)
                .style(ButtonStyle),
            )
            .push(
                Button::new(
                    seven_player_state,
                    Text::new("7 Player").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::SevenPlayerGamePressed)
                .style(ButtonStyle),
            )
    }

    fn game(
        game: &Game,
        bet_btn_state: &'a mut button::State,
        bet_sdr_state: &'a mut slider::State,
        bet_sdr_val: f32,
        call_btn_state: &'a mut button::State,
        fold_btn_state: &'a mut button::State,
        scroll: &'a mut scrollables::Variant,
    ) -> Column<'a, Message> {
        Column::new().push(
            Row::new()
                .spacing(100)
                .push(
                    Self::container("Table", 30)
                        .push({
                            let mut col = Column::new().spacing(100);
                            let mut player_row = Row::new().spacing(5);
                            for (i, p) in game.players.iter().enumerate() {
                                if i % 2 == 0 && i != 0 {
                                    col = col.push(player_row);
                                    player_row = Row::new().spacing(10);
                                }
                                player_row = player_row.push({
                                    let mut card_row = Row::new();
                                    card_row = card_row.push(Text::new(p.name));
                                    for c_img in p.hand.get_hand_imgs().iter() {
                                        card_row = card_row.push(c_img.clone());
                                    }
                                    card_row
                                })
                            }
                            col
                        })
                        .push({
                            Row::new()
                                .spacing(50)
                                .push({
                                    let hand = Self::container("Your hand", 20);
                                    let mut card_row = Row::new();
                                    for c_img in game.players[4].hand.get_hand_imgs().into_iter() {
                                        card_row = card_row.push(c_img);
                                    }
                                    hand.push(card_row)
                                })
                                .push(
                                    Self::container("Options", 20)
                                        .push(
                                            Text::new(format!(
                                                "Chips left: {}",
                                                game.players[4].chips
                                            ))
                                            .size(15),
                                        )
                                        .push(
                                            Text::new(format!("${:.2}", bet_sdr_val))
                                                .size(15)
                                                // .width(Length::Fill)
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .push(Slider::new(
                                            bet_sdr_state,
                                            0.0..=100.0,
                                            bet_sdr_val,
                                            Message::BetAmountChanged,
                                        ))
                                        .push(
                                            Button::new(
                                                bet_btn_state,
                                                Text::new("Bet").horizontal_alignment(
                                                    HorizontalAlignment::Center,
                                                ),
                                            )
                                            .on_press(Message::BetPressed)
                                            .style(ButtonStyle),
                                        )
                                        .push(
                                            Button::new(
                                                call_btn_state,
                                                Text::new("Call").horizontal_alignment(
                                                    HorizontalAlignment::Center,
                                                ),
                                            )
                                            .on_press(Message::CallPressed)
                                            .style(ButtonStyle),
                                        )
                                        .push(
                                            Button::new(
                                                fold_btn_state,
                                                Text::new("Fold").horizontal_alignment(
                                                    HorizontalAlignment::Center,
                                                ),
                                            )
                                            .on_press(Message::FoldPressed)
                                            .style(ButtonStyle),
                                        ),
                                )
                        }),
                )
                .push(
                    Self::container("State", 20)
                        .push(Text::new(format!("Chips in pot: {}", game.pot)).size(15))
                        .push(
                            game.status.iter()
                                .fold(Scrollable::new(&mut scroll.state).height(350.into()),
                                    |s, msg| s.push(Text::new(msg).size(15))
                                ),
                        ),
                )
        )
    }

    fn view(&mut self, bet_sdr_val: f32) -> Element<Message> {
        match *self {
            Page::Menu {
                ref mut start_button,
            } => Self::menu(start_button),
            Page::Setup {
                ref mut five_player,
                ref mut seven_player,
            } => Self::setup(five_player, seven_player),
            Page::Game {
                ref mut bet,
                ref mut bet_amount,
                ref mut call,
                ref mut fold,
                ref mut scroll,
                ref mut game,
            } => Self::game(game, bet, bet_amount, bet_sdr_val, call, fold, scroll),
            _ => panic!("state not supported"),
        }
        .into()
    }
}

struct Pages {
    pages: [Page; 4],
    current: usize,
    bet_sdr_val: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    NewGamePressed,
    FivePlayerGamePressed,
    SevenPlayerGamePressed,
    BetPressed,
    BetAmountChanged(f32),
    CallPressed,
    FoldPressed,
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
                Page::Game {
                    bet: button::State::new(),
                    bet_amount: slider::State::new(),
                    call: button::State::new(),
                    fold: button::State::new(),
                    scroll: scrollables::Variant::default(),
                    game: {
                        let mut g = Game::new();
                        start(&mut g);
                    g }
                },
                Page::Finish,
            ],
            current: 0,
            bet_sdr_val: 0.,
        }
    }

    fn title(&self) -> String {
        self.pages[self.current].title()
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view(self.bet_sdr_val)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::NewGamePressed => self.current = 1,
            Message::FivePlayerGamePressed => self.current = 2,
            Message::BetPressed => {
                if let Page::Game { ref mut game, ..} = self.pages[self.current] {
                    game.player_bet(4, self.bet_sdr_val as i32);
                }
                else {
                    panic!("Bet pressed outside of game context!");
                }
            },
            Message::BetAmountChanged(amount) => self.bet_sdr_val = amount,
            Message::CallPressed => (),
            Message::FoldPressed => (),
            _ => panic!("not implemented this message yet."),
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
