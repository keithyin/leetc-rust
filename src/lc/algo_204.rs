
pub fn count_primes(n: i32) -> i32 {

    // 0: prime, 1: non prime
    let mut primes_tag = vec![0; (n+1) as usize];

    for i in 2..(n as usize + 1) {
        if (i * i) > (n as usize) {
            break;
        }
        for j in i..(n as usize + 1) {
            if (i * j) <= (n as usize) {
                primes_tag[i * j] = 1;
            } else {
                break;
            }
        }
    }
    let mut primes_count = 0;
    for i in 2..=n as usize {
        primes_count += (1-primes_tag[i]);
    }
    primes_count
}