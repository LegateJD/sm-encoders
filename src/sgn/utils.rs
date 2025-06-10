use rand::{seq::IndexedRandom, Rng};

pub fn coin_flip() -> bool {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_label(length: usize) -> String {
    const CUSTOM_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::rng();

    CUSTOM_CHARSET
        .choose_multiple(&mut rng, length)
        .map(|&b| b as char)
        .collect()
}