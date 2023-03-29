use groove_core::{control::F32ControlValue, traits::HasUid};
use groove_proc_macros::{Everything, Nano, Uid};
use std::str::FromStr;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumString, FromRepr, IntoStaticStr};

enum AppMessages {
    Wrapper(usize, OtherEntityMessage),
}

#[derive(Clone, Copy, Debug, Default, EnumCountMacro, FromRepr, PartialEq)]
pub enum CherryType {
    #[default]
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
impl From<F32ControlValue> for CherryType {
    fn from(value: F32ControlValue) -> Self {
        CherryType::from_repr((value.0 * CherryType::COUNT as f32) as usize).unwrap_or_default()
    }
}
impl Into<F32ControlValue> for CherryType {
    fn into(self) -> F32ControlValue {
        F32ControlValue((self as usize as f32) / CherryType::COUNT as f32)
    }
}

// #[derive(Clone, Copy, Debug, Default, PartialEq, )]
// pub struct NanoStuff {
//     #[sync]
//     apple_count: usize,
//     #[sync]
//     banana_quality: f32,
//     #[sync]
//     cherry_type: CherryType,
// }
impl NanoStuff {
    fn make_fake() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        Self {
            apple_count: rng.gen_range(5..1000),
            banana_quality: rng.gen_range(0.0..1.0),
            cherry_type: CherryType::from_repr(rng.gen_range(0..CherryType::COUNT)).unwrap(),
        }
    }

    fn make_different_from(other: &Self) -> Self {
        Self {
            apple_count: other.apple_count() + 1,
            banana_quality: (other.banana_quality() + 0.777).fract(),
            cherry_type: other.cherry_type().next_cherry(),
        }
    }
}

#[derive(Debug, Nano, PartialEq, Uid)]
pub struct Stuff {
    uid: usize,

    #[nano]
    apple_count: usize,
    #[nano]
    banana_quality: f32,
    #[nano]
    cherry_type: CherryType,
}

impl Stuff {
    pub fn new(nano: NanoStuff) -> Self {
        let mut r = Self {
            uid: Default::default(),
            apple_count: nano.apple_count(),
            banana_quality: nano.banana_quality(),
            cherry_type: nano.cherry_type(),
        };
        r.precompute();
        r
    }
    pub fn update(&mut self, message: StuffMessage) {
        match message {
            StuffMessage::Stuff(s) => *self = Self::new(s),
            StuffMessage::AppleCount(s) => self.set_apple_count(s),
            StuffMessage::BananaQuality(s) => self.set_banana_quality(s),
            StuffMessage::CherryType(s) => self.set_cherry_type(s),
        }
    }

    fn precompute(&mut self) {
        // This is here as a demo of logic depending on setters/getters
    }

    fn clear_precomputed(&mut self) {
        // This is here as a demo of logic depending on setters/getters
    }

    pub fn apple_count(&self) -> usize {
        self.apple_count
    }

    fn set_apple_count(&mut self, count: usize) {
        self.apple_count = count;
        self.clear_precomputed();
    }

    fn banana_quality(&self) -> f32 {
        self.banana_quality
    }

    fn set_banana_quality(&mut self, banana_quality: f32) {
        self.banana_quality = banana_quality;
        self.clear_precomputed();
    }

    fn cherry_type(&self) -> CherryType {
        self.cherry_type
    }

    fn set_cherry_type(&mut self, cherry_type: CherryType) {
        self.cherry_type = cherry_type;
        self.clear_precomputed();
    }
}

impl NanoMisc {
    fn make_fake() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        Self {
            cat_count: rng.gen_range(5..1000),
            dog_count: rng.gen_range(5..1000),
        }
    }
}

#[derive(Debug, Nano, Uid)]
pub struct Misc {
    uid: usize,

    #[nano]
    cat_count: usize,
    #[nano]
    dog_count: usize,
}
impl Misc {
    pub fn new_with(params: NanoMisc) -> Self {
        Self {
            uid: Default::default(),
            cat_count: params.cat_count(),
            dog_count: params.dog_count(),
        }
    }
    pub fn update(&mut self, message: MiscMessage) {
        match message {
            MiscMessage::Misc(s) => *self = Self::new_with(s),
            MiscMessage::CatCount(s) => self.set_cat_count(s),
            MiscMessage::DogCount(s) => self.set_dog_count(s),
        }
    }

    pub fn cat_count(&self) -> usize {
        self.cat_count
    }

    pub fn set_cat_count(&mut self, cat_count: usize) {
        self.cat_count = cat_count;
    }

    pub fn dog_count(&self) -> usize {
        self.dog_count
    }

    pub fn set_dog_count(&mut self, dog_count: usize) {
        self.dog_count = dog_count;
    }
}

type MsgType = OtherEntityMessage;
#[derive(Everything)]
enum Models {
    Stuff(Stuff),
    Misc(Misc),
}

#[cfg(test)]
mod tests {
    use super::*;
    use groove_core::traits::Controllable;

    #[test]
    fn update_full() {
        let a = NanoStuff::make_fake();
        let mut b = NanoStuff::make_different_from(&a);
        assert_ne!(a, b);
        b.update(StuffMessage::Stuff(a.clone()));
        assert_eq!(a, b);
    }

    #[test]
    fn update_incrementally() {
        let mut a = NanoStuff::make_fake();
        let mut b = NanoStuff::make_different_from(&a);
        assert_ne!(a, b);
        let message = StuffMessage::AppleCount(a.apple_count() + 1);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffMessage::BananaQuality(b.banana_quality() / 3.0);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffMessage::CherryType(a.cherry_type().next_cherry());
        a.update(message.clone());
        b.update(message);
        assert_eq!(a, b);
    }

