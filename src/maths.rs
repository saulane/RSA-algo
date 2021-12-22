use rand::thread_rng;
use num_bigint::{self, BigUint, RandBigInt, ToBigInt, BigInt};
use num_traits::{Zero, One};

const FIRST_PRIMES: [u128; 70] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
31, 37, 41, 43, 47, 53, 59, 61, 67,
71, 73, 79, 83, 89, 97, 101, 103,
107, 109, 113, 127, 131, 137, 139,
149, 151, 157, 163, 167, 173, 179,
181, 191, 193, 197, 199, 211, 223,
227, 229, 233, 239, 241, 251, 257,
263, 269, 271, 277, 281, 283, 293,
307, 311, 313, 317, 331, 337, 347, 349];

pub fn n_bit_rand(n:u32) -> BigUint{
    let deux = BigUint::from(2_u32);
    return thread_rng().gen_biguint_range(&(deux.pow(n-1)+1_u32), &(deux.pow(n)-1_u32))
}

pub fn get_low_level_primes(n:u32) -> BigUint{
    loop{
        let pc = n_bit_rand(n);

        for d in FIRST_PRIMES{
            if &pc%d == Zero::zero(){
                break
            }else{
                return pc
            }
        }
    }
}

pub fn miller_rabin(n: &BigUint, k: u16) -> bool{
    let mut s: u32 = 0;
    let mut d: BigUint = n - 1u8;

    while &d % 2_u8 ==  Zero::zero(){
        d>>=1;
        s += 1;
    }

    for _ in 0..k{
        let a = thread_rng().gen_biguint_below(&(n-4_u16)) + 2_u16;
        if temoin_de_miller(n, &a,&s ,&d){
            return false
        }
    }
    true
}

fn temoin_de_miller(n: &BigUint, a: &BigUint, s: &u32, d: &BigUint) -> bool{
    let mut x = a.modpow(d, n);
    
    if x == One::one() || x == n-1_u8{
        return false
    }

    for _ in 0..(s-1_u32){
        x = x.pow(2_u32) % n;
        if x == n - 1_u8{
            return false
        }
    }
    
    true
}

pub fn large_prime(n:u32) -> BigUint{
    loop{
        let pc = get_low_level_primes(n);

        if !miller_rabin(&pc, 25){
            continue;
        }else{
            return pc
        }
    }
}

pub fn egcd(x1: BigUint, y1:BigUint) -> (BigInt, BigInt){
    let mut x = x1.clone().to_bigint().unwrap();
    let mut y = y1.clone().to_bigint().unwrap();

    let (mut a0, mut a1, mut b0, mut b1) = (One::one(),Zero::zero(),Zero::zero(),One::one());

    while y != Zero::zero(){
        let (q,r) = (&x / &y, &x % &y);
        let (c,d) = (a0 - &q * &a1, b0 -&q *&b1);

        x=y;
        y=r;
        a0=a1;
        a1=c;
        b0=b1;
        b1=d;
    }

    (a0,b0)
}