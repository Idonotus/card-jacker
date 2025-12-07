use std::mem::ManuallyDrop;

struct Player {

}

struct Card {

}

union GameVar {
	num: i32,
	player: ManuallyDrop<Player>,
	card: ManuallyDrop<Card>
}

struct TypeContainer {
	game_type: GameType,
	var: GameVar
}

#[derive(PartialEq, Eq)]
enum GameType {
	Number,
	Player,
	Card,
}

impl TypeContainer {
	fn is_safe(&self, expected_type: GameType) -> bool {
		self.game_type == expected_type
	}
}

pub trait Rule {
	fn get_text() -> String;
}

type ActionContext = Vec<GameVar>;

pub trait Action {
	fn is_expected_context_safe(&self, context: Vec<GameType>) -> bool;
	fn is_context_safe(&self, action_context: ActionContext) -> bool;
	fn run_action(&self, action_context: ActionContext);
}