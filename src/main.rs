mod map;
mod robot;
mod resource;
mod station;
mod simulation;
mod utils;

fn main() {
    let seed = utils::generate_seed();
    simulation::run_simulation(seed);
}
