use std::any::{Any, TypeId};

use crate::components::{
    component::Component, component_column::AnyColumn, component_column::Column,
};

pub trait Bundle {
    fn get_signature() -> Vec<TypeId>;
    fn get_components(self, func: &mut impl FnMut(TypeId, Box<dyn Any>));
    fn register_components(func: &mut impl FnMut(TypeId, fn() -> Box<dyn AnyColumn>));
}

macro_rules! impl_bundle {
    ($($T:ident => $idx:tt),*) => {
        impl<$($T: Component + 'static),*> Bundle for ($($T,)*) {

            fn get_signature() -> Vec<TypeId> {
                let mut signature = Vec::new();
                $(signature.push(TypeId::of::<$T>());)*
                signature.sort();
                return signature
            }

            fn get_components(
                self,
                func: &mut impl FnMut(TypeId, Box<dyn Any>)
            ) {
                $(
                    func(
                        TypeId::of::<$T>(),
                        Box::new(self.$idx)
                    );
                )*
            }

            fn register_components(
                func: &mut impl FnMut(TypeId, fn() -> Box<dyn AnyColumn>)
            ) {
                $(
                    func(TypeId::of::<$T>(),Column::<$T>::create_any);
                )*
            }

        }
    };
}

impl_bundle!(A => 0);
impl_bundle!(A => 0, B => 1);
impl_bundle!(A => 0, B => 1, C => 2);
impl_bundle!(A => 0, B => 1, C => 2, D => 3);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9, K => 10);
impl_bundle!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9, K => 10, L => 11);
