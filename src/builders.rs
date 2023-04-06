use rand::seq::index;

pub fn vector_with_holes(length: usize, holes: usize) -> Vec<u32> {
    let initial_size = length as u32 + holes as u32 + 1;
    let mut sequence: Vec<u32> = (1..initial_size).collect();

    let mut rng = rand::thread_rng();
    for index in index::sample(&mut rng, length, holes) {
        sequence.remove(index);
    }

    return sequence;
}
