pub fn nth(n: u32) -> u32
{
    let mut primes: Vec<u32> = vec![2];
    let mut current: u32 = 2;

    while primes.len() <= n as usize
    {
        let mut divided = false;
        for i in primes.iter()
        {
            if matches!(current % i, 0)
            {
                divided = true;
                break;
            }
        }
        if !divided { primes.push(current); }
        current += 1;
    };

    return primes[primes.len() - 1];
}
