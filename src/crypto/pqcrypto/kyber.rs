// Copyright (C) 2019 Quentin Kniep <kniepque@hu-berlin.de>
// Distributed under terms of the MIT license.

#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;


pub const PQ_SECRET_KEY_SIZE: usize = 1632;
pub const PQ_PUBLIC_KEY_SIZE: usize = 736;
pub const PQ_CIPHERTEXT_SIZE: usize = 800;
pub const PQ_SHARED_KEY_SIZE: usize = 32;

#[repr(C)]
pub struct PQPublicKey {
    bytes: [u8; PQ_PUBLIC_KEY_SIZE],
}

impl PQPublicKey {
    pub fn encaps(&self) -> (PQCiphertext, PQSharedSecret) {
        unimplemented!()
            // FFI *_encaps()
    }
}

impl fmt::Debug for PQPublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.bytes[..].fmt(formatter)
    }
}

/// Will panic if the slice.len() != PQ_PUBLIC_KEY_SIZE
impl<'a> From<&'a [u8]> for PQPublicKey {
    fn from(slice: &[u8]) -> Self {
        let mut bytes = [0u8; PQ_PUBLIC_KEY_SIZE];
        bytes[..].copy_from_slice(slice);
        PQPublicKey { bytes }
    }
}

#[repr(C)]
pub struct PQSecretKey {
    bytes: [u8; PQ_SECRET_KEY_SIZE],
}

impl fmt::Debug for PQSecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.bytes[..].fmt(formatter)
    }
}

#[repr(C)]
pub struct PQCiphertext {
    bytes: [u8; PQ_CIPHERTEXT_SIZE],
}

impl PQCiphertext {
    pub fn decaps(&self, secret_key: PQSecretKey) -> (PQSharedSecret) {
        unimplemented!()
            // FFI *_decaps()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
}

#[repr(C)]
pub struct PQSharedSecret {
    bytes: [u8; PQ_SHARED_KEY_SIZE],
}

impl PQSharedSecret {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
}

#[repr(C)]
pub struct PQKeyPair {
    pub secret_key: PQSecretKey,
    pub public_key: PQPublicKey,
}

impl PQKeyPair {
    pub fn new() -> PQKeyPair {
        unimplemented!()
            // FFI *_keygen()
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key.bytes[..]
    }
}
