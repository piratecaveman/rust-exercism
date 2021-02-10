pub fn square(s: u32) -> u64 
{
    if !matches!((1..=64).contains(&s), true) { panic!("Square must be between 1 and 64") };
    return (2 as u64).pow(s - 1);   
}

pub fn total() -> u64 
{
    return (1..=64).fold(0u64, |mut sum, x: u32| { sum = sum + square(x); return sum; } );
}