    #[test]
    fn update_incrementally_with_full_structs() {
        let a_params = NanoStuff::make_fake();
        let b_params = NanoStuff::make_different_from(&a_params);
        let mut a = Stuff::new(a_params);
        let mut b = Stuff::new(b_params);
        assert_ne!(a, b);
        let message = StuffMessage::AppleCount(a.apple_count() + 1);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffMessage::BananaQuality(b.banana_quality() / 3.0);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffMessage::CherryType(a.cherry_type().next_cherry());
        a.update(message.clone());
        b.update(message);
        assert_eq!(a, b);
    }

    #[test]
    fn control_params_by_name() {
        let a_params = NanoStuff::make_fake();
        let b_params = NanoStuff::make_different_from(&a_params);
        let a = Stuff::new(a_params);
        let mut b = Stuff::new(b_params);
        assert_ne!(a, b);

        if let Some(message) = b.message_for_name("apple-count", a.apple_count().into()) {
            b.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b.message_for_name("banana-quality", a.banana_quality().into()) {
            b.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b.message_for_name("cherry-type", a.cherry_type().into()) {
            b.update(message);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn control_params_by_index() {
        let a_params = NanoStuff::make_fake();
        let b_params = NanoStuff::make_different_from(&a_params);
        let a = Stuff::new(a_params);
        let mut b = Stuff::new(b_params);
        assert_ne!(a, b);

        // We exclude the full message from the index.
        assert_eq!(a.control_index_count(), 3);

        if let Some(message) = b.message_for_index(0, a.apple_count().into()) {
            b.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b.message_for_index(1, a.banana_quality().into()) {
            b.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b.message_for_index(2, a.cherry_type().into()) {
            b.update(message);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn control_ergonomics() {
        let a = Stuff::new(NanoStuff::make_fake());

        assert_eq!(a.control_name_for_index(2), Some("cherry-type"));
        assert_eq!(a.control_index_count(), 3);
        assert_eq!(a.control_name_for_index(a.control_index_count()), None);

        let a = NanoMisc::make_fake();

        assert_eq!(a.control_name_for_index(0), Some("cat-count"));
        assert_eq!(a.control_index_count(), 2);
        assert_eq!(a.control_name_for_index(a.control_index_count()), None);
    }

    #[test]
    fn core_struct_gets_notifications() {
        // This test used to do something intricate with the precompute logic in
        // Stuff. It got more complicated than necessary for this small test
        // suite. This is a memorial of that idea.
    }

    #[test]
    fn build_views() {
        let entities = vec![
            EntityParams::Stuff(Box::new(NanoStuff::make_fake())),
            EntityParams::Misc(Box::new(NanoMisc::make_fake())),
            EntityParams::Misc(Box::new(NanoMisc::make_fake())),
        ];

        // Build custom views from entity getters
        for entity in entities.iter() {
            match entity {
                EntityParams::Stuff(_e) => {}
                EntityParams::Misc(_e) => {}
            }
        }

        // Build an automation matrix
        for _ in entities.iter().filter(|e| e.is_controller()) {
            // if entity implements controller trait, add it to sources
            eprintln!("adding controller");
        }
        for entity in entities.iter().filter(|e| e.is_controllable()) {
            eprintln!("adding controllable");
            let controllable = entity.as_controllable().unwrap();
            for index in 0..controllable.control_index_count() {
                if let Some(point_name) = controllable.control_name_for_index(index) {
                    eprintln!("adding control point {}", point_name);
                } else {
                    eprintln!("couldn't find name for control point #{}", index);
                }
            }
        }
    }

    #[test]
    fn handle_app_updates() {
        let mut entities = vec![
            EntityParams::Stuff(Box::new(NanoStuff::make_fake())),
            EntityParams::Misc(Box::new(NanoMisc::make_fake())),
            EntityParams::Misc(Box::new(NanoMisc::make_fake())),
        ];

        // Connect two things
        // send message: connect(source, dest, index)
        // send message: disconnect(source, dest, index)

        // Handle an incoming message
        let message = StuffMessage::AppleCount(45);
        let wrapped_message = AppMessages::Wrapper(1, OtherEntityMessage::Stuff(message));

        let AppMessages::Wrapper(uid, message) = wrapped_message;
        let entity = &mut entities[uid];
        match message {
            OtherEntityMessage::Stuff(message) => {
                if let EntityParams::Stuff(entity) = entity {
                    entity.update(message);
                }
            }
            OtherEntityMessage::Misc(message) => {
                if let EntityParams::Misc(entity) = entity {
                    entity.update(message);
                }
            }
        }
    }

    #[test]
    fn engine_usage() {
        let a = Stuff::new(NanoStuff::make_fake());
        let next_cherry = a.cherry_type().next_cherry();
        let mut ea = Entity::Stuff(Box::new(a));

        if let Some(message) = ea.message_for(0, 50.0.into()) {
            ea.update(message);
        }
        if let Some(message) = ea.message_for(1, 0.14159265.into()) {
            ea.update(message);
        }
        if let Some(message) = ea.message_for(2, next_cherry.into()) {
            ea.update(message);
        }

        if let Entity::Stuff(a) = ea {
            assert_eq!(a.apple_count(), 50);
            assert_eq!(a.banana_quality(), 0.14159265);
            assert_eq!(a.cherry_type(), next_cherry);
        }
    }
}
