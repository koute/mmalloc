use allocator::*;

#[linkage = "external"]
#[no_mangle]
#[inline]
pub extern fn __rust_allocate( size: usize, alignment: usize ) -> *mut u8 {
    allocate( size, alignment )
}

#[linkage = "external"]
#[no_mangle]
#[inline]
pub unsafe extern fn __rust_deallocate( ptr: *mut u8, size: usize, alignment: usize ) {
    deallocate( ptr, size, alignment )
}

#[linkage = "external"]
#[no_mangle]
#[inline]
pub unsafe extern fn __rust_reallocate( ptr: *mut u8, old_size: usize, size: usize, align: usize ) -> *mut u8 {
    reallocate( ptr, old_size, size, align )
}

#[linkage = "external"]
#[no_mangle]
#[inline]
pub unsafe extern fn __rust_reallocate_inplace( _ptr: *mut u8, old_size: usize, size: usize, _align: usize ) -> usize {
    if size < old_size {
        size
    } else {
        old_size
    }
}

#[linkage = "external"]
#[no_mangle]
#[inline]
pub extern fn __rust_usable_size( size: usize, _align: usize ) -> usize {
    size
}
