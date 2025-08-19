use super::*;

pub use linux4_11::{
    UFFDIO_API, UFFDIO_COPY, UFFDIO_COPY_MODE_DONTWAKE, UFFDIO_COPY_MODE_WP, UFFDIO_REGISTER,
    UFFDIO_REGISTER_MODE_MISSING, UFFDIO_REGISTER_MODE_WP, UFFDIO_UNREGISTER, UFFDIO_WAKE,
    UFFDIO_ZEROPAGE, UFFDIO_ZEROPAGE_MODE_DONTWAKE, UFFD_API, UFFD_API_IOCTLS,
    UFFD_API_RANGE_IOCTLS,
};

// The following are preprocessor constants that bindgen can't figure out, so we enter them manually
// from <linux/userfaultfd.h>, and have tests to make sure they're accurate.

pub const UFFD_API_FEATURES: u64 =
    linux4_11::UFFD_API_FEATURES | UFFD_FEATURE_SIGBUS | UFFD_FEATURE_THREAD_ID;
pub const UFFD_API_RANGE_IOCTLS_BASIC: u64 =
    linux4_11::UFFD_API_RANGE_IOCTLS_BASIC | 1 << _UFFDIO_WAKE | 1 << _UFFDIO_COPY;
