// src\codec.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports
use std::io;

// Third-party imports
use bytes::BytesMut;
use rmpv::Value;
use tokio_io::codec::{Decoder, Encoder};

// Local imports
use siminau_rpc::core::{AsBytes, FromBytes, FromMessage, Message};


// ===========================================================================
// Codec
// ===========================================================================


pub struct MsgPackCodec;


// TODO: Use Message for Item
impl Decoder for MsgPackCodec {
    type Item = Value;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Value>>
    {
        match Message::from_bytes(buf) {
            Ok(Some(m)) => Ok(Some(m.into())),
            Ok(None) => Ok(None),
            Err(e) => Err(io::Error::from(e))
        }
    }
}


// TODO: Use Message for Item
impl Encoder for MsgPackCodec {
    type Item = Value;
    type Error = io::Error;

    fn encode(&mut self, val: Value, buf: &mut BytesMut) -> io::Result<()>
    {
        let msg = Message::from_msg(val).unwrap();
        let b = msg.as_bytes();
        buf.extend_from_slice(&b);
        Ok(())
    }
}


// ===========================================================================
//
// ===========================================================================
