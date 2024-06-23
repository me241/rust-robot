use std::sync::{Arc, Mutex};
use std::thread;
use crate::map::Map;
use crate::robot::{Robot, Role};
use crate::station::Station;
use rand::Rng;

pub fn run_simulation(seed: u32) {
    let map = Arc::new(Mutex::new(Map::new(seed)));
    let station = Arc::new(Mutex::new(Station::new(Arc::clone(&map))));

    let mut robots = vec![
        Robot::new(1, 1, Role::Explorer),
        Robot::new(3, 3, Role::Collector),
        Robot::new(5, 5, Role::Scientist),
    ];

    let handles: Vec<_> = robots.into_iter().map(|mut robot| {
        let map = Arc::clone(&map);
        let station = Arc::clone(&station);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..10 {
                robot.explore(map.clone());
                thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..500)));
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let map = map.lock().unwrap();
    map.display();

    let station = station.lock().unwrap();
    station.collect_data();
}
