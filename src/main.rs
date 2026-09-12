use rand::random_range;

const BYTES_PER_NUMBER: usize = size_of::<u32>();
const PRIMES: &[u8] = include_bytes!("../data/primes.bin");

fn main() {
    let prime = get_random_prime();
    println!("{}", prime);
}

fn get_random_prime() -> u32 {
    let nb_elements = PRIMES.len() / BYTES_PER_NUMBER;
    let offset = random_range(0..nb_elements) * BYTES_PER_NUMBER;
    let buf: [u8; BYTES_PER_NUMBER] = PRIMES[offset..offset + BYTES_PER_NUMBER]
        .try_into()
        .unwrap();

    u32::from_le_bytes(buf)
}
