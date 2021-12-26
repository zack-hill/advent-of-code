use crate::solver::AoCSolver;

pub struct Solver {
    cups: Vec<usize>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            cups: vec![2, 8, 4, 5, 7, 3, 9, 6, 1],
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut linked_list = convert_to_linked_list(&self.cups);
        move_cups(100, self.cups[0], &mut linked_list);
        let cups = convert_from_linked_list(&linked_list);

        return cups
            .iter()
            .skip(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .concat();
    }

    fn solve_part_2(&self) -> String {
        let mut cups = self.cups.clone();
        // Add remaining cups up through one million
        let max = *cups.iter().max().unwrap();
        cups.extend(max + 1..=1_000_000);

        let mut linked_list = convert_to_linked_list(&cups);
        move_cups(10_000_000, cups[0], &mut linked_list);
        let cups = convert_from_linked_list(&linked_list);

        return (cups[1] * cups[2]).to_string();
    }
}

fn move_cups(move_count: u32, start: usize, linked_list: &mut Vec<usize>) {
    let mut value = start;
    for _ in 0..move_count {
        // println!("cups: {:?}", convert_from_linked_list(&linked_list));
        // println!("current: {}", value);

        let a = linked_list[value - 1];
        let b = linked_list[a - 1];
        let c = linked_list[b - 1];

        // Find dest
        let mut dest = value;
        while dest == a || dest == b || dest == c || dest == value {
            dest = if dest == 1 {
                linked_list.len()
            } else {
                dest - 1
            }
        }

        // Move (a, b, c) to dest
        linked_list[value - 1] = linked_list[c - 1];
        linked_list[c - 1] = linked_list[dest - 1];
        linked_list[dest - 1] = a;

        // Move to next value
        value = linked_list[value - 1];

        // println!("pick up: {:?}", [a, b, c]);
        // println!("destination: {}", dest);
        // println!("");
    }
}

fn convert_to_linked_list(cups: &Vec<usize>) -> Vec<usize> {
    let mut linked_list = Vec::new();
    linked_list.resize(cups.len(), 0);
    for window in cups.windows(2) {
        let current = window[0];
        let next = window[1];
        linked_list[current - 1] = next;
    }
    // Link end of list to beginning
    linked_list[cups[cups.len() - 1] - 1] = cups[0];

    // for i in 0..linked_list.len() {
    //     println!("{} -> {}", i + 1, linked_list[i]);
    // }
    return linked_list;
}

fn convert_from_linked_list(linked_list: &Vec<usize>) -> Vec<usize> {
    let mut vec = Vec::new();
    vec.resize(linked_list.len(), 0);
    let mut num = 1;
    for i in 0..linked_list.len() {
        vec[i] = num;
        num = linked_list[num - 1];
    }
    return vec;
}

#[cfg(test)]
mod tests {

    #[test]
    fn convert_to_linked_list() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let cups_to_move = super::convert_to_linked_list(&cups);

        assert_eq!(vec![2, 5, 8, 6, 4, 7, 3, 9, 1], cups_to_move);
    }

    #[test]
    fn convert_from_linked_list() {
        let linked_list = vec![2, 5, 8, 6, 4, 7, 3, 9, 1];

        let actual = super::convert_from_linked_list(&linked_list);

        assert_eq!(vec![1, 2, 5, 4, 6, 7, 3, 8, 9], actual);
    }

    #[test]
    fn linked_list_round_trip() {
        let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let expected = vec![1, 2, 5, 4, 6, 7, 3, 8, 9];

        let linked_list = super::convert_to_linked_list(&input);
        let actual = super::convert_from_linked_list(&linked_list);

        assert_eq!(expected, actual);
    }
}
