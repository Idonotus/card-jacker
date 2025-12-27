use std::mem::ManuallyDrop;
use crate::cards::Card;

struct Player {

}

union Var {
	num: i32,
	player: ManuallyDrop<Player>,
	card: ManuallyDrop<Card>
}

struct GameVar {
	game_type: GameType,
	var: Var
}

#[derive(PartialEq, Eq)]
enum GameType {
	Number,
	Player,
	Card,
	Action,

}

impl GameVar {
	fn is_safe(&self, expected_type: GameType) -> bool {
		self.game_type == expected_type
	}
}

pub trait Rule {
	fn get_text() -> String;
}

type ActionContext = Vec<GameVar>;

pub trait Action {
	fn is_expected_context_safe(&self, context: &Vec<GameType>) -> bool;
	fn is_context_safe(&self, action_context: &ActionContext) -> Option<&'static str>;
	fn run_action(&self, action_context: &ActionContext) -> Option<&'static str> {
		match self.is_context_safe(action_context) {
			Some(msg) => Some(msg),
			None => self.run_action_unsafe(action_context)
		}
	}
	fn run_action_unsafe(&self, action_context: &ActionContext) -> Option<&'static str>;
	fn save(&self) -> &str;
}