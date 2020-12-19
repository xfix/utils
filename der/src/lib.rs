//! Pure Rust embedded-friendly implementation of the Distinguished Encoding Rules (DER)
//! for Abstract Syntax Notation One (ASN.1) as described in ITU [X.690].
//!
//! # About
//!
//! This crate provides a low-level implementation of a subset of ASN.1 DER
//! necessary for decoding/encoding various cryptography-related formats.
//!
//! It avoids any heap usage, and is presently specialized for documents which
//! are smaller than 64kB.
//!
//! # Minimum Supported Rust Version
//!
//! This crate requires **Rust 1.46** at a minimum.
//!
//! [X.690]: https://www.itu.int/rec/T-REC-X.690/

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png",
    html_root_url = "https://docs.rs/der/0.0.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod any;
mod bit_string;
mod decoder;
mod encoder;
mod error;
mod header;
mod integer;
mod length;
mod null;
mod octet_string;
mod optional;
mod sequence;
mod tag;
mod traits;

#[cfg(feature = "oid")]
mod oid;

pub use crate::{
    any::Any,
    bit_string::BitString,
    decoder::Decoder,
    encoder::Encoder,
    error::{Error, Result},
    integer::Integer,
    length::Length,
    null::Null,
    octet_string::OctetString,
    sequence::Sequence,
    tag::Tag,
    traits::{Decodable, Encodable, Message, Tagged},
};

pub(crate) use crate::header::Header;

#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
pub use const_oid::ObjectIdentifier;