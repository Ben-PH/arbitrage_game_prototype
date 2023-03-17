use std::{
    collections::HashMap,
    num::{NonZeroIsize, NonZeroUsize},
};

use crate::bot::{Bot, BotConfig, BotId};
const WORLD_SIZE: usize = 12;
const GAME_TURNS: usize = 12;
/// What bots are willing to pay to have their actions prioritised
struct Bid(usize);

pub struct World {
    locations: [Location; WORLD_SIZE],
    bots: HashMap<BotId, Bot>,
    turn_number: usize,
}
#[derive(Clone)]
pub struct Location {
    name: String,
    // todo: locations
}

pub enum Action {
    Move(NonZeroIsize),
    Buy(NonZeroUsize),
    Sell(NonZeroUsize),
    Charge,
}

impl World {
    /// TODO: Canfigure the world
    pub fn new() -> World {
        unimplemented!()
    }
    pub fn add_bot(&mut self, bot_config: BotConfig) -> BotId {
        let bot = Bot {
            name: bot_config.name,
            id: todo!(),
            location: todo!(),
            funds: todo!(),
            battery_charge: todo!(),
            stock: todo!(),
            turns_left: todo!(),
        };
        let id = BotId(self.bots.len());
        self.bots.insert(id, bot).unwrap();
        id
    }
    pub fn run_game(mut self) -> BotId {
        while self.turn_number < GAME_TURNS {
            self.run_turn();
        }

        self.bots
            .into_iter()
            .max_by(|(_, botx), (_, boty)| botx.funds.cmp(&boty.funds))
            .unwrap()
            .0
    }
    pub fn run_turn(&mut self) {
        let actions = self.bots.iter().map(|(id, bot)| unimplemented!()).collect();
        let ordered_actions = World::order_actions(actions);
        for (bot, action) in ordered_actions.into_iter() {
            self.perform_turn(bot, action);
        }
    }
    pub fn get_info(&self) -> Vec<Location> {
        self.locations.to_vec()
    }

    /// Changes the state of the world according to a bots actions.
    pub fn perform_turn(&mut self, bot_id: BotId, action: Action) {
        unimplemented!()
    }
    /// Orders the actions. If two bots are attempting to buy the same item, the one that is
    /// paying a higher action fee in the form of a bid will pre-empt the other
    fn order_actions(actions: HashMap<BotId, (Action, Bid)>) -> HashMap<BotId, Action> {
        unimplemented!()
    }
}
