use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Player {
    id: String,
    name: String,
    secret_number: Option<u16>, // This will live here until this turns a multi game binary
    life: u8,                   //This will live here until this turns a multi game binary
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: Uuid::new_v4().to_string(),
            name: name,
            secret_number: None,
            life: 3,
        }
    }

    pub fn set_secret_number(&mut self, secret_number: u16) {
        self.secret_number = Some(secret_number);
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    WaitingForPlayer,
    Ready,
    InProgress,
    Finished,
}

pub struct Game<'a> {
    id: String,
    player_one: Option<&'a Box<Player>>,
    player_two: Option<&'a Box<Player>>,
    turn_player: Option<&'a Box<Player>>,
    mode: GameMode,
}

impl<'a> Game<'a> {
    pub fn new(player: &Box<Player>) -> Game {
        Game {
            id: Uuid::new_v4().to_string(),
            player_one: Some(player),
            player_two: None,
            turn_player: None,
            mode: GameMode::WaitingForPlayer,
        }
    }

    fn start(&mut self) -> Result<(), String> {
        match self.mode {
            GameMode::Ready => {
                self.mode = GameMode::InProgress;
                self.turn_player = self.player_one;
                Ok(())
            }
            _ => Err(String::from("Game is not ready to start")),
        }
    }

    fn validate_players(&self) -> Result<(), String> {
        if self.player_one.is_none() {
            Err(String::from("No player 1 present"))
        } else if self.player_two.is_none() {
            Err(String::from("No player 2 present"))
        } else {
            Ok(())
        }
    }

    fn set_ready(&mut self) -> Result<(), String> {
        self.validate_players()?;
        self.mode = GameMode::Ready;
        Ok(())
    }

