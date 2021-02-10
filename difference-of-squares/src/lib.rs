pub fn square_of_sum(n: u32) -> u32 
{
    let sum: u32 = (0..=n).sum::<u32>().pow(2);
    return sum;
}

pub fn sum_of_squares(n: u32) -> u32 
{
    let sum: u32 = (0..=n).map(|x: u32| { x.pow(2) }).sum::<u32>();
    return sum;
}

pub fn difference(n: u32) -> u32 
{
    return square_of_sum(n) - sum_of_squares(n);
}
