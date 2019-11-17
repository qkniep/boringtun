// Copyright (c) 2019 Quentin Kniep. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

use std::fmt;

mod tests;

mod firesaber;
pub use firesaber::*;


#[repr(C)]
#[derive(Clone, Copy)]
pub struct PQPublicKey {
    pub bytes: [u8; PQ_PUBLIC_KEY_SIZE],
}

impl PQPublicKey {
    pub fn encaps(&self) -> (PQCiphertext, PQSharedSecret) {
        let mut ciphertext = PQCiphertext { bytes: [0; PQ_CIPHERTEXT_SIZE] };
        let mut shared_secret = PQSharedSecret { bytes: [0; PQ_SHARED_KEY_SIZE] };
        unsafe {
            pq_encaps(&mut ciphertext.bytes[0],
                      &mut shared_secret.bytes[0],
                      &self.bytes[0]);
        }
        (ciphertext, shared_secret)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
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
    pub bytes: [u8; PQ_SECRET_KEY_SIZE],
}

impl PQSecretKey {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
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
    pub fn decaps(&self, secret_key: &PQSecretKey) -> PQSharedSecret {
        let mut shared_secret = PQSharedSecret { bytes: [0; PQ_SHARED_KEY_SIZE] };
        unsafe {
            pq_decaps(&mut shared_secret.bytes[0],
                      &self.bytes[0],
                      &secret_key.bytes[0]);
        }
        shared_secret
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
}

/// Will panic if the slice.len() != PQ_CIPHERTEXT_SIZE
impl<'a> From<&'a [u8]> for PQCiphertext {
    fn from(slice: &[u8]) -> Self {
        let mut bytes = [0u8; PQ_CIPHERTEXT_SIZE];
        bytes[..].copy_from_slice(slice);
        PQCiphertext { bytes }
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
    pub public_key: PQPublicKey,
    pub secret_key: PQSecretKey,
}

impl PQKeyPair {
    pub fn new() -> PQKeyPair {
        let mut public_key = PQPublicKey { bytes: [0; PQ_PUBLIC_KEY_SIZE] };
        let mut secret_key = PQSecretKey { bytes: [0; PQ_SECRET_KEY_SIZE] };
        unsafe {
            while pq_keypair(&mut public_key.bytes[0],
                             &mut secret_key.bytes[0]) != 0 {};
        }
        //println!("PUBLIC PQ KEY:\n{:?}\n\nSECRET PQ KEY:\n{:?}", public_key, secret_key);
        PQKeyPair { public_key, secret_key }
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key.bytes[..]
    }
}
