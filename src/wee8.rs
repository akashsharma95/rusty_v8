// Copyright 2019-2021 the Deno authors. All rights reserved. MIT license.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::support::Opaque;
use paste::paste;

// Ownership

macro_rules! WASM_DECLARE_OWN {
  ($name:ident) => {
    paste! {
      #[repr(C)]
      pub struct [<wasm_ $name _t>](Opaque);
      extern "C" {
        pub fn [<wasm_ $name _delete>](_: *mut [<wasm_ $name _t>]);
      }
    }
  };
}

// Vectors

macro_rules! WASM_DECLARE_VEC {
  ($name:ident, $ty:ty) => {
    paste! {
      #[repr(C)]
      pub struct [<wasm_ $name _vec_t>] {
        pub size: usize,
        pub data: *mut $ty,
      }
      extern "C" {
        pub fn [<wasm_ $name _vec_new_empty>](_: *mut [<wasm_ $name _vec_t>]);
        pub fn [<wasm_ $name _vec_new_uninitialized>](
          _: *mut [<wasm_ $name _vec_t>],
          _: usize,
        );
        pub fn [<wasm_ $name _vec_new>](
          _: *mut [<wasm_ $name _vec_t>],
          _: usize,
          _: *const $ty,
        );
        pub fn [<wasm_ $name _vec_copy>](
          _: *mut [<wasm_ $name _vec_t>],
          _: *const [<wasm_ $name _vec_t>],
        );
        pub fn [<wasm_ $name _vec_delete>](_: *mut [<wasm_ $name _vec_t>]);
      }
    }
  };
}

// Byte vectors

pub type wasm_byte_t = i8;

WASM_DECLARE_VEC!(byte, wasm_byte_t);

///////////////////////////////////////////////////////////////////////////////
// Runtime Environment

// Configuration

WASM_DECLARE_OWN!(config);

extern "C" {
  pub fn wasm_config_new() -> *mut wasm_config_t;
}

// Engine

WASM_DECLARE_OWN!(engine);

extern "C" {
  pub fn wasm_engine_new() -> *mut wasm_engine_t;
  pub fn wasm_engine_new_with_config(
    _: *mut wasm_config_t,
  ) -> *mut wasm_engine_t;
}

// Store

WASM_DECLARE_OWN!(store);

extern "C" {
  pub fn wasm_store_new(_: *mut wasm_engine_t) -> *mut wasm_store_t;
}

///////////////////////////////////////////////////////////////////////////////
// Type Representations

// Type attributes

pub type wasm_mutability_t = u8;

#[repr(u8)]
pub enum wasm_mutability_enum {
  WASM_CONST,
  WASM_VAR,
}

#[repr(C)]
pub struct wasm_limits_t {
  min: u32,
  max: u32,
}

const wasm_limits_max_default: u32 = 0xffffffff;

// Generic

macro_rules! WASM_DECLARE_TYPE {
  ($name:ident) => {
    paste! {
      WASM_DECLARE_OWN!($name);
      WASM_DECLARE_VEC!($name, *mut [<wasm_ $name _t>]);
      extern "C" {
        pub fn [<wasm_ $name _copy>](_: *mut [<wasm_ $name _t>]);
      }
    }
  };
}

// Value Types

WASM_DECLARE_TYPE!(valtype);

pub type wasm_valkind_t = u8;

#[repr(u8)]
pub enum wasm_valkind_enum {
  WASM_I32,
  WASM_I64,
  WASM_F32,
  WASM_F64,
  WASM_ANYREF = 128,
  WASM_FUNCREF,
}

extern "C" {
  pub fn wasm_valtype_new(
    _: *mut wasm_valkind_t,
  ) -> *mut wasm_valtype_t;

  fn wasm_valtype_kind(
    _: *const wasm_valtype_t,
  ) -> *mut wasm_valkind_t;
}

// Function Types

WASM_DECLARE_TYPE!(functype);

extern "C" {
  pub fn wasm_functype_new(
    _: *mut wasm_valtype_vec_t,
    _: *mut wasm_valtype_vec_t,
  ) -> *mut wasm_functype_t;

  fn wasm_functype_params(
    _: *const wasm_functype_t,
  ) -> *const wasm_valtype_vec_t;

  fn wasm_functype_results(
    _: *const wasm_functype_t,
  ) -> *const wasm_valtype_vec_t;
}

// Global Types

WASM_DECLARE_TYPE!(globaltype);

extern "C" {
  pub fn wasm_globaltype_new(
    _: *mut wasm_valtype_t,
    _: *mut wasm_mutability_t,
  );

  fn wasm_globaltype_content(
    _: *const wasm_globaltype_t,
  ) -> *const wasm_valtype_t;

  fn wasm_globaltype_mutability(
    _: *const wasm_globaltype_t,
  ) -> *mut wasm_mutability_t;
}

// Table Types

WASM_DECLARE_TYPE!(tabletype);

extern "C" {
  pub fn wasm_tabletype_new(
    _: *mut wasm_valtype_t,
    _: *const wasm_limits_t,
  ) -> *mut wasm_tabletype_t;

  fn wasm_tabletype_element(
    _: *const wasm_tabletype_t,
  ) -> *const wasm_valtype_t;

  fn wasm_tabletype_limits(
    _: *const wasm_tabletype_t,
  ) -> *mut wasm_limits_t;
}

// Memory Types

WASM_DECLARE_TYPE!(memorytype);

extern "C" {
  pub fn wasm_memorytype_new(
    _: *const wasm_limits_t,
  ) -> *mut wasm_memorytype_t;

  fn wasm_memorytype_limits(
    _: *const wasm_memorytype_t,
  ) -> *const wasm_limits_t;
}

///////////////////////////////////////////////////////////////////////////////
// Runtime Objects

// extern "C" {
//   pub fn wasm_module_new(
//     _: *mut wasm_store_t,
//     _: *mut wasm_byte_vec_t,
//   ) -> *mut wasm_module_t;
// }