use uuid::Uuid;


#[derive(Debug)]
pub struct Player {
    id: String,
    name: String,
    secret_number: Option<u16>,
    life: u8,
}


impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: Uuid::new_v4().to_string(),
            name: name,
            secret_number: None,
            life: 3
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
    InProgress
}


pub struct Game<'a> {
    id: String,
    player_one: Option<&'a Box<Player>>,
    player_two: Option<&'a Box<Player>>,
    turn_player_id: Option<String>,
    mode: GameMode,
}


impl<'a> Game<'a> {
    pub fn new(player: &Box<Player>) -> Game {
        Game {
            id: Uuid::new_v4().to_string(),
            player_one: Some(player),
            player_two: None,
            turn_player_id: None,
            mode: GameMode::WaitingForPlayer
        }
    }

    fn start(&mut self) -> Result<(), String> {
        match self.mode {
            GameMode::Ready => {
                self.mode = GameMode::InProgress;
                self.turn_player_id = Some(self.player_one.unwrap().id.clone());
                Ok(())
            },
            _ => Err(String::from("Game is not ready to start"))
        }
    }

    fn set_ready(&mut self) -> Result<(), String> {
        match self.player_two {
            Some(_) => {
                match self.player_one {
                    Some(_) => {
                        self.mode = GameMode::Ready;
                        Ok(())
                    },
                    None => Err(String::from("No player 1 present"))
                }
            },
            None => Err(String::from("No player 2 present"))
        }
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
}


#[cfg(test)]
mod tests {
    use crate::game::{Game, GameMode, Player};

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
        assert_eq!(Err(String::from("This game is full")), game.accept_challenge(&player_three));
    }

    #[test]
    fn test_start_error() {
        let player_one = box Player::new(String::from("Chico"));

        let mut game = Game::new(&player_one);

        assert_eq!(Err(String::from("Game is not ready to start")), game.start());
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
}
