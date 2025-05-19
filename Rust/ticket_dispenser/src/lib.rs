// src/lib.rs

use std::sync::{Arc, Mutex};

// TODO
// 1. how can we avoid to make the TicketDispenser#get_turn_ticker mutable?
//    we only need it due to the current implementation of TurnNumberSequence!
//    => shall we use Arc<Mutex<>>?
// 2. write a test with multiple dispensers
pub struct TicketDispenser<'a, NumberSequence: TurnNumberSequence> {
    turn_number_sequence: &'a NumberSequence,
}

impl<'a, NumberSequence: TurnNumberSequence> TicketDispenser<'a, NumberSequence> {
    pub(crate) fn new(turn_number_sequence: &'a NumberSequence) -> Self {
        Self {
            turn_number_sequence,
        }
    }

    pub fn get_turn_ticket(&mut self) -> TurnTicket {
        let new_turn_number = self.turn_number_sequence.get_next_turn_number();
        TurnTicket::new(new_turn_number)
    }
}

pub struct ConsecutiveTurnNumberSequence {
    turn_number: Arc<Mutex<usize>>,
}

impl ConsecutiveTurnNumberSequence {
    pub fn new() -> Self {
        Self { turn_number: Arc::new(Mutex::new(0)) }
    }
}

impl TurnNumberSequence for ConsecutiveTurnNumberSequence {
    fn get_next_turn_number(&self) -> usize {
        let result = self.turn_number.lock();
        todo!();
        let next_turn_number = self.turn_number;
        self.turn_number += 1;
        next_turn_number
    }
}

pub trait TurnNumberSequence {
    fn get_next_turn_number(&self) -> usize;
}

pub struct TurnTicket {
    turn_number: usize,
}

impl TurnTicket {
    pub fn new(turn_number: usize) -> Self {
        Self { turn_number }
    }

    pub fn get_turn_number(&self) -> usize {
        self.turn_number
    }
}

// src/test.rs
#[cfg(test)]
mod tests {
    use super::{TicketDispenser, TurnNumberSequence};

    #[test]
    fn issues_a_sequence_of_tickets() {
        let number_sequence = FakeTurnNumberSequence(vec![2, 5, 9, 1, 11]);
        let mut dispenser = TicketDispenser::new(&number_sequence);

        assert_eq!(dispenser.get_turn_ticket().get_turn_number(), 2);
        assert_eq!(dispenser.get_turn_ticket().get_turn_number(), 5);
        assert_eq!(dispenser.get_turn_ticket().get_turn_number(), 9);
        assert_eq!(dispenser.get_turn_ticket().get_turn_number(), 1);
    }

    #[test]
    fn the_same_ticket_should_not_be_issued_to_two_different_customers() {
        let number_sequence = FakeTurnNumberSequence(vec![2, 5, 9, 1, 11]);
        let mut first_dispenser = TicketDispenser::new(&number_sequence);
        let mut second_dispenser = TicketDispenser::new(&number_sequence);

        assert_eq!(first_dispenser.get_turn_ticket().get_turn_number(), 2);
        assert_eq!(second_dispenser.get_turn_ticket().get_turn_number(), 5);
        assert_eq!(first_dispenser.get_turn_ticket().get_turn_number(), 9);
        assert_eq!(second_dispenser.get_turn_ticket().get_turn_number(), 1);
        assert_eq!(second_dispenser.get_turn_ticket().get_turn_number(), 11);
    }

    struct FakeTurnNumberSequence(Vec<usize>);

    impl TurnNumberSequence for FakeTurnNumberSequence {
        fn get_next_turn_number(&self) -> usize {
            self.0.remove(0)
        }
    }
}
