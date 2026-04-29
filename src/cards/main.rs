use enum_derived::Rand;

#[derive(Rand, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub suite: Suite,
    #[custom_rand(custom_card_val)]
    pub value: u32,
}
pub fn custom_card_val() -> u32 {
    rand::random_range(1..=10)
}
#[derive(Rand, PartialEq, Eq, Clone, Copy)]
pub enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Cloves,
}

#[derive(Default)]
pub struct BlackJackGame {
    pub played_cards: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
}
impl BlackJackGame {
    pub fn add_hand(&mut self) {
        self.hands.push(Vec::new());
    }
    pub fn hit_hand(&mut self, hand_index: usize) {
        let mut card = Card::rand();
        while self.played_cards.contains(&card) {
            card = Card::rand();
        }
        self.played_cards.push(card);
        self.hands[hand_index].push(card);
        if self.hand_val(hand_index).0 > 21 {
            todo!()
        }
    }
    pub fn hand_val(&self, hand_index: usize) -> (u32,u32) {
        let mut counter = 0;
        let mut ace_played = false;
        for card in &self.hands[hand_index] {
            counter += card.value;
            if card.value == 1 {
                ace_played = true;
            }
        }
        if ace_played && (counter + 10) <= 21 {
            (counter, counter + 10)
        }
        else {
            (counter,counter)
        }
    }

}

fn main() {
    let mut new_game = BlackJackGame::default();
    new_game.add_hand();
    new_game.hit_hand(0);
    new_game.hit_hand(0);
    new_game.hit_hand(0);
    for card in &new_game.hands[0] {
        println!("{}",card.value);
    }
    println!("{} {}",new_game.hand_val(0).0,new_game.hand_val(0).1);
}