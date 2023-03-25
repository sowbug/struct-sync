#[allow(dead_code)]
#[allow(unused_variables)]
mod models;

// use `cargo expand` to see what the macro is generating.

trait Controller {}
trait Controllable {
    fn count(&self) -> usize;
    fn name_by_index(&self, index: usize) -> Option<&'static str>;
}

#[macro_export]
macro_rules! register_impl {
    ($trait_:ident for $ty:ty, true) => {
        impl<'a> MaybeImplements<'a, dyn $trait_> for $ty {
            fn as_trait_ref(&self) -> Option<&(dyn $trait_ + 'static)> {
                Some(self)
            }
            fn as_trait_mut(&mut self) -> Option<&mut (dyn $trait_ + 'static)> {
                Some(self)
            }
        }
    };
    ($trait_:ident for $ty:ty, false) => {
        impl<'a> MaybeImplements<'a, dyn $trait_> for $ty {
            fn as_trait_ref(&self) -> Option<&(dyn $trait_ + 'static)> {
                None
            }
            fn as_trait_mut(&mut self) -> Option<&mut (dyn $trait_ + 'static)> {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! all_entities {
    () => {};
    ($($entity:ident; $params:tt; $message:ident; $is_controller:tt; $is_controllable:tt ,)*) => {
        pub(crate) enum EntityMessage {
            $( $params($message) ),*
        }
        pub(crate) enum EntityParams {
            $( $entity(Box<$params>) ),*
        }
        impl EntityParams {
            fn is_controller(&self) -> bool {
                self.as_controller_ref().is_some()
            }
            fn is_controllable(&self) -> bool {
                self.as_controllable_ref().is_some()
            }
            fn as_controller_ref(&self) -> Option<&(dyn Controller + 'static)> {
                match self {
                    EntityParams::Stuff(e) => e.as_trait_ref(),
                    EntityParams::Misc(e) => e.as_trait_ref(),
                }
            }
            fn as_controller_mut(&mut self) -> Option<&mut (dyn Controller + 'static)> {
                match self {
                    EntityParams::Stuff(e) => e.as_trait_mut(),
                    EntityParams::Misc(e) => e.as_trait_mut(),
                }
            }
            fn as_controllable_ref(&self) -> Option<&(dyn Controllable + 'static)> {
                match self {
                    EntityParams::Stuff(e) => e.as_trait_ref(),
                    EntityParams::Misc(e) => e.as_trait_ref(),
                }
            }
            fn as_controllable_mut(&mut self) -> Option<&mut (dyn Controllable + 'static)> {
                match self {
                    EntityParams::Stuff(e) => e.as_trait_mut(),
                    EntityParams::Misc(e) => e.as_trait_mut(),
                }
            }
        }
        trait MaybeImplements<'a, Trait: ?Sized> {
            fn as_trait_ref(&'a self) -> Option<&'a Trait>;
            fn as_trait_mut(&mut self) -> Option<&mut Trait>;
        }
        $( register_impl!(Controller for $params, $is_controller); )*
        $( register_impl!(Controllable for $params, $is_controllable); )*
    };
}

// The view side needs getters/setters
//   introspection (count of controllable points, names)
// Propagation to engine.
// It will handle messages from the engine side.
// It will use Iced messages in GUI widgets.
//
// The engine side needs getters/setters
// propagation to view
// It will handle messages from the view side
// It uses its control apparatus to automate controls
