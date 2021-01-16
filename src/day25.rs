pub const MODULO: u128 = 20201227;
pub const SUBJECT_NUMBER: u128 = 7;

pub fn transform(subject_number: u128, loop_size: u128) -> u128 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % MODULO;
    }
    value
}

pub fn crack_transform(subject_number: u128, public_key: u128) -> u128 {
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        value = (value * subject_number) % MODULO;
        loop_size += 1;
        if value == public_key {
            break;
        }
    }
    loop_size
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_transform() {
        assert_eq!(5764801, transform(SUBJECT_NUMBER, 8));
        assert_eq!(17807724, transform(SUBJECT_NUMBER, 11));
    }

    #[test]
    fn test_crack() {
        assert_eq!(8, crack_transform(SUBJECT_NUMBER, 5764801));
        assert_eq!(11, crack_transform(SUBJECT_NUMBER, 17807724));
    }

    #[test]
    fn test_encryption_key() {
        assert_eq!(14897079, transform(17807724, 8));
        assert_eq!(14897079, transform(5764801, 11));
    }

    #[test]
    fn test_part1() {
        let public_keys: (u128, u128) = (18499292, 8790390);
        let loop_sizes = (crack_transform(SUBJECT_NUMBER, public_keys.0),
                          crack_transform(SUBJECT_NUMBER, public_keys.1));
        println!("{:?}", loop_sizes);
        println!("{}", transform(public_keys.0, loop_sizes.1));
    }
}