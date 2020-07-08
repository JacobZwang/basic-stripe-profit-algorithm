fn main() {
    let stripe_flat = 0.3;
    let stripe_percent = 0.029;

    let gitbid_flat = 0.3;
    let gitbid_percent = 0.05;

    let price = [1.0, 2.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0];
    let mut index = 0;

    while index < 5 {
        let profit =
            (gitbid_flat - stripe_flat) + ((price[index]) * (gitbid_percent - stripe_percent));
        println!("{}$ will yield {}$", price[index], profit);
        index += 1;
    }
}
