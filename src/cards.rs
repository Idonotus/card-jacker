

struct Suit<'a> {
	id: i8,
	name: &'a str,
}

pub struct SuitReference(i8);

enum Value {
	Unknown,
	Definite(i32),
}

pub struct Card {
	suit: SuitReference,
	face: i32,
	value: Value,
}

trait AddCard {
	
}

trait DelCard {
	
}

trait CardCount {
	fn get_count() -> i32;
}

trait OrderedSelect: CardCount {
	fn fetch(item: i32) -> Card;
}

trait PooledSelect {
	fn select(count: i32) -> Vec<Card>;
}

trait Shuffle {
	fn shuffle(&mut self);
}

pub struct Deck {
	card: Vec<Card>,
}

impl CardCount for Deck {
	fn get_count() -> i32 {
		todo!()
	}
}

impl OrderedSelect for Deck {
	fn fetch(item: i32) -> Card {
		todo!()
	}
}

impl AddCard for Deck {

	
}

impl PooledSelect for Deck {
	fn select(count: i32) -> Vec<Card> {
		todo!()
	}
}

pub struct CardPool {
	cards: Vec<Card>,
}

impl PooledSelect for CardPool {
	fn select(count: i32) -> Vec<Card> {
		todo!()
	}
}