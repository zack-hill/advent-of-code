use crate::solver::AoCSolver;

pub struct Solver {
    public_keys: Vec<u64>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            public_keys: vec![9093927, 11001876],
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let loop_size = find_loop_size(self.public_keys[0]);
        let encryption_key = loop_transform(self.public_keys[1], loop_size);
        return encryption_key.to_string();
    }

    fn solve_part_2(&self) -> String {
        return 0.to_string();
    }
}

fn find_loop_size(public_key: u64) -> u64 {
    let mut value = 1;
    let mut loop_count = 0;
    while value != public_key {
        value = transform(7, value);
        loop_count += 1;
    }
    return loop_count;
}

fn transform(subject: u64, value: u64) -> u64 {
    (value * subject) % 20201227
}

fn loop_transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform(subject, value);
    }
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_public_key() {
        assert_eq!(5764801, loop_transform(7, 8));
    }

    #[test]
    fn door_public_key() {
        assert_eq!(17807724, loop_transform(7, 11));
    }

    #[test]
    fn encryption_key_from_door_public_key() {
        assert_eq!(14897079, loop_transform(17807724, 8));
    }

    #[test]
    fn encryption_key_from_card_public_key() {
        assert_eq!(14897079, loop_transform(5764801, 11));
    }

    #[test]
    fn find_card_loop_size() {
        assert_eq!(8, find_loop_size(5764801))
    }

    #[test]
    fn find_door_loop_size() {
        assert_eq!(11, find_loop_size(17807724))
    }
}
