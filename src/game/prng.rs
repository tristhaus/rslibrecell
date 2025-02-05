pub struct Prng {
    pub state: u32,
}

impl Prng {
    pub fn get_next(&mut self) -> u32 {
        self.state = (self.state.wrapping_mul(214013)).wrapping_add(2531011) % 2147483648;
        self.state / 65536
    }
}

#[cfg(test)]
mod test;
