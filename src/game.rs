use crate::player::{self, Player};
use iced::{Checkbox, Column, Length, Row, Svg, Text};
use ionic_deckhandler::{Card, Deck, Rank, Suit};

// Card display.
fn card_img(card_name: &str) -> Svg {
    Svg::from_path(format!(
        "{}/assets/cards/{}",
        telluricdeckay::config_h::get_assetsdir(),
        card_name
    ))
    .width(Length::Units(50))
    .height(Length::Units(50))
}

trait CardToImg {
    fn get_card_img(&self) -> Svg;
}

impl CardToImg for Card {
    fn get_card_img(&self) -> Svg {
        match *self {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            } => card_img("AS.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
            } => card_img("KS.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            } => card_img("QS.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
            } => card_img("JS.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
            } => card_img("TS.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
            } => card_img("9S.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            } => card_img("8S.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
            } => card_img("7S.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
            } => card_img("6S.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            } => card_img("5S.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            } => card_img("4S.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
            } => card_img("3S.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
            } => card_img("2S.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            } => card_img("AH.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            } => card_img("KH.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Hearts,
            } => card_img("QH.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
            } => card_img("JH.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Hearts,
            } => card_img("TH.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
            } => card_img("9H.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            } => card_img("8H.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Hearts,
            } => card_img("7H.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            } => card_img("6H.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            } => card_img("5H.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            } => card_img("4H.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            } => card_img("3H.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
            } => card_img("2H.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            } => card_img("AC.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            } => card_img("KC.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Clubs,
            } => card_img("QC.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            } => card_img("JC.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            } => card_img("TC.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            } => card_img("9C.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            } => card_img("8C.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Clubs,
            } => card_img("7C.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            } => card_img("6C.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Clubs,
            } => card_img("5C.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            } => card_img("4C.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            } => card_img("3C.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            } => card_img("2C.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            } => card_img("AD.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            } => card_img("KD.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            } => card_img("QD.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            } => card_img("JD.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            } => card_img("TD.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamonds,
            } => card_img("9D.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            } => card_img("8D.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamonds,
            } => card_img("7D.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            } => card_img("6D.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            } => card_img("5D.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            } => card_img("4D.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            } => card_img("3D.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            } => card_img("2D.svg"),
            _ => panic!("card image missing"),
        }
    }
}

pub trait HandToImgs {
    fn get_hand_imgs(&self) -> Vec<Svg>;
}

impl HandToImgs for [Card] {
    fn get_hand_imgs(&self) -> Vec<Svg> {
        self.iter().map(|c| c.get_card_img()).collect()
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub number_of_players: usize,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub round: BettingRound,
    pub status: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::<Player>::new(),
            number_of_players: 0,
            pot: 0,
            deck: Card::get_deck(),
            round: BettingRound::new(),
            status: String::new(),
        }
    }

    pub fn player_bet(&mut self, player_id: usize, bet_amount: i32) {
        if bet_amount < self.round.initial_bet_plus_raises {
            if self.round.initial_bet_plus_raises < self.players[player_id].chips {
                self.status
                    .push_str("\nBet is less than initial plus raises.");
                return;
            }
            else if bet_amount < self.players[player_id].chips {
                self.status
                    .push_str("\nYou must bet all your chips.");
                return;
            }
        }
        if self.players[player_id].chips < bet_amount {
            self.status
                .push_str("\nBet exceeds player's remaining chips!");
        }
        else {
            self.players[player_id].chips -= bet_amount;
            self.pot += bet_amount;
            self.round.initial_bet_plus_raises = bet_amount;
            self.status.push_str(&format!(
                "\n{} bets {} chips.",
                self.players[player_id].name, bet_amount
            ));
        }
    }
}

pub fn start(new_game: &mut Game) {
    let mut players = vec![
        player::Player::new("Bambi"),
        player::Player::new("Randi"),
        player::Player::new("Candi"),
        player::Player::new("Paul"),
        player::Player::new("John"),
    ];

    // TODO: The server will keep track of connected players.

    // let mut new_game = Game::new();

    // reset some values for players before the next hand is dealt.
    for pl in players.iter_mut() {
        pl.total_amount_added_this_round = 0;
        pl.has_folded = false;
        // TODO: When a player connects, a new Player struct will be created and they'll
        //  be added to the new_game.players Vector
        new_game.players.push(*pl);
    }

    new_game.number_of_players = new_game.players.len();

    new_game.deck.shuffle_deck();

    let input_game_variation = CardDealing::FiveCardDraw;
    // Each of these cases will likely get moved to the Gui file as a "Step"
    match input_game_variation {
        CardDealing::FiveCardDraw => {
            // Deal the cards
            for i in 0..5 {
                for pl in new_game.players.iter_mut() {
                    pl.hand[i] = new_game
                        .deck
                        .take_from_top()
                        .expect("Error: deck is empty!");
                }
            }

            ante(new_game);
            round(new_game);
            // discard/draw
            // another round
            // showdown
        }
        CardDealing::SevenCardStud => {
            ()
            // deal
            // round
            // deal a card face up to each player
            // round
            // deal a card face up to each player
            // round
            // deal a card face up to each player
            // round
            // deal a card face DOWN to each player
            // round
            // showdown
        }
        _ => (),
    }

    // TODO: Don't show the hands for players that folded

    // Showdown
    new_game
        .status
        .push_str(&format!("\nTotal in pot = ${}", new_game.pot));
    for pl in new_game.players.iter_mut() {
        new_game.status.push_str(&format!(
            "\nPlayer {} got a {} and has {} chips remaining",
            pl.name,
            telluric_handeval::poker::evaluate(&mut pl.hand).0.name(),
            pl.chips
        ));
    }
    println!();
}

