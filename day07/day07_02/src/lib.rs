fn fuel_cost(xx: i32) -> i32 {
    return ((xx * xx) / 2) + (xx / 2);
}

pub fn solve(numbers: &[i32]) -> u32 {
    let mut dist_low = u32::MAX;
    for xx in 0..=500 {
        let mut dist_now = 0_u32;
        for yy in numbers {
            let dist = (xx - yy).abs();
            let dist = fuel_cost(dist);
            dist_now += dist as u32;
        }
        if dist_low > dist_now {
            dist_low = dist_now;
        }
    }
    dist_low
}
