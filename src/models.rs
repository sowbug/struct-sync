use crate::{all_entities, Controllable, Controller};
use groove_core::control::F32ControlValue;
use std::str::FromStr;
use struct_sync_macros::Synchronization;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumString, FromRepr, IntoStaticStr};

enum AppMessages {
    Wrapper(usize, EntityMessage),
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

#[derive(Clone, Debug, Default, PartialEq, Synchronization)]
pub struct StuffParams {
    #[sync]
    apple_count: usize,
    #[sync]
    banana_quality: f32,
    #[sync]
    cherry_type: CherryType,
}
impl Controller for StuffParams {}
impl StuffParams {
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

    fn apple_count(&self) -> usize {
        self.apple_count
    }

    fn set_apple_count(&mut self, count: usize) {
        self.apple_count = count;
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

#[derive(Debug, PartialEq)]
pub struct Stuff {
    params: StuffParams,
    computed_data: bool, // true = computed, false = purged
}

impl Stuff {
    pub fn new(params: StuffParams) -> Self {
        let mut r = Self {
            params,
            computed_data: false,
        };
        r.precompute();
        r
    }

    pub fn params(&self) -> &StuffParams {
        &self.params
    }

    pub fn update(&mut self, message: StuffParamsMessage) {
        self.params.update(message)
    }

    fn precompute(&mut self) {
        self.computed_data = true;
    }
    fn clear_precomputed(&mut self) {
        self.computed_data = false;
    }

    fn apple_count(&self) -> usize {
        self.params.apple_count()
    }

    fn set_apple_count(&mut self, count: usize) {
        self.clear_precomputed();
        self.params.set_apple_count(count);
    }

    fn banana_quality(&self) -> f32 {
        self.params.banana_quality()
    }

    fn set_banana_quality(&mut self, banana_quality: f32) {
        self.clear_precomputed();
        self.params.set_banana_quality(banana_quality);
    }

    fn cherry_type(&self) -> CherryType {
        self.params.cherry_type()
    }

    fn set_cherry_type(&mut self, cherry_type: CherryType) {
        self.clear_precomputed();
        self.params.set_cherry_type(cherry_type);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Synchronization)]
pub struct MiscParams {
    #[sync]
    cat_count: usize,
    #[sync]
    dog_count: usize,
}
impl MiscParams {
    fn make_fake() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        Self {
            cat_count: rng.gen_range(5..1000),
            dog_count: rng.gen_range(5..1000),
        }
    }

    pub fn set_control_cat_count(&mut self, v: F32ControlValue) {
        self.set_cat_count((v.0 * 10.0) as usize);
    }

    pub fn set_control_dog_count(&mut self, v: F32ControlValue) {
        self.set_dog_count((v.0 * 10.0) as usize);
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

struct Misc {
    params: MiscParams,
}
impl Misc {
    pub fn update(&mut self, message: MiscParamsMessage) {
        self.params.update(message)
    }
}

use crate::register_impl;
all_entities! {
    // struct; params; message; is_controller; is_controllable,
    Stuff; StuffParams; StuffParamsMessage; true; false,
    Misc; MiscParams; MiscParamsMessage; false; true,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_full() {
        let a = StuffParams::make_fake();
        let mut b = StuffParams::make_different_from(&a);
        assert_ne!(a, b);
        b.update(StuffParamsMessage::StuffParams(a.clone()));
        assert_eq!(a, b);
    }

    #[test]
    fn update_incrementally() {
        let mut a = StuffParams::make_fake();
        let mut b = StuffParams::make_different_from(&a);
        assert_ne!(a, b);
        let message = StuffParamsMessage::AppleCount(a.apple_count() + 1);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffParamsMessage::BananaQuality(b.banana_quality() / 3.0);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffParamsMessage::CherryType(a.cherry_type().next_cherry());
        a.update(message.clone());
        b.update(message);
        assert_eq!(a, b);
    }

    #[test]
    fn update_incrementally_with_full_structs() {
        let mut a = Stuff::new(StuffParams::make_fake());
        let mut b = Stuff::new(StuffParams::make_different_from(&a.params()));
        assert_ne!(a, b);
        let message = StuffParamsMessage::AppleCount(a.apple_count() + 1);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffParamsMessage::BananaQuality(b.banana_quality() / 3.0);
        a.update(message.clone());
        b.update(message);
        assert_ne!(a, b);

        let message = StuffParamsMessage::CherryType(a.cherry_type().next_cherry());
        a.update(message.clone());
        b.update(message);
        assert_eq!(a, b);
    }

    #[test]
    fn control_params() {
        let a = Stuff::new(StuffParams::make_fake());
        let mut b = Stuff::new(StuffParams {
            apple_count: a.apple_count() + 1,
            banana_quality: a.banana_quality() / 2.0,
            cherry_type: a.cherry_type().next_cherry(),
        });
        assert_ne!(a, b);

        if let Some(message) = b
            .params()
            .message_for("apple-count", a.apple_count().into())
        {
            b.params.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b
            .params()
            .message_for("banana-quality", a.banana_quality().into())
        {
            b.params.update(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b
            .params()
            .message_for("cherry-type", a.cherry_type().into())
        {
            b.params.update(message);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn control_ergonomics() {
        let a = Stuff::new(StuffParams::make_fake());

        assert_eq!(a.params().name_by_index(2), Some("cherry-type"));
        assert_eq!(a.params().count(), 3);
        assert_eq!(a.params().name_by_index(a.params().count()), None);

        let a = MiscParams::make_fake();

        assert_eq!(a.name_by_index(0), Some("cat-count"));
        assert_eq!(a.count(), 2);
        assert_eq!(a.name_by_index(a.count()), None);
    }

    #[test]
    fn core_struct_gets_notifications() {
        let mut stuff = Stuff::new(StuffParams::make_fake());

        assert!(stuff.computed_data);
        stuff.set_apple_count(stuff.params.apple_count() + 10);
        assert!(!stuff.computed_data);
    }

    #[test]
    fn build_views() {
        let entities = vec![
            EntityParams::Stuff(Box::new(StuffParams::make_fake())),
            EntityParams::Misc(Box::new(MiscParams::make_fake())),
            EntityParams::Misc(Box::new(MiscParams::make_fake())),
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
            let controllable = entity.as_controllable_ref().unwrap();
            for index in 0..controllable.count() {
                if let Some(point_name) = controllable.name_by_index(index) {
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
            EntityParams::Stuff(Box::new(StuffParams::make_fake())),
            EntityParams::Misc(Box::new(MiscParams::make_fake())),
            EntityParams::Misc(Box::new(MiscParams::make_fake())),
        ];

        // Connect two things
        // send message: connect(source, dest, index)
        // send message: disconnect(source, dest, index)

        // Handle an incoming message
        let message = StuffParamsMessage::AppleCount(45);
        let wrapped_message = AppMessages::Wrapper(1, EntityMessage::StuffParams(message));

        let AppMessages::Wrapper(uid, message) = wrapped_message;
        let entity = &mut entities[uid];
        match message {
            EntityMessage::StuffParams(message) => {
                if let EntityParams::Stuff(entity) = entity {
                    entity.update(message);
                }
            }
            EntityMessage::MiscParams(message) => {
                if let EntityParams::Misc(entity) = entity {
                    entity.update(message);
                }
            }
        }
    }

    #[test]
    fn engine_usage() {
        let a = Stuff::new(StuffParams::make_fake());

        //        let message = a.params.
    }
}