/* let question = Column::new()
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
)); */

// TODO: Checkboxes for each card needed here
// pub fn view_hand<'a>(player_hand: &[Card; 5]) -> Column<'a, StepMessage> {
//     let container = gui::Step::container("Game Start").push(Text::new("(Test) Game Start"));

//     // Create row of cards.
//     container.push(
//         player_hand
//             .get_hand_imgs()
//             .into_iter()
//             .fold(Row::new(), |acc, img| acc.push(img)),
//     )
// }

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    // FiveCardDoubleDraw,
    SevenCardStud,
    // FiveCardStud,
}

#[derive(Debug)]
pub struct BettingRound {
    pub initial_bet_plus_raises: i32,
    pub turns: usize,
    pub all_bets_paid: bool,
}

impl BettingRound {
    pub fn new() -> Self {
        Self {
            initial_bet_plus_raises: 0,
            turns: 0,
            all_bets_paid: false,
        }
    }
}

pub fn ante(new_game: &mut Game) {
    for pl in new_game.players.iter_mut() {
        pl.chips -= 1;
        new_game.pot += 1;
        new_game
            .status
            .push_str(&format!("\n{} adds 1 for the ante", pl.name));
    }
}

pub fn round(new_game: &mut Game) {
    while new_game.round.all_bets_paid == false {
        let mut input: player::Action;
        for pl in new_game.players.iter_mut() {
            // println!("{:?}-{:?}", &new_game.initial_bet_plus_raises, &pl.total_amount_added_this_round);
            // println!("{:?}", &new_game.roundturns_this_round);
            // println!("pot - {:?}", &new_game.pot);
            // println!();

            if (pl.total_amount_added_this_round == new_game.round.initial_bet_plus_raises
                && new_game.round.turns > new_game.number_of_players)
                || (new_game.round.turns == new_game.number_of_players
                    && new_game.round.initial_bet_plus_raises == 0)
            {
                new_game.round.all_bets_paid = true;

                // reset some values before the start of the next betting round
                for pl in new_game.players.iter_mut() {
                    pl.total_amount_added_this_round = 0;
                }

                break;
            }

            new_game.round.turns += 1;

            if pl.has_folded {
                continue;
            }
            match new_game.round.initial_bet_plus_raises == 0 {
                true => {
                    match new_game.round.turns {
                        1 => input = player::Action::Check,
                        2 => input = player::Action::Check,
                        3 => input = player::Action::Open,
                        4 => input = player::Action::Fold,
                        _ => input = player::Action::Fold,
                    }
                    match input {
                        player::Action::Open => {
                            let input_open = 5;
                            player::open(
                                input_open,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &mut new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            );
                            new_game
                                .status
                                .push_str(&format!("\n{} opens with {}", pl.name, input_open));
                            new_game.round.initial_bet_plus_raises = input_open;
                        }
                        player::Action::Fold => {
                            pl.has_folded = player::fold(&pl.name);
                        }
                        player::Action::Check => player::check(&pl.name),

                        _ => println!("!Condition mismatch 1"), // The UI should only allow the options above
                    }
                }

                false => {
                    match new_game.round.turns {
                        4 => input = player::Action::Raise,
                        5 => input = player::Action::Call,
                        6 => input = player::Action::Raise,
                        7 => input = player::Action::Fold,
                        8 => input = player::Action::Raise,
                        9 => input = player::Action::Raise,
                        10 => input = player::Action::Fold,
                        _ => input = player::Action::Call,
                    }
                    match input {
                        player::Action::Call => {
                            player::call(
                                &pl.name,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            );
                        }
                        player::Action::Raise => {
                            let input_raise = 8;
                            player::raise(
                                &pl.name,
                                &input_raise,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &mut new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            );
                        }
                        player::Action::Fold => pl.has_folded = player::fold(&pl.name),
                        _ => println!("!Condition mismatch 2"),
                    }
                }
            }
        }
    }
}
