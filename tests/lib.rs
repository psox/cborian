// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(iter_arith, plugin)]
#![plugin(quickcheck_macros)]

extern crate cbor;
extern crate quickcheck;
extern crate rustc_serialize;

use cbor::encoder::{Encoder, EncodeResult};
use cbor::decoder::{Config, Decoder};
use cbor::values::{Bytes, Text, Value};
use std::io::Cursor;

#[quickcheck]
fn prop_identity_u8(x: u8) -> bool {
    identity(2, |mut e| e.u8(x), |mut d| d.u8().unwrap() == x)
}

#[quickcheck]
fn prop_identity_u16(x: u16) -> bool {
    identity(3, |mut e| e.u16(x), |mut d| d.u16().unwrap() == x)
}

#[quickcheck]
fn prop_identity_u32(x: u32) -> bool {
    identity(5, |mut e| e.u32(x), |mut d| d.u32().unwrap() == x)
}

#[quickcheck]
fn prop_identity_u64(x: u64) -> bool {
    identity(9, |mut e| e.u64(x), |mut d| d.u64().unwrap() == x)
}

#[quickcheck]
fn prop_identity_i8(x: i8) -> bool {
    identity(2, |mut e| e.i8(x), |mut d| d.i8().unwrap() == x)
}

#[quickcheck]
fn prop_identity_i16(x: i16) -> bool {
    identity(3, |mut e| e.i16(x), |mut d| d.i16().unwrap() == x)
}

#[quickcheck]
fn prop_identity_i32(x: i32) -> bool {
    identity(5, |mut e| e.i32(x), |mut d| d.i32().unwrap() == x)
}

#[quickcheck]
fn prop_identity_i64(x: i64) -> bool {
    identity(9, |mut e| e.i64(x), |mut d| d.i64().unwrap() == x)
}

#[quickcheck]
fn prop_identity_f32(x: f32) -> bool {
    identity(5, |mut e| e.f32(x), |mut d| d.f32().unwrap() == x)
}

#[quickcheck]
fn prop_identity_f64(x: f64) -> bool {
    identity(9, |mut e| e.f64(x), |mut d| d.f64().unwrap() == x)
}

#[quickcheck]
fn prop_identity_bytes(x: Vec<u8>) -> bool {
    identity(9 + x.len(), |mut e| e.bytes(&x[..]), |mut d| d.bytes().unwrap() == x)
}

#[quickcheck]
fn prop_identity_bytes_stream(x: Vec<Vec<u8>>) -> bool {
    let len = 2 + x.iter().map(|v| 9 + v.len()).sum::<usize>();
    identity(len, |mut e| e.bytes_indef(|mut e| {
        for v in &x {
            try!(e.bytes(v))
        }
        Ok(())
    }),
    |mut d| {
        match d.value().unwrap() {
            Value::Bytes(Bytes::Chunks(chunks)) =>
                x.iter().zip(chunks.iter()).all(|(x, y)| x == y),
            _ => false
        }
    })
}

#[quickcheck]
fn prop_identity_text(x: String) -> bool {
    identity(9 + x.len(), |mut e| e.text(&x[..]), |mut d| d.text().unwrap() == x)
}

#[quickcheck]
fn prop_identity_text_stream(x: Vec<String>) -> bool {
    let len = 2 + x.iter().map(|v| 9 + v.len()).sum::<usize>();
    identity(len, |mut e| e.text_indef(|mut e| {
        for v in &x {
            try!(e.text(&v[..]))
        }
        Ok(())
    }),
    |mut d| {
        match d.value().unwrap() {
            Value::Text(Text::Chunks(chunks)) =>
                x.iter().zip(chunks.iter()).all(|(x, y)| x == y),
            _ => false
        }
    })
}

fn identity<F, G>(len: usize, enc: F, dec: G) -> bool
where F: Fn(Encoder<Cursor<&mut [u8]>>) -> EncodeResult,
      G: Fn(Decoder<Cursor<&[u8]>>) -> bool
{
    let mut buffer = vec![0u8; len];
    match enc(Encoder::new(Cursor::new(&mut buffer[..]))) {
        Ok(_)  => (),
        Err(e) => panic!("encoder failure: {:?}", e)
    }
    dec(Decoder::new(Config::default(), Cursor::new(&buffer)))
}
