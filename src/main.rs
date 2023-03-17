use world::World;

pub mod bot;
pub mod input;
pub mod world;

pub enum StockItem {
    Apples,
    Bananas,
    Pears,
    Pineapple,
    Orange,
}

const MAX_TURNS: usize = 999;
const WORLD_SIZE: usize = 4;
const MAX_BOTS: usize = 2048;
const MAX_FRUIT_TYPES: usize = 128;
const MAX_NAME_CHARS: usize = 64;
const MAX_SUPPLIED_BOT_NAME_CHARS: usize = 32;

fn main() {
    let mut world = World::new();
    world.add_bot(unimplemented!());
    // todo: provide the bot runner with the id used, or something. In a smart contract, it would most likely just be the SC address...
    // create bots
    world.run_game();
}
