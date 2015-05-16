
extern crate sodiumoxide;
use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::asymmetricbox;

extern crate base32;
use base32::*;

mod lob;

fn main() {
    sodiumoxide::init();
    let hash::sha256::Digest(d) = hash::sha256::hash("Hello, world!".as_bytes());

    let rd = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &d);
    println!("{}", rd);
    println!("{}", 'A' as u8);
    let dr = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &rd);
    println!("{:?}", dr);

    // Generate the key pair
    let (asymmetricbox::curve25519xsalsa20poly1305::PublicKey(pubk), _privk) =
        asymmetricbox::curve25519xsalsa20poly1305::gen_keypair();

    // Generate intermediate key
    let hash::sha256::Digest(intermedk) = hash::sha256::hash(&pubk);

    // Push rollup and intermediate into a vector
    let mut rollup: Vec<u8> = Vec::new();
    rollup.push(0x1a);
    rollup.extend(intermedk.iter().cloned());

    // Generate digest and then base32 encode
    let hash::sha256::Digest(finalk) = hash::sha256::hash(&rollup[..]);
    let kd = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &finalk);
    println!("Key: {}", kd);

    /*    js hash = sha256(0x1a) hash = sha256(hash + base32decode(")) hash = sha256(hash + 0x3a) hash = sha256(hash + base32decode("ckczcg2fq5hhaksfqgnm44xzheku6t7c4zksbd3dr4wffdvvem6q")) print base32encode(hash) "27ywx5e5ylzxfzxrhptowvwntqrd3jhksyxrfkzi6jfn64d3lwxa"*/
    
    // lob packet rigging
    let p = lob::parse("test".as_bytes());
    let b = lob::raw(&p);
    println!("{}", String::from_utf8(b).ok().unwrap());
}



