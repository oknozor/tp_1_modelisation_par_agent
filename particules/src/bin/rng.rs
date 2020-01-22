use particules::sma::Sma;
fn main() {
    let mut sma = Sma::new(500, 500);
    sma.gen_agents(5);
}
