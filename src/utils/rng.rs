use rand::{Rng, RngExt};

pub trait RngCoinFlip: Rng {
    fn coin_flip(&mut self) -> bool {
        self.random()
    }
}

impl<R: Rng + ?Sized> RngCoinFlip for R {}