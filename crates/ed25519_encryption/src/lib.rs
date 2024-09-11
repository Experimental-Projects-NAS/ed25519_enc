use neon::prelude::*;
use std::borrow::BorrowMut;
use neon::types::buffer::TypedArray;
use ed25519_to_curve25519::{ed25519_sk_to_curve25519, ed25519_pk_to_curve25519};
use x25519_dalek::x25519;
use aes_gcm::{
    aes::cipher::generic_array::typenum::U12,
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

fn to_x25519_priv_js(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
    let input_array: Handle<JsUint8Array> = cx.argument::<JsUint8Array>(0)?;
    let ed25519_priv: [u8; 32] = input_array.as_slice(&cx).try_into().unwrap();
    let x25519_priv: [u8; 32] = to_x25519_priv(ed25519_priv);
    let mut result: Handle<JsUint8Array> = JsUint8Array::new(& mut cx, 32)?;
    result.borrow_mut().as_mut_slice(&mut cx).copy_from_slice(&x25519_priv);
    Ok(result)
}

fn to_x25519_pub_js(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
    let input_array: Handle<JsUint8Array> = cx.argument::<JsUint8Array>(0)?;
    let ed25519_pub: [u8; 32] = input_array.as_slice(&cx).try_into().unwrap();
    let x25519_pub: [u8; 32] = to_x25519_pub(ed25519_pub);
    let mut result: Handle<JsUint8Array> = JsUint8Array::new(& mut cx, 32)?;
    result.borrow_mut().as_mut_slice(&mut cx).copy_from_slice(&x25519_pub);
    Ok(result)
}

fn encrypt_js(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
    let sender_ed25519_priv: [u8; 32] = cx.argument::<JsUint8Array>(0)?.as_slice(&cx).try_into().unwrap();
    let receiver_ed25519_pub: [u8; 32] = cx.argument::<JsUint8Array>(1)?.as_slice(&cx).try_into().unwrap();
    let data: &[u8] = cx.argument::<JsUint8Array>(2)?.as_slice(&cx).try_into().unwrap();
    let encrypted_data: Vec<u8> = encrypt(sender_ed25519_priv, receiver_ed25519_pub, data);
    let mut result: Handle<JsUint8Array> = JsUint8Array::new(&mut cx, encrypted_data.len())?;
    result.borrow_mut().as_mut_slice(&mut cx).copy_from_slice(&encrypted_data);
    Ok(result)
}

fn decrypt_js(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
    let receiver_ed25519_priv: [u8; 32] = cx.argument::<JsUint8Array>(0)?.as_slice(&cx).try_into().unwrap();
    let sender_ed25519_pub: [u8; 32] = cx.argument::<JsUint8Array>(1)?.as_slice(&cx).try_into().unwrap();
    let enc_msg: &[u8] = cx.argument::<JsUint8Array>(2)?.as_slice(&cx).try_into().unwrap();
    let decrypted_data: Vec<u8> = decrypt(receiver_ed25519_priv, sender_ed25519_pub, enc_msg);
    let mut result: Handle<JsUint8Array> = JsUint8Array::new(&mut cx, decrypted_data.len())?;
    result.borrow_mut().as_mut_slice(&mut cx).copy_from_slice(&decrypted_data);
    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("to_x25519_priv", to_x25519_priv_js)?;
    cx.export_function("to_x25519_pub", to_x25519_pub_js)?;
    cx.export_function("encrypt", encrypt_js)?;
    cx.export_function("decrypt", decrypt_js)?;
    Ok(())
}

pub fn to_x25519_priv(ed25519_priv_key: [u8; 32]) -> [u8; 32] {
    ed25519_sk_to_curve25519(ed25519_priv_key)
}

pub fn to_x25519_pub(ed25519_pub_key: [u8; 32]) -> [u8; 32] {
    ed25519_pk_to_curve25519(ed25519_pub_key)
}

pub fn encrypt(sender_ed25519_priv: [u8; 32], receiver_ed25519_pub: [u8; 32], data: &[u8]) -> Vec<u8> {
    let sender_x25519_priv: [u8; 32] = to_x25519_priv(sender_ed25519_priv);
    let receiver_x25519_pub: [u8; 32] = to_x25519_pub(receiver_ed25519_pub);
    let shared_secret: [u8; 32] = x25519(sender_x25519_priv, receiver_x25519_pub);
    let aes_key: &Key<Aes256Gcm> = &shared_secret.into();
    let cipher: Aes256Gcm = Aes256Gcm::new(&aes_key);
    let nonce: Nonce<U12>  = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher_text: Vec<u8> = cipher.encrypt(&nonce, data).unwrap();
    let enc_data: Vec<u8> = [nonce.as_slice(), cipher_text.as_slice()].concat();
    enc_data
}

pub fn decrypt(receiver_ed25519_priv: [u8; 32], sender_ed25519_pub: [u8; 32], enc_msg: &[u8]) -> Vec<u8> {
    let receiver_x25519_priv: [u8; 32] = to_x25519_priv(receiver_ed25519_priv);
    let sender_x25519_pub: [u8; 32] = to_x25519_pub(sender_ed25519_pub);
    let shared_secret: [u8; 32] = x25519(receiver_x25519_priv, sender_x25519_pub);
    let aes_key: &Key<Aes256Gcm> = &shared_secret.into();
    let nonce_slice: &[u8] = &enc_msg[0..12];
    let enc_data: &[u8] = &enc_msg[12..];
    let cipher: Aes256Gcm = Aes256Gcm::new(&aes_key);
    let nonce: Nonce<U12> = Nonce::<U12>::clone_from_slice(nonce_slice);
    let data: Vec<u8> = cipher.decrypt(&nonce, enc_data).unwrap();
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let sender_ed25519_priv: [u8; 32] = [
            157, 097, 177, 157, 239, 253, 090, 096,
            186, 132, 074, 244, 146, 236, 044, 196,
            068, 073, 197, 105, 123, 050, 105, 025,
            112, 059, 172, 003, 028, 174, 127, 096,
        ];

        let sender_ed25519_pub: [u8; 32] = [
            215, 090, 152, 001, 130, 177, 010, 183,
            213, 075, 254, 211, 201, 100, 007, 058,
            014, 225, 114, 243, 218, 166, 035, 037,
            175, 002, 026, 104, 247, 007, 081, 026,
        ];

        let receiver_ed25519_priv: [u8; 32] = [
            074, 032, 095, 005, 083, 039, 001, 050,
            066, 088, 090, 056, 021, 011, 031, 011,
            087, 068, 033, 014, 025, 028, 010, 043,
            024, 066, 058, 076, 030, 074, 017, 076,
        ];

        let receiver_ed25519_pub: [u8; 32] = [
            204, 095, 038, 061, 080, 055, 143, 219,
            253, 138, 015, 190, 145, 041, 028, 081,
            001, 004, 227, 080, 197, 078, 000, 043,
            160, 130, 158, 047, 197, 058, 228, 232
        ];

        let data: &[u8] = b"Hello world!";

        let enc_msg: Vec<u8> = encrypt(
            sender_ed25519_priv,
            receiver_ed25519_pub,
            data
        );

        let dec_data: Vec<u8> = decrypt(
            receiver_ed25519_priv,
            sender_ed25519_pub,
            &enc_msg
        );

        assert_eq!(data, dec_data);
    }
}