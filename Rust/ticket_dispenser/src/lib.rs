// src/lib.rs

// TODO
// 1. how can we avoid to make the TicketDispenser#get_turn_ticker mutable?
//    we only need it due to the current implementation of TurnNumberSequence!
pub struct TicketDispenser {
    turn_number_sequence: ConsecutiveTurnNumberSequence,
}

impl TicketDispenser {
    pub(crate) fn new(turn_number_sequence: ConsecutiveTurnNumberSequence) -> Self {
        Self { turn_number_sequence }
    }
}

impl TicketDispenser {
    pub fn get_turn_ticket(&mut self) -> TurnTicket {
        let mut turn_number_sequence = &mut self.turn_number_sequence;
        let new_turn_number = turn_number_sequence.get_next_turn_number();
        TurnTicket::new(new_turn_number)
    }
}

pub struct ConsecutiveTurnNumberSequence {
    turn_number: usize,
}

impl ConsecutiveTurnNumberSequence {
    pub fn new() -> Self {
        Self { turn_number: 0 }
    }

    pub fn get_next_turn_number(&mut self) -> usize {
        let next_turn_number = self.turn_number;
        self.turn_number += 1;
        next_turn_number
    }
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
    use super::{TicketDispenser, ConsecutiveTurnNumberSequence, TurnTicket};

    #[test]
    fn foo() {
        let mut dispenser = TicketDispenser::new(ConsecutiveTurnNumberSequence::new());
        let ticket = dispenser.get_turn_ticket();
        assert_eq!(ticket.get_turn_number(), 0);
    }
}
