use super::*;

pub use linux5_7::{
    UFFDIO_API, UFFDIO_COPY, UFFDIO_COPY_MODE_DONTWAKE, UFFDIO_COPY_MODE_WP, UFFDIO_REGISTER,
    UFFDIO_REGISTER_MODE_MISSING, UFFDIO_REGISTER_MODE_WP, UFFDIO_UNREGISTER, UFFDIO_WAKE,
    UFFDIO_WRITEPROTECT, UFFDIO_WRITEPROTECT_MODE_DONTWAKE, UFFDIO_WRITEPROTECT_MODE_WP,
    UFFDIO_ZEROPAGE, UFFDIO_ZEROPAGE_MODE_DONTWAKE, UFFD_API, UFFD_API_FEATURES, UFFD_API_IOCTLS,
    UFFD_API_RANGE_IOCTLS_BASIC,
};

pub const UFFD_API_RANGE_IOCTLS: u64 = linux5_7::UFFD_API_RANGE_IOCTLS | 1 << _UFFDIO_CONTINUE;

pub const UFFDIO_REGISTER_MODE_MINOR: u64 = 1 << 2;

pub const UFFDIO_CONTINUE_MODE_DONTWAKE: u64 = 1 << 0;

pub const UFFDIO_CONTINUE: u32 = 0xc020aa07;

#[cfg(test)]
mod const_tests {
    use super::*;

    extern "C" {
        static _const_UFFDIO_REGISTER_MODE_MINOR: u64;
        static _const_UFFDIO_CONTINUE_MODE_DONTWAKE: u64;
        static _const_UFFDIO_CONTINUE: u32;
    }

    #[test]
    fn consts_correct() {
        unsafe {
            assert_eq!(
                UFFDIO_REGISTER_MODE_MINOR, _const_UFFDIO_REGISTER_MODE_MINOR,
                "UFFDIO_REGISTER_MODE_MINOR"
            );
            assert_eq!(
                UFFDIO_CONTINUE_MODE_DONTWAKE, _const_UFFDIO_CONTINUE_MODE_DONTWAKE,
                "UFFDIO_CONTINUE_MODE_DONTWAKE"
            );
            assert_eq!(UFFDIO_CONTINUE, _const_UFFDIO_CONTINUE, "UFFDIO_CONTINUE");
        }
    }
}
