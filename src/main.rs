mod day1;
mod day2;

fn main() {
    let pair = day1::pair();
    day1::hashing();
    day1::signing(&pair);
    day1::derive();

    day2::decrypt_ciphered_message();
    day2::vrf::run();
}
