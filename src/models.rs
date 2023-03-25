use crate::{all_entities, Controllable, Controller};
use groove_core::control::F32ControlValue;
use groove_macros::Control;
use std::str::FromStr;
use struct_sync_macros::Synchronization;
use strum::EnumCount;
use strum_macros::{
    Display, EnumCount as EnumCountMacro, EnumIter, EnumString, FromRepr, IntoStaticStr,
};

enum AppMessages {
    Wrapper(usize, EntityMessage),
}

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

#[derive(Clone, Control, Debug, PartialEq, Synchronization)]
pub struct StuffParams {
    #[sync]
    #[controllable]
    apple_count: usize,
    #[sync]
    #[controllable]
    banana_quality: f32,
    #[sync]
    #[controllable]
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

    pub fn set_control_apple_count(&mut self, v: F32ControlValue) {
        self.set_and_propagate_apple_count((v.0 * 10.0) as usize);
    }
    pub fn set_control_banana_quality(&mut self, v: F32ControlValue) {
        self.set_and_propagate_banana_quality(v.0);
    }
    pub fn set_control_cherry_type(&mut self, v: F32ControlValue) {
        self.set_and_propagate_cherry_type(CherryType::Black);
    }
}

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
        self.params.set_and_propagate_apple_count(count);
    }

    fn banana_quality(&self) -> f32 {
        self.params.banana_quality()
    }

    fn set_banana_quality(&mut self, banana_quality: f32) {
        self.clear_precomputed();
        self.params.set_and_propagate_banana_quality(banana_quality);
    }

    fn cherry_type(&self) -> CherryType {
        self.params.cherry_type()
    }

    fn set_cherry_type(&mut self, cherry_type: CherryType) {
        self.clear_precomputed();
        self.params.set_and_propagate_cherry_type(cherry_type);
    }
}

#[derive(Clone, Control, Debug, PartialEq, Synchronization)]
pub struct MiscParams {
    #[sync]
    #[controllable]
    cat_count: usize,
    #[sync]
    #[controllable]
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
        self.set_and_propagate_cat_count((v.0 * 10.0) as usize);
    }
    pub fn set_control_dog_count(&mut self, v: F32ControlValue) {
        self.set_and_propagate_dog_count((v.0 * 10.0) as usize);
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
impl Controllable for MiscParams {
    fn count(&self) -> usize {
        2
    }

    fn name_by_index(&self, index: usize) -> &'static str {
        match index {
            0 => "cat_count",
            1 => "dog_count",
            _ => panic!(),
        }
    }
}

struct Misc {
    params: MiscParams,
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
    use groove_core::traits::Controllable;

    #[test]
    fn update_full() {
        let a = StuffParams::make_fake();
        let mut b = StuffParams::make_fake();
        assert_ne!(a, b);
        b.handle_message(StuffParamsMessage::StuffParams(a.clone()));
        assert_eq!(a, b);
    }

    #[test]
    fn update_incrementally() {
        let mut a = StuffParams::make_fake();
        let mut b = StuffParams {
            apple_count: a.apple_count + 1,
            banana_quality: a.banana_quality / 2.0,
            cherry_type: a.cherry_type().next_cherry(),
        };
        assert_ne!(a, b);
        if let Some(message) = a.set_and_propagate_apple_count(a.apple_count() + 1) {
            b.handle_message(message);
        }
        assert_ne!(a, b);
        if let Some(message) = b.set_and_propagate_banana_quality(b.banana_quality() / 3.0) {
            a.handle_message(message);
        }
        assert_ne!(a, b);
        if let Some(message) = a.set_and_propagate_cherry_type(a.cherry_type().next_cherry()) {
            b.handle_message(message);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn address_params_by_index() {
        let stuff = Stuff::new(StuffParams::make_fake());

        assert_eq!(stuff.params.control_name_for_index(2), "cherry-type");
        assert_eq!(stuff.params.control_index_count(), 3);
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
                let point_name = controllable.name_by_index(index);
                eprintln!("adding control point {}", point_name);
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
                    entity.handle_message(message);
                }
            }
            EntityMessage::MiscParams(message) => {
                if let EntityParams::Misc(entity) = entity {
                    entity.handle_message(message);
                }
            }
        }
    }

    #[test]
    fn engine_usage() {
        let a = Stuff::new(StuffParams::make_fake());
    }
}
