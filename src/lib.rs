use struct_sync_macros::Synchronization;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, FromRepr};

// use `cargo expand` to see what the macro is generating.

#[derive(Clone, Copy, Debug, EnumCountMacro, FromRepr, PartialEq)]
pub enum CherryType {
    Bing,
    Black,
    Cornelian,
    Maraschino,
    QueenAnne,
    Ranier,
    Sour,
    Sweet,
    Van,
    Yellow,
}
impl CherryType {
    fn next_cherry(&self) -> Self {
        CherryType::from_repr((*self as usize + 1) % CherryType::COUNT).unwrap()
    }
}

#[derive(Synchronization, Clone, Debug, PartialEq)]
pub struct Stuff {
    #[sync]
    apple_count: usize,
    #[sync]
    banana_quality: f32,
    #[sync]
    cherry_type: CherryType,
}

impl Stuff {
    fn apple_count(&self) -> usize {
        self.apple_count
    }

    //  #[sync_setter]
    fn set_apple_count(&mut self, count: usize) {
        self.apple_count = count;
    }

    fn make_fake() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        Self {
            apple_count: rng.gen_range(5..1000),
            banana_quality: rng.gen_range(0.0..1.0),
            cherry_type: CherryType::from_repr(rng.gen_range(0..CherryType::COUNT)).unwrap(),
        }
    }

    fn banana_quality(&self) -> f32 {
        self.banana_quality
    }

    fn set_banana_quality(&mut self, banana_quality: f32) {
        self.banana_quality = banana_quality;
    }

    fn cherry_type(&self) -> CherryType {
        self.cherry_type
    }

    fn set_cherry_type(&mut self, cherry_type: CherryType) {
        self.cherry_type = cherry_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_update() {
        let a = Stuff::make_fake();
        let mut b = Stuff::make_fake();
        assert_ne!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);

        b.handle_message(StuffMessage::Stuff(a.clone()));
        assert_eq!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);
    }

    #[test]
    fn incremental_updates() {
        let mut a = Stuff::make_fake();
        let mut b = Stuff::make_fake();
        assert_ne!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);

        if let Some(message) = a.set_and_propagate_apple_count(a.apple_count() + 1) {
            b.handle_message(message);
        }
        assert_ne!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);
        if let Some(message) = a.set_and_propagate_banana_quality(a.banana_quality() / 2.0) {
            b.handle_message(message);
        }
        assert_ne!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);
        if let Some(message) = a.set_and_propagate_cherry_type(a.cherry_type().next_cherry()) {
            b.handle_message(message);
        }
        assert_eq!(a, b);
        eprintln!("A: {:?}\r\nB: {:?}\r\n", a, b);
    }
}
