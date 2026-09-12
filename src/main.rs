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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_prime() {
        for _ in 0..100 {
            let prime = get_random_prime();
            assert!(is_prime(prime));
        }
    }

    fn is_prime(x: u32) -> bool {
        if x < 2 {
            return false;
        }

        if x == 2 {
            return true;
        }

        if x % 2 == 0 {
            return false;
        }

        let max = (x as f32).sqrt().ceil() as u32;
        let mut i = 3;

        while i <= max {
            if x % i == 0 {
                return false;
            }

            i += 2;
        }

        true
    }
}
