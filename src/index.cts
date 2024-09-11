// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function to_x25519_priv(ed25519_priv: Uint8Array): Uint8Array;
  function to_x25519_pub(ed25519_pub: Uint8Array): Uint8Array;
  function encrypt(sender_priv: Uint8Array, receiver_pub: Uint8Array, data: Uint8Array): Uint8Array;
  function decrypt(receiver_priv: Uint8Array, sender_pub: Uint8Array, data: Uint8Array): Uint8Array;
}

export function to_x25519_priv(ed25519_priv: Uint8Array): Uint8Array {
  const x25519_priv = addon.to_x25519_priv(ed25519_priv);
  return x25519_priv;
}

export function to_x25519_pub(ed25519_pub: Uint8Array): Uint8Array {
  const x25519_pub = addon.to_x25519_pub(ed25519_pub);
  return x25519_pub;
}

export function encrypt(sender_priv: Uint8Array, receiver_pub: Uint8Array, data: Uint8Array): Uint8Array {
  const encrypted = addon.encrypt(sender_priv, receiver_pub, data);
  return encrypted;
}

export function decrypt(receiver_priv: Uint8Array, sender_pub: Uint8Array, data: Uint8Array): Uint8Array {
  const decrypted = addon.decrypt(receiver_priv, sender_pub, data);
  return decrypted;
}