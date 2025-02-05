// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "_secure")]
mod auth_context;
#[cfg(feature = "_secure")]
mod credentials;

use grpcio_sys::{grpc_channel_credentials, grpc_server_credentials};

#[cfg(feature = "_secure")]
pub use self::auth_context::*;
#[cfg(feature = "_secure")]
pub use self::credentials::{
    CertificateRequestType, ChannelCredentialsBuilder, ServerCredentialsBuilder,
    ServerCredentialsFetcher,
};

/// Client-side SSL credentials.
///
/// Use [`ChannelCredentialsBuilder`] to build a [`ChannelCredentials`].  This is the preferred
/// method for creating credentials.  The `from_raw` constructor is provided for advanced use
/// cases and requires careful handling of the underlying pointer.
pub struct ChannelCredentials {
    creds: *mut grpc_channel_credentials,
}

impl ChannelCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut grpc_channel_credentials {
        self.creds
    }

    /// Creates an insecure channel credentials object.
    pub fn insecure() -> ChannelCredentials {
        unsafe {
            let creds = grpcio_sys::grpc_insecure_credentials_create();
            ChannelCredentials { creds }
        }
    }

    /// Creates a `ChannelCredentials` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The `creds` pointer must be a valid pointer to a `grpc_channel_credentials` object
    /// created by a grpc_sys function.  The pointer must not be null.
    /// ChannelCredentials takes ownership of the pointer.
    pub unsafe fn from_raw(creds: *mut grpc_channel_credentials) -> ChannelCredentials {
        ChannelCredentials { creds }
    }
}

impl Drop for ChannelCredentials {
    fn drop(&mut self) {
        unsafe { grpcio_sys::grpc_channel_credentials_release(self.creds) }
    }
}

/// Server-side SSL credentials.
///
/// Use [`ServerCredentialsBuilder`] to build a [`ServerCredentials`]. This is the preferred
/// method for creating credentials. The `from_raw` constructor is provided for advanced use
/// cases and requires careful handling of the underlying pointer.
pub struct ServerCredentials {
    creds: *mut grpc_server_credentials,
    // Double allocation to get around C call.
    #[cfg(feature = "_secure")]
    _fetcher: Option<Box<Box<dyn crate::ServerCredentialsFetcher + Send + Sync>>>,
}

unsafe impl Send for ServerCredentials {}

impl ServerCredentials {
    /// Creates an insecure server credentials object.
    pub fn insecure() -> ServerCredentials {
        unsafe {
            let creds = grpcio_sys::grpc_insecure_server_credentials_create();
            ServerCredentials::from_raw(creds)
        }
    }

    /// Creates a `ServerCredentials` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The `creds` pointer must be a valid pointer to a `grpc_server_credentials` object
    /// created by a grpc_sys function. The pointer must not be null. The caller takes
    /// ownership of the pointer and is responsible for ensuring it is not released
    /// multiple times.
    pub unsafe fn from_raw(creds: *mut grpc_server_credentials) -> ServerCredentials {
        ServerCredentials {
            creds,
            #[cfg(feature = "_secure")]
            _fetcher: None,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut grpc_server_credentials {
        self.creds
    }
}

impl Drop for ServerCredentials {
    fn drop(&mut self) {
        unsafe {
            grpcio_sys::grpc_server_credentials_release(self.creds);
        }
    }
}
