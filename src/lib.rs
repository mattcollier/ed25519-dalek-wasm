use wasm_bindgen::prelude::*;
// use ed25519_dalek::{PublicKey, Signer, Verifier, Signature};
use ed25519_dalek::{PublicKey, Verifier};
use std::convert::TryInto;
// use web_sys::console;

#[wasm_bindgen]
pub fn verify(public_key_bytes: &[u8], message: &[u8], signature_bytes: &[u8]) -> bool {
    let s: [u8; 64] = signature_bytes.try_into().unwrap();

    // let t = format!("{:?}", s);
    // console::log_2(&"SIGNATURE".into(), &t.into());
    let signature = ed25519_dalek::Signature::new(s);
    // let u = format!("{:?}", signature.to_bytes());
    // console::log_2(&"SIGNATURE_to_bytes".into(), &u.into());
    let verify_key = PublicKey::from_bytes(public_key_bytes).unwrap();

    return verify_key.verify(message, &signature).is_ok()
}
