fn main() {
    let stripe_flat = 0.3;
    let stripe_percent = 0.029;

    let gitbid_flat = 0.3;
    let gitbid_percent = 0.05;

    let mut counter = 0.0;

    loop {
        counter += 1.0;
        let profit = (gitbid_flat - stripe_flat) + ((counter) * (gitbid_percent - stripe_percent));
        println!("{}$ will yield {}$", counter, profit);
        if counter == 100.0 {
            break;
        }
    }
}
