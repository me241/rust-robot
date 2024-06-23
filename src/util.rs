use rand::Rng;

pub fn generate_seed() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
