fn main() {
    //let start = [3, 4, 3, 1, 2];
    let start: [u8; 300] = [
        1, 4, 1, 1, 1, 1, 1, 1, 1, 4, 3, 1, 1, 3, 5, 1, 5, 3, 2, 1, 1, 2, 3, 1, 1, 5, 3, 1, 5, 1,
        1, 2, 1, 2, 1, 1, 3, 1, 5, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 4, 5, 3, 1, 1, 1, 1, 1, 1, 2, 1,
        1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 5, 1, 2, 4, 1, 1, 4, 1, 2, 1, 1, 1, 2, 1, 5, 1, 1, 1, 3, 4,
        1, 1, 1, 3, 2, 1, 1, 1, 4, 1, 1, 1, 5, 1, 1, 4, 1, 1, 2, 1, 4, 1, 1, 1, 3, 1, 1, 1, 1, 1,
        3, 1, 3, 1, 1, 2, 1, 4, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 4, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 5, 1, 1, 1, 2, 2, 1, 1, 3, 5, 1, 1, 1, 1, 3, 1, 3, 3,
        1, 1, 1, 1, 3, 5, 2, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1,
        1, 2, 1, 1, 1, 1, 1, 5, 1, 4, 3, 3, 1, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 3, 5, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 2, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1,
        1, 1, 1, 1, 1, 1, 1, 2, 1, 4, 4, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 2, 5, 1, 1, 4, 1, 3, 1, 1,
    ];
    println!("{}", start.len());
    let mut chunks = start.chunks(300 / 12);
    println!("{}", chunks.len());

    let mut threads = Vec::new();
    for (ii, mut chunk) in chunks.enumerate() {
        let mut fishes = chunk.to_vec();
        let t = std::thread::spawn(move || {
            let chunks = fishes.chunks(fishes.len() / 25);
            let mut count_local = 0_u128;
            let mut new_fishes = Vec::new();
            for (jj, chunk) in chunks.enumerate() {
                let mut fishes = chunk.to_vec();
                for day in 0..256 {
                    println!("thread {:5} day {:5} chunk {:5} fishes {:20}", ii, day, jj, fishes.len());
                    for fish in fishes.iter_mut() {
                        if *fish == 0 {
                            new_fishes.push(8);
                            *fish = 6;
                        } else {
                            *fish = *fish - 1;
                        }
                    }
                    fishes.extend_from_slice(&mut new_fishes);
                    new_fishes = Vec::new();
                }
                count_local += fishes.len() as u128;
            }
            count_local
        });
        threads.push(t);
    }
    let mut all = 0_u128;
    for tt in threads {
        let count = tt.join().unwrap();
        all += count;
    }
    println!("{}", all);
}
