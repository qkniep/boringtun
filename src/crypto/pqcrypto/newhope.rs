// Copyright (C) 2019 Quentin Kniep <kniepque@hu-berlin.de>
// Distributed under terms of the MIT license.

pub const SECRET_KEY_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 32;

pub fn generateKeyPair() -> (public_key: [u8; PUBLIC_KEY_SIZE],
                             secret_key: [u8; SECRET_KEY_SIZE]);
