pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self {
            Card::Ace => 1u8,
            Card::Two => 2u8,
            Card::Three => 3u8,
            Card::Four => 4u8,
            Card::Five => 5u8,
            Card::Six => 6u8,
            Card::Seven => 7u8,
            Card::Eight => 8u8,
            Card::Nine => 9u8,
            _ => 10u8,
        }
    }
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec![]
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn is_loosing_hand(&self) -> bool {
        self.value() > 21u8
    }

    pub fn value(&self) -> u8 {
        let mut value = 0u8;
        for card in &self.cards {
            value += card.value();
        }
        value
    }
}

#[cfg(test)]
mod test {
    use crate::value_of_a_hand_of_cards::*;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.value(), 0u8);
        assert_eq!(hand.is_loosing_hand(), false);
    }

    #[test]
    fn win_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card::Jack);
        hand.add_card(Card::Jack);
        hand.add_card(Card::Ace);
        assert_eq!(hand.value(), 21u8);
        assert_eq!(hand.is_loosing_hand(), false);
    }

    #[test]
    fn loose_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card::Jack);
        hand.add_card(Card::Jack);
        hand.add_card(Card::Jack);
        assert_eq!(hand.value(), 30u8);
        assert_eq!(hand.is_loosing_hand(), true);
    }
}
