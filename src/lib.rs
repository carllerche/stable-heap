use std::mem;

/// An arbitrary non-null address to represent zero-size allocations.
///
/// This preserves the non-null invariant for types like `Box<T>`. The address
/// may overlap with non-zero-size memory allocations.
pub const EMPTY: *mut () = 0x1 as *mut ();

/// Return a pointer to `size` bytes of memory aligned to `align`.
///
/// On failure, return a null pointer.
///
/// Behavior is undefined if the requested size is 0 or the alignment is not a
/// power of 2. The alignment must be no larger than the largest supported page
/// size on the platform.
#[inline]
pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
    assert!(size & align == 0, "invalid allocate arguments; size={}; align={}", size, align);

    match align {
        1 => do_allocate::<u8>(size),
        2 => do_allocate::<u16>(size >> 1),
        4 => do_allocate::<u32>(size >> 2),
        8 => do_allocate::<u64>(size >> 3),
        _ => panic!("unsupported alignment {}", align),
    }
}

unsafe fn do_allocate<T>(capacity: usize) -> *mut u8 {
    let vec = Vec::<T>::with_capacity(capacity);
    let ptr = vec.as_ptr();

    mem::forget(vec);

    ptr as *mut u8
}

/// Deallocates the memory referenced by `ptr`.
///
/// The `ptr` parameter must not be null.
///
/// The `old_size` and `align` parameters are the parameters that were used to
/// create the allocation referenced by `ptr`. The `old_size` parameter may be
/// any value in range_inclusive(requested_size, usable_size).
#[inline]
pub unsafe fn deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    match align {
        1 => do_deallocate::<u8>(ptr, old_size),
        2 => do_deallocate::<u16>(ptr, old_size >> 1),
        4 => do_deallocate::<u32>(ptr, old_size >> 2),
        8 => do_deallocate::<u64>(ptr, old_size >> 3),
        _ => panic!("unsupported alignment {}", align),
    }
}

unsafe fn do_deallocate<T>(ptr: *mut u8, capacity: usize) {
    let _ = Vec::from_raw_parts(ptr as *mut T, 0, capacity);
}

#[cfg(test)]
mod test {
    use std::mem;

    #[test]
    fn test_align() {
        assert_eq!(1, mem::align_of::<u8>());
        assert_eq!(2, mem::align_of::<u16>());
        assert_eq!(4, mem::align_of::<u32>());
        assert_eq!(8, mem::align_of::<u64>());
    }

    #[test]
    fn test_allocate_deallocate() {
        unsafe {
            ::deallocate(::allocate(2048, 4), 2048, 4);
        }
    }

    #[test]
    fn test_empty_constant() {
        let mut v = Vec::<()>::with_capacity(0);
        assert_eq!(::EMPTY, v.as_mut_ptr());
    }
}
