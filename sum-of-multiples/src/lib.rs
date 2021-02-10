use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32
{
    let mut multiples: HashSet<u32> = HashSet::new();
    for factor in factors
    {
        if matches!(factor, 0) { continue; };
        let multiple = (0..limit).filter(|x| { x % factor == 0 });
        multiples.extend(multiple);
    }
    
    let sum: u32 = multiples.into_iter().sum::<u32>();
    return sum;
}
