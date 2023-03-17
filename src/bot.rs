use std::num::{NonZeroIsize, NonZeroUsize};

use crate::{
    world::{Action, World},
    StockItem,
};

#[derive(Hash, Eq, PartialEq)]
pub struct BotId(pub usize);
pub struct Bot {
    pub name: String,
    pub id: BotId,
    pub location: usize,
    pub funds: usize,
    pub battery_charge: usize,
    pub stock: Option<(StockItem, NonZeroUsize)>,
    pub turns_left: usize,
}
pub struct BotConfig {
    pub name: String,
}

impl Bot {
    /// The game runner would run this method at the beginning of each turn
    pub fn run_bot(&mut self, world: &mut World, turn: usize) -> Option<Action> {
        match &self.stock {
            // Try to sell our stock
            Some((item, quantity)) => {
                let distance = self.find_best_buyer(item, world);
                match distance == 0 {
                    // sell at current location
                    true => Some(Action::Sell(*quantity)),
                    // move to best location to sell
                    false => Some(Action::Move(distance.try_into().unwrap())),
                }
            }
            // Try to buy stock
            None => match self.find_best_seller() {
                Some(distance) => Some(Action::Move(distance)),
                None => Some(Action::Buy(self.buy_max(&world)?)),
            },
        }
    }

    fn find_best_buyer(&self, item: &StockItem, world: &World) -> isize {
        unimplemented!()
    }

    fn find_best_seller(&self) -> Option<NonZeroIsize> {
        unimplemented!()
    }

    fn buy_max(&self, world: &World) -> Option<NonZeroUsize> {
        unimplemented!()
    }
}
