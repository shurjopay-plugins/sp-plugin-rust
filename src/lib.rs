// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![cfg_attr(docsrs, feature(doc_cfg))]
// #![cfg_attr(test, deny(warnings))]

//! # Shurjopay
//!
//! The `Shurjopay` crate provides a convenient way to integrate 
//! [`Shurjopay`][shurjopay] payment gateway.
//!
//! It handles many of the things that most people need to do manually
//! 
//! - Handles http request and request errors
//! - JSON serialization and deserialization
//! - Authentication during checkout and verification of payments
//!
//! 
//! [shurjopay]: crate::Shurjopay

pub mod shurjopay;
pub mod shurjopay_client;



