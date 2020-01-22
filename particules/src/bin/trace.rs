use particules::sma::Sma;
use particules::Direction;
use particules::HDirection;
use particules::Point;
use particules::VDirection;

fn main() {
    let mut sma = Sma::new(1100, 1100);

    sma.gen_agents(40);


    let mut count = 0;
    loop {
        sma.tick();
        println!("tick {}", count);
        count+=1;
    }
}
