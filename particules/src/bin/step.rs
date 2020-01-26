use particules::environment::Cell;
use particules::sma::Sma;
use particules::SMA;
fn main() {
    Sma::new_with_fish(5, 5, 20, 20, 20);

    SMA.write().unwrap().gen_fish_agents(10, 10);
    SMA.read()
        .unwrap()
        .get_state()
        .iter()
        .enumerate()
        .for_each(|(idx, cell)| match cell {
            Cell::Empty => print!("{} :empty, ", idx),
            Cell::Filled(_) => print!("{}: filled, ", idx),
        });
    SMA.read().unwrap().agents.iter().for_each(|agent| {
        println!("{}:{}", agent.coordinate().x, agent.coordinate().y);
    });

    SMA.write().unwrap().tick();
}
