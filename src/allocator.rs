use core::ptr;
use core::usize;

#[allow(dead_code)]
mod constants {
    pub const PROT_READ:      usize = 0x1;     // Page can be read
    pub const PROT_WRITE:     usize = 0x2;     // Page can be written
    pub const PROT_EXEC:      usize = 0x4;     // Page can be executed
    pub const PROT_SEM:       usize = 0x8;     // Page may be used for atomic ops
    pub const PROT_NONE:      usize = 0x0;     // Page can not be accessed
    pub const PROT_GROWSDOWN: usize = 0x01000000;  // mprotect flag: extend change to start of growsdown vma
    pub const PROT_GROWSUP:   usize = 0x02000000;  // mprotect flag: extend change to end of growsup vma

    pub const MAP_SHARED:        usize = 0x01;        // Share changes
    pub const MAP_PRIVATE:       usize = 0x02;        // Changes are private
    pub const MAP_TYPE:          usize = 0x0f;        // Mask for type of mapping
    pub const MAP_FIXED:         usize = 0x10;        // Interpret addr exactly
    pub const MAP_ANONYMOUS:     usize = 0x20;        // Don't use a file
    pub const MAP_UNINITIALIZED: usize = 0x4000000;   // For anonymous mmap, memory could be uninitialized
}

use self::constants::*;

fn mmap( address: *mut u8, size: usize, protection: usize, flags: usize, fd: usize, offset: usize ) -> Result< *mut u8, () > {
    let value = unsafe {
        syscall!( MMAP, address, size, protection, flags, fd, offset )
    };

    if value > usize::MAX - 4096 {
        Err(())
    } else {
        Ok( value as *mut u8 )
    }
}

fn mmap_anonymous( size: usize ) -> Result< *mut u8, () > {
    mmap( ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, usize::MAX, 0 )
}

fn munmap( address: *mut u8, size: usize ) {
    unsafe {
        syscall!( MUNMAP, address, size );
    }
}

pub fn allocate( size: usize, _alignment: usize ) -> *mut u8 {
    if let Ok( pointer ) = mmap_anonymous( size ) {
        pointer
    } else {
        ptr::null_mut()
    }
}

pub fn reallocate( pointer: *mut u8, old_size: usize, size: usize, alignment: usize ) -> *mut u8 {
    if pointer == ptr::null_mut() {
        return ptr::null_mut();
    }

    if size == 0 {
        deallocate( pointer, size, alignment );
        return ptr::null_mut();
    }

    if size < old_size {
        return pointer;
    }

    let new_pointer = allocate( size, alignment );
    if new_pointer == ptr::null_mut() {
        return ptr::null_mut();
    }

    unsafe {
        ptr::copy_nonoverlapping( pointer as *const u8, new_pointer, size );
    }
    deallocate( pointer, size, alignment );

    return new_pointer;
}

pub fn deallocate( pointer: *mut u8, size: usize, _alignment: usize ) {
    if pointer == ptr::null_mut() {
        return;
    }

    munmap( pointer, size );
}

#[test]
fn test_mmap() {
    assert!( mmap_anonymous( 4096 ).is_ok() );
    assert!( mmap_anonymous( 5000 ).is_ok() );
}

#[test]
fn test_failed_mmap() {
    use core::usize;
    assert_eq!( mmap_anonymous( usize::MAX ), Err(()) );
}

#[test]
fn test_allocation() {
    let mut pointer = allocate( 100, 0 );
    assert_ne!( pointer, ptr::null_mut() );
    unsafe {
        *pointer = 0;
    }
}
