use super::maths;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero};

use std::fs::File;
use std::io::prelude::*;

pub struct RSA{
    public_key: BigUint,
    private_key: BigUint,
    e: BigUint,
}

impl RSA{
    pub fn new(bits: u32) -> Self{
        loop{
            let p = maths::large_prime(bits/2);
            let q = maths::large_prime(bits/2);
            let n = &p * &q;
    
            let phi_n = &n - &p - &q + 1_u8;
            let e = maths::large_prime(bits/2 +1);
    
            let bez_coef = maths::egcd(e.clone().to_biguint().unwrap(), phi_n); 
            
            if bez_coef.0 > Zero::zero(){
                return Self{
                    e,
                    public_key: n,
                    private_key: bez_coef.0.to_biguint().expect("Problem while creating the private key")
                }
            }
        }
    }

    pub fn from_files(public_key_path: &str, private_key_path: &str) -> std::io::Result<Self>{
        let mut public_key_file = File::open(public_key_path)?;
        let mut private_key_file = File::open(private_key_path)?;

        let mut public_key_all = String::new();
        public_key_file.read_to_string(&mut public_key_all)?;
        
        let mut public_key_split = public_key_all.lines();

        let mut private_key = String::new();
        private_key_file.read_to_string(&mut private_key)?;

        Ok(Self{
            private_key: BigUint::parse_bytes(private_key.as_bytes(), 16).unwrap(),
            public_key: BigUint::parse_bytes(public_key_split.next().unwrap().as_bytes(), 16).unwrap(),
            e: BigUint::parse_bytes(public_key_split.next().unwrap().as_bytes(),16).unwrap(),
        })
    }

    pub fn show_keys(&self){
        println!("Public Key: ({},{}) \nPrivate Key: {}", self.public_key, self.e, self.private_key);
    }

    pub fn encrypt(&self, n: BigUint) -> BigUint{
        n.modpow(&self.e, &self.public_key)
    }

    pub fn decrypt(&self, c: &BigUint) -> BigUint{
        c.modpow(&self.private_key, &self.public_key)
    }

    pub fn save_public_key(&self) -> std::io::Result<()>{
        let mut pub_key_file = File::create("public_key.txt")?;
        pub_key_file.write(format!("{:X}\n{:X}", &self.public_key, &self.e).as_bytes())?;

        Ok(())
    }

    pub fn save_private_key(&self) -> std::io::Result<()>{
        let mut priv_key_file = File::create("private_key.txt")?;
        priv_key_file.write(format!("{:X}", &self.private_key).as_bytes())?;

        Ok(())
    }


}