use super::*;

pub use linux4_14::{
    UFFDIO_API, UFFDIO_COPY, UFFDIO_COPY_MODE_DONTWAKE, UFFDIO_COPY_MODE_WP, UFFDIO_REGISTER,
    UFFDIO_REGISTER_MODE_MISSING, UFFDIO_REGISTER_MODE_WP, UFFDIO_UNREGISTER, UFFDIO_WAKE,
    UFFDIO_ZEROPAGE, UFFDIO_ZEROPAGE_MODE_DONTWAKE, UFFD_API, UFFD_API_FEATURES, UFFD_API_IOCTLS,
    UFFD_API_RANGE_IOCTLS_BASIC,
};

// The following are preprocessor constants that bindgen can't figure out, so we enter them manually
// from <linux/userfaultfd.h>, and have tests to make sure they're accurate.

pub const UFFD_API_RANGE_IOCTLS: u64 = linux4_14::UFFD_API_RANGE_IOCTLS | 1 << _UFFDIO_WRITEPROTECT;

pub const UFFDIO_WRITEPROTECT_MODE_WP: u64 = 1 << 0;
pub const UFFDIO_WRITEPROTECT_MODE_DONTWAKE: u64 = 1 << 1;

pub const UFFDIO_WRITEPROTECT: u32 = 0xc018aa06;

#[cfg(test)]
mod const_tests {
    use super::*;

    extern "C" {
        static _const_UFFDIO_WRITEPROTECT_MODE_WP: u64;
        static _const_UFFDIO_WRITEPROTECT_MODE_DONTWAKE: u64;
        static _const_UFFDIO_WRITEPROTECT: u32;
    }

    #[test]
    fn consts_correct() {
        unsafe {
            assert_eq!(
                UFFDIO_WRITEPROTECT_MODE_WP, _const_UFFDIO_WRITEPROTECT_MODE_WP,
                "UFFDIO_WRITEPROTECT_MODE_WP"
            );
            assert_eq!(
                UFFDIO_WRITEPROTECT_MODE_DONTWAKE, _const_UFFDIO_WRITEPROTECT_MODE_DONTWAKE,
                "UFFDIO_WRITEPROTECT_MODE_DONTWAKE"
            );
            assert_eq!(
                UFFDIO_WRITEPROTECT, _const_UFFDIO_WRITEPROTECT,
                "UFFDIO_WRITEPROTECT"
            );
        }
    }
}
