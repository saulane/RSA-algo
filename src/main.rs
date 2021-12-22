use num_bigint::{ToBigUint, BigUint};

use hex;

mod maths;
mod rsa;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    match args.len(){
        1 => {
            println!("We can do what do we want to do");
        },
        2 => {
            match args[1].as_str(){
                "new" => {
                    let key = rsa::RSA::new(3072);
                    key.save_private_key().expect("Error saving private key to file");
                    key.save_public_key().expect("Error saving public key to file");
                },
                _ => {}
            }
        },
        _ => ()
    }


    let key = rsa::RSA::from_files("public_key.txt", "private_key.txt").unwrap();
    key.show_keys();

    // key.save_public_key();
    // key.save_private_key();

    let m = BigUint::from(50_u32);

    let c = key.encrypt(50.to_biguint().unwrap());

    println!("Cipher of {} is {}",&m, &c);

    let decrypted = key.decrypt(&c);

    println!("Decrypted value of {} is {}",&c ,&decrypted);
}