/*crate::six! {
import core::mem::forget as _forget;
pub import core::mem::size_of;
pub import core::mem::size_of_val;

@inline
pub unsafe fn forget<T>(val: T) -> unit {
    _forget(val);
}

@inline
pub fn drop<T>(_x: T) -> unit {}
}*/