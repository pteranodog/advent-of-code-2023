fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {
    let input_lines = read_lines(input);
    let mut hands = parse_input(input_lines, false);
    hands.sort();

    let mut sum = 0;

    for index in 0..hands.len() {
        sum += hands[index].bid * (index as i32 + 1);
    }

    sum
}

pub fn puzzle_2(input: String) -> i32 {
    let input_lines = read_lines(input);
    let mut hands = parse_input(input_lines, true);
    for hand in &mut hands {
        hand.convert_to_joker_rule();
    }
    hands.sort();

    let mut sum = 0;

    for index in 0..hands.len() {
        sum += hands[index].bid * (index as i32 + 1);
    }

    sum
}

fn parse_input(input: Vec<String>, joker_rule: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input {
        let mut cards = [0; 5];
        let bid;
        
        let line = line.trim();
        let hand_chars: Vec<char> = line.split_whitespace().collect::<Vec<&str>>()[0].chars().collect();
        let bid_str = line.split_whitespace().collect::<Vec<&str>>()[1];
        for index in 0..5 {
            cards[index] = match hand_chars[index] {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if joker_rule {
                        1
                    } else {
                        11
                    }
                },
                'T' => 10,
                _ => hand_chars[index].to_digit(10).unwrap() as i32,
            };
        }
        bid = bid_str.parse().unwrap();
        hands.push(Hand::new(cards, bid));
    }

    hands
}

struct Hand {
    cards: [i32; 5],
    bid: i32,
    hand_type: i32,
}

impl Hand {
    fn new(cards: [i32; 5], bid: i32) -> Hand {
        let hand_type;
        let mut type_count = [0; 13];
        for card in cards {
            if card == 1 {
                type_count[9] += 1;
            } else {
                type_count[card as usize - 2] += 1;
            }
        }
        if value_count(type_count, 5) == 1 {
            hand_type = 6;
        } else if value_count(type_count, 4) == 1 {
            hand_type = 5;
        } else if value_count(type_count, 3) == 1 && value_count(type_count, 2) == 1 {
            hand_type = 4;
        } else if value_count(type_count, 3) == 1 {
            hand_type = 3;
        } else if value_count(type_count, 2) == 2 {
            hand_type = 2;
        } else if value_count(type_count, 2) == 1 {
            hand_type = 1;
        } else {
            hand_type = 0;
        }
        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn convert_to_joker_rule(&mut self) {
        let mut new_hand = Hand {
            cards: self.cards.clone(),
            bid: self.bid,
            hand_type: self.hand_type,
        };
        for index in 1..15 {
            let mut new_cards: [i32; 5] = [0; 5];
            for card_location in 0..5 {
                if self.cards[card_location] == 1 {
                    new_cards[card_location] = index as i32;
                } else {
                    new_cards[card_location] = self.cards[card_location];
                }
            }
            let maybe_new_hand = Hand::new(new_cards, self.bid);
            if maybe_new_hand.hand_type > new_hand.hand_type {
                new_hand = maybe_new_hand;
            }
        }
        self.hand_type = new_hand.hand_type;
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.bid == other.bid && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type > other.hand_type {
            Some(std::cmp::Ordering::Greater)
        } else if self.hand_type < other.hand_type {
            Some(std::cmp::Ordering::Less)
        } else {
            for index in 0..5 {
                if self.cards[index] > other.cards[index] {
                    return Some(std::cmp::Ordering::Greater);
                } else if self.cards[index] < other.cards[index] {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            std::cmp::Ordering::Greater
        } else if self.hand_type < other.hand_type {
            std::cmp::Ordering::Less
        } else {
            for index in 0..5 {
                if self.cards[index] > other.cards[index] {
                    return std::cmp::Ordering::Greater;
                } else if self.cards[index] < other.cards[index] {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

fn value_count(array: [i32; 13], value: i32) -> i32 {
    let mut count = 0;
    for i in 0..13 {
        if array[i] == value {
            count += 1;
        }
    }
    count
}