use particules::sma::Sma;
use std::time::{Duration, SystemTime};

fn main() {
    let mut sma = Sma::new(1100, 1100);

    sma.gen_agents(90);
    loop {
        let now = SystemTime::now();
        sma.tick();
        println!("{}", now.elapsed().unwrap().as_millis());
    }
}
