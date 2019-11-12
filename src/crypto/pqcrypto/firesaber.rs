// Copyright (C) 2019 Quentin Kniep <kniepque@hu-berlin.de>
// Distributed under terms of the MIT license.

pub const PQ_SECRET_KEY_SIZE: usize = 3040;
pub const PQ_PUBLIC_KEY_SIZE: usize = 1312;
pub const PQ_CIPHERTEXT_SIZE: usize = 1472;
pub const PQ_SHARED_KEY_SIZE: usize = 32;

#[link(name = "oqs")]
extern {
    #[link_name = "OQS_KEM_saber_firesaber_keypair"]
    pub fn pq_keypair(public_key: *mut u8,
                      secret_key: *mut u8) -> u32;
    #[link_name = "OQS_KEM_saber_firesaber_encaps"]
    pub fn pq_encaps(ciphertext: *mut u8,
                     shared_secret: *mut u8,
                     public_key: *const u8) -> u32;
    #[link_name = "OQS_KEM_saber_firesaber_decaps"]
    pub fn pq_decaps(shared_secret: *mut u8,
                     ciphertext: *const u8,
                     secret_key: *const u8) -> u32;
}
