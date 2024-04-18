use clock::Clock;

fn main() {
    let mut c = Clock::new(0, 10);
    c = c.add_minutes(-20);

    let clock = Clock::new(2, 20).add_minutes(-3000);
    println!("{}", clock);
}