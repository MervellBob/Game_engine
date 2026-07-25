use std::any::type_name;
use std::collections::HashMap;
use std::{any::TypeId, ptr::NonNull};

use crate::components::component::Component;
use crate::queries::query::BorrowKind;

pub trait QueryItem {
    type Component: Component;
    const MUTABLE: BorrowKind;
    type Ref<'w>;
    unsafe fn fetch<'w>(ptr: NonNull<()>, row: usize) -> Self::Ref<'w>;
}

impl<T: Component> QueryItem for &T {
    type Component = T;
    const MUTABLE: BorrowKind = BorrowKind::Shared;
    type Ref<'w> = &'w T;

    unsafe fn fetch<'w>(ptr: NonNull<()>, row: usize) -> Self::Ref<'w> {
        unsafe { &*ptr.cast::<T>().as_ptr().add(row) }
    }
}

impl<T: Component> QueryItem for &mut T {
    type Component = T;
    const MUTABLE: BorrowKind = BorrowKind::Exclusive;
    type Ref<'w> = &'w mut T;

    unsafe fn fetch<'w>(ptr: NonNull<()>, row: usize) -> Self::Ref<'w> {
        unsafe { &mut *ptr.cast::<T>().as_ptr().add(row) }
    }
}

pub trait QueryItems {
    type Item<'w>;
    fn checked_params() -> Vec<QueryItemInfo>;
    unsafe fn fetch<'w>(columns: &[NonNull<()>], row: usize) -> Self::Item<'w>;
}

macro_rules! impl_query_item {
    ($($T:ident => $idx:tt),*) => {
        impl<$($T: QueryItem),*> QueryItems for ($($T,)*) {

            type Item<'w> = ($(<$T as QueryItem>::Ref<'w>,)*);

            fn checked_params() -> Vec<QueryItemInfo> {
                let mut params = Vec::new();
                let mut seen = HashMap::<TypeId, BorrowKind>::new();
                $(
                    let current_item = QueryItemInfo {
                        type_id: TypeId::of::<<$T as QueryItem>::Component>(),
                        mutable: $T::MUTABLE,
                        type_name: type_name::<<$T as QueryItem>::Component>(),
                    };
                    // checks for multiple use of exclusive (mutable) components.
                    match seen.get(&current_item.type_id()) {
                        None => {
                            seen.insert(current_item.type_id(),current_item.mutable());
                        }
                        Some (previous) => {
                            if !(*previous == BorrowKind::Shared && current_item.mutable() == BorrowKind::Shared) {
                                panic!("Component {} is used mutably, thus should be exclusive", current_item.type_name() );
                                }
                        }
                    }
                    params.push(current_item);
                )*
                return params
            }

            unsafe fn fetch<'w>(
                columns: &[NonNull<()>],
                row: usize,
            ) -> Self::Item<'w> {
                (
                    $(
                        unsafe { <$T as QueryItem>::fetch(columns[$idx], row) },
                    )*
                )
            }
        }
    };
}

pub struct QueryItemInfo {
    type_id: TypeId,
    mutable: BorrowKind,
    type_name: &'static str,
}

impl QueryItemInfo {
    pub(crate) fn type_id(&self) -> TypeId {
        self.type_id
    }
    pub(crate) fn type_name(&self) -> &'static str {
        self.type_name
    }
    pub(crate) fn mutable(&self) -> BorrowKind {
        self.mutable
    }
}

impl_query_item!(A => 0);
impl_query_item!(A => 0, B => 1);
impl_query_item!(A => 0, B => 1, C => 2);
impl_query_item!(A => 0, B => 1, C => 2, D => 3);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9, K => 10);
impl_query_item!(A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, I => 8, J => 9, K => 10, L => 11);
