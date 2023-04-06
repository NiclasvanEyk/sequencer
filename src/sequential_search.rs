/// Finds the next integer to insert into the sequence.
///
/// The sequence is assumed to be sorted, positive and starts at 1.
pub fn next_number(sequence: &[u32]) -> (u32, u32) {
    if sequence.is_empty() {
        return (0, 1);
    }

    for (index, number) in sequence.iter().enumerate() {
        let expected_number = (index + 1) as u32;
        if *number == expected_number {
            continue;
        }

        if *number < expected_number {
            return (index as u32, index as u32 + 2);
        }

        return (index as u32, expected_number);
    }

    return (sequence.len() as u32, sequence.len() as u32 + 1);
}

/// Fills the next available hole in the sequence and returns the filled
/// number.
///
/// If there are no holes, it just appends the next number to the end of the
/// sequence and returns it.
pub fn fill_sequence(sequence: &mut Vec<u32>) -> u32 {
    let (_, next) = next_number(sequence);
    sequence.insert((next - 1) as usize, next);

    return next;
}

#[cfg(test)]
mod tests {
    use super::{fill_sequence, next_number};

    #[test]
    fn it_works_for_empty_case() {
        let empty: Vec<u32> = Vec::new();
        assert_eq!(1, next_number(empty.as_slice()));
    }

    #[test]
    fn it_works_for_sole_case() {
        assert_eq!(2, next_number(vec![1].as_slice()));
    }

    #[test]
    fn it_works_for_two_with_hole() {
        assert_eq!(2, next_number(vec![1, 3].as_slice()));
    }

    #[test]
    fn it_works_for_three_with_hole() {
        assert_eq!(3, next_number(vec![1, 2, 4].as_slice()));
    }

    #[test]
    fn it_chooses_the_smallest_hole() {
        assert_eq!(2, next_number(vec![1, 3, 4, 6].as_slice()));
        assert_eq!(2, next_number(vec![1, 4, 6].as_slice()));
    }

    #[test]
    fn it_works_when_not_starting_at_1() {
        let mut sequence = vec![6];

        let mut next = fill_sequence(&mut sequence);
        assert_eq!(1, next);

        next = fill_sequence(&mut sequence);
        assert_eq!(2, next);

        next = fill_sequence(&mut sequence);
        assert_eq!(3, next);
    }

    #[test]
    fn it_chooses_works_with_large_vectors() {
        let mut sequence: Vec<u32> = (1..1001).collect();
        let mut end: Vec<u32> = (1002..2456).collect();
        sequence.append(&mut end);

        assert_eq!(1001, next_number(sequence.as_slice()));
    }
}
