enum GameVariation {
  FiveCardDraw,
  FiveCardDoubleDraw
}


struct GameRound {
  game_variation: GameVariation,
  betting_round: BettingRound,
  pot: i32
}
