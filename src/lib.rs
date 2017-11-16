//! Simple _Merkle Tree_ implementation.
//!
//! Merkle Tree (MT) implemented as a full binary tree allocated as a vec
//! of statically sized hashes to give hashes more locality. MT specialized
//! to the extent of hashing algorithm and hash item, compatible to the
//! std::hash::Hasher and supports custom hash algorithms.
//! Implementation does not depend on any external crypto libraries,
//! and tries to be as performant, as possible.
//!
//! This tree implementation uses encoding scheme as in _Certificate Transparency_
//! RFC 6962 (https://tools.ietf.org/html/rfc6962):
//!
//! ```
//! MTH({d(0)}) = ALG(0x00 || d(0)).
//! For n > 1, let k be the largest power of two smaller than n (i.e.,
//! k < n <= 2k).  The Merkle Tree Hash of an n-element list D[n] is then
//! defined recursively as
//! MTH(D[n]) = ALG(0x01 || MTH(D[0:k]) || MTH(D[k:n])),
//! ```
//!
//! Link: https://en.wikipedia.org/wiki/Merkle_tree
//!
//! # Implementation choices
//!
//! Main idea was that the whole code must obtain specialization at compile
//! time, hashes must be fixed size arrays known at compile time, hash algorithm
//! must be interface (lib should not dep on crypto libs) and lib must somehow
//! mimic std Rust api.
//!
//! Standard way in Rust is to hash objects with a std::hash::Hasher, and mainly
//! that is the reason behind the choice of the abstractions:
//!
//! ```
//!     Object : Hash<H> -> Hasher + Algorithm <- Merkle Tree
//! ```
//!
//! Custom [`merkle::hash::Hash`] trait provided to allow implementations differ
//! from [`std::collection`] related hashes, different implementations for
//! different hashing algorithms / schemas.
//!
//! Big issue was missing [`const_generics`] feature implemented in the compiler,
//! and I didn't want to use unsafe [`FixedSizeArray<T>`], so there is just 1
//! additional trait to bypass this limitation until dependent types will be
//! implemented.
//!
//! [`const_generics`]: generic_const
//! [`FixedSizeArray<T>`]: https://github.com/rust-lang/rust/issues/27778
//!
//! I would like to remove [`AsBytes`] trait, but [`Unsize`] is unsafe, so its a
//! decent replacement.
//!
//! [`Unsize`]: https://doc.rust-lang.org/std/marker/trait.Unsize.html
//!
//! # Interface
//! - eval tree (items) -> tree
//! - get root -> hash
//! - get proof -> proof
//! - validate proof (tree, proof) -> result
//!
//! # Examples
//!
//! TODO

#![deny(
    missing_docs, unused_qualifications,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces
)]

/// Hash infrastructure for items in Merkle Tree.
pub mod hash;

/// Merkle Tree abstractions, implementation and algorithms.
pub mod merkle;
