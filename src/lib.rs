// src/lib.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! General types and traits for siminau servers
//!
//! A siminau server uses the RPC messages defined in the `siminau-rpc` crate to
//! create a protocol for communicating with other siminau servers and with
//! siminau clients. All RPC messages rely on a filesystem abstraction, and it
//! is this abstraction that protocols rely on in order for servers and clients
//! to communicate with each other.

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs

#[macro_use] extern crate failure;
extern crate futures;
extern crate tokio_core;

// Local externs

#[macro_use] extern crate siminau_rpc;

// Test externs

#[cfg(test)]
#[macro_use] extern crate proptest;


// ===========================================================================
//
// ===========================================================================
