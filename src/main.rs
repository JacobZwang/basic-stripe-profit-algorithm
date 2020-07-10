fn main() {
    let stripe_flat = 0.3;
    let stripe_percent = 0.029;

    let our_flat = 0.3;
    let our_percent = 0.05;

    let price = [1.0, 2.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0];

    for price in price.iter() {
        let profit = (our_flat - stripe_flat) + ((price) * (our_percent - stripe_percent));
        println!("${} will yield ${}", price, profit);
    }
}