    fn get_free_position(&mut self) -> Result<&mut Option<&'a Box<Player>>, String> {
        if self.player_one.is_none() {
            Ok(&mut self.player_one)
        } else if self.player_two.is_none() {
            Ok(&mut self.player_two)
        } else {
            Err(String::from("This game is full"))
        }
    }

    fn accept_challenge(&mut self, joining_player: &'a Box<Player>) -> Result<(), String> {
        let free_position = self.get_free_position()?;
        *free_position = Some(joining_player);
        Ok(())
    }

    fn validate_wip_mode(&self) -> Result<(), String> {
        match self.mode {
            GameMode::InProgress => Ok(()),
            _ => Err(String::from("This game is nor in progress")),
        }
    }

    fn get_opponent(&self, player: &Box<Player>) -> &'a Box<Player> {
        if player == self.player_one.unwrap() {
            self.player_two.unwrap()
        } else {
            self.player_one.unwrap()
        }
    }

    fn guess_number(
        &mut self,
        player: &'a Box<Player>,
        number: u16,
    ) -> Result<GameMessages, String> {
        self.validate_wip_mode()?;

        if player != self.turn_player.unwrap() {
            return Ok(GameMessages::NotYourTurn);
        }

        let opponent = self.get_opponent(player);
        let secret_number = opponent.secret_number.unwrap();

        if number > secret_number {
            self.turn_player = Some(opponent);
            Ok(GameMessages::WrongAnswer(format!(
                "Secret number is less than {}",
                number
            )))
        } else if number < secret_number {
            self.turn_player = Some(opponent);
            Ok(GameMessages::WrongAnswer(format!(
                "Secret number is greater than {}",
                number
            )))
        } else {
            self.mode = GameMode::Finished;
            Ok(GameMessages::YouWin)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMessages {
    YouWin,
    NotYourTurn,
    WrongAnswer(String),
    UnexpectedError(String),
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameMessages, GameMode, Player};

    #[test]
    fn test_game_creation() {
        let player_one = box Player::new(String::from("Chico"));

        let game = Game::new(&player_one);

        assert_eq!(player_one.id, game.player_one.unwrap().id);
        assert_eq!(GameMode::WaitingForPlayer, game.mode);
    }

    #[test]
    fn test_set_ready_error() {
        let player_one = box Player::new(String::from("Chico"));

        let mut game = Game::new(&player_one);

        assert_eq!(Err(String::from("No player 2 present")), game.set_ready());
    }

    #[test]
    fn test_set_ready_error_player_one() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));

        let mut game = Game::new(&player_one);
        assert_eq!(Ok(()), game.accept_challenge(&player_two));
        game.player_one = None;

        assert_eq!(Err(String::from("No player 1 present")), game.set_ready());
    }

    #[test]
    fn test_set_ready() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));

        let mut game = Game::new(&player_one);
        assert_eq!(Ok(()), game.accept_challenge(&player_two));

        assert_eq!(Ok(()), game.set_ready());
    }

    #[test]
    fn test_accept_challenge_player_two() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));

        let mut game = Game::new(&player_one);
        assert_eq!(Ok(()), game.accept_challenge(&player_two));
        assert_eq!(player_one.id, game.player_one.unwrap().id);
        assert_eq!(player_two.id, game.player_two.unwrap().id);
    }

    #[test]
    fn test_accept_challenge_player_one() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));
        let player_three = box Player::new(String::from("Allan"));

        let mut game = Game::new(&player_one);

        assert_eq!(Ok(()), game.accept_challenge(&player_two));
        assert_eq!(player_two.id, game.player_two.unwrap().id);

        game.player_one = None;

        assert_eq!(Ok(()), game.accept_challenge(&player_three));
        assert_eq!(player_three.id, game.player_one.unwrap().id);
    }

    #[test]
    fn test_accept_challenge_error() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));
        let player_three = box Player::new(String::from("Allan"));

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two).unwrap();
        assert_eq!(
            Err(String::from("This game is full")),
            game.accept_challenge(&player_three)
        );
    }

    #[test]
    fn test_start_error() {
        let player_one = box Player::new(String::from("Chico"));

        let mut game = Game::new(&player_one);

        assert_eq!(
            Err(String::from("Game is not ready to start")),
            game.start()
        );
    }

    #[test]
    fn test_start() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();

        assert_eq!(Ok(()), game.start());
    }

    #[test]
    fn test_guess_unexpected_error() {}

    #[test]
    fn test_guess_not_your_turn() {
        let player_one = box Player::new(String::from("Chico"));
        let player_two = box Player::new(String::from("Paloma"));

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();
        game.start();

        assert_eq!(
            Ok(GameMessages::NotYourTurn),
            game.guess_number(&player_two, 42)
        );
    }

    #[test]
    fn test_guess_wrong_answer_bigger() {
        let mut player_one = box Player::new(String::from("Chico"));
        player_one.secret_number = Some(43);

        let mut player_two = box Player::new(String::from("Paloma"));
        player_two.secret_number = Some(42);

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();
        game.start();

        assert_eq!(
            Ok(GameMessages::WrongAnswer(String::from(
                "Secret number is less than 43"
            ))),
            game.guess_number(&player_one, 43)
        );
    }

    #[test]
    fn test_guess_wrong_answer_smaller() {
        let mut player_one = box Player::new(String::from("Chico"));
        player_one.secret_number = Some(42);

        let mut player_two = box Player::new(String::from("Paloma"));
        player_two.secret_number = Some(43);

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();
        game.start();

        assert_eq!(
            Ok(GameMessages::WrongAnswer(String::from(
                "Secret number is greater than 42"
            ))),
            game.guess_number(&player_one, 42)
        );
    }

    #[test]
    fn test_guess_you_win() {
        let mut player_one = box Player::new(String::from("Chico"));
        player_one.secret_number = Some(43);

        let mut player_two = box Player::new(String::from("Paloma"));
        player_two.secret_number = Some(42);

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();
        game.start();

        assert_eq!(Ok(GameMessages::YouWin), game.guess_number(&player_one, 42));
        assert_eq!(GameMode::Finished, game.mode);
    }

    #[test]
    fn test_player_switch() {
        let mut player_one = box Player::new(String::from("Chico"));
        player_one.secret_number = Some(42024);

        let mut player_two = box Player::new(String::from("Paloma"));
        player_two.secret_number = Some(42024);

        let mut game = Game::new(&player_one);
        game.accept_challenge(&player_two);
        game.set_ready();
        game.start();

        game.guess_number(&player_one, 42);
        assert_eq!(**game.turn_player.unwrap(), *player_two);

        game.guess_number(&player_two, 42);
        assert_eq!(**game.turn_player.unwrap(), *player_one);
    }
}
