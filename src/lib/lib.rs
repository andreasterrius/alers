#![allow(non_upper_case_globals)]
pub extern crate approx;
pub extern crate cgmath;
pub extern crate fbxcel_dom;
pub extern crate gl;
pub extern crate glfw;
pub extern crate image;
#[macro_use]
pub extern crate log as lg;
pub extern crate rand;
pub extern crate rodio;
pub extern crate rusttype;
pub extern crate simplelog;
pub extern crate snowflake;
pub extern crate time;
pub extern crate unicode_normalization;
#[macro_use]
pub extern crate bitflags;

use std::path::Path;
use std::str;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use cgmath::{Matrix4, Vector2, Vector3};
use cgmath::prelude::*;

use self::glfw::{Action, Context, Key};

#[macro_use]
pub mod macros;
pub mod log;
pub mod resource;
pub mod math;
pub mod engine;
pub mod window;
pub mod renderer;
pub mod data;
pub mod camera;
pub mod input;
