use particules::sma::Sma;
use particules::Direction;
use particules::HDirection;
use particules::Point;
use particules::VDirection;

fn main() {
    let mut sma = Sma::new(5, 5);
    let point = Point { x: 0, y: 0 };
    let direction = Direction {
        x: HDirection::Right,
        y: VDirection::None,
    };

    sma.add_agent(point, direction);

    let point = Point { x: 2, y: 0 };
    let direction = Direction {
        x: HDirection::Left,
        y: VDirection::None,
    };

    sma.add_agent(point, direction);

    sma.tick();
    println!("1 {:?}", sma.env);
    sma.tick();
    println!("2 {:?}", sma.env);
    sma.tick();
    println!("3 {:?}", sma.env);
}
