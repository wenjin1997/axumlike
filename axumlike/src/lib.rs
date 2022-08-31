pub use async_trait::async_trait;
pub use http;
pub use hyper::Server;
pub use tower_http::add_extension::{AddExtension, AddExtensionLayer};

#[macro_use]
mod macros;

mod router;
mod body;
mod error;
mod util;
pub mod handler;
pub mod response;
pub mod extract;
pub mod service;

pub use router::Router;

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;