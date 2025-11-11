use quokkasim::prelude::*;
use quokkasim::define_model_enums;
use std::{error::Error, time::Duration};

define_model_enums! {
    pub enum ComponentModel<'a> {}
    pub enum ComponentLogger<'a> {}
}

impl<'a> CustomComponentConnection for ComponentModel<'a> {
    fn connect_components(a: Self, b: Self) -> Result<(), Box<dyn Error>> {
        Err(format!("connect_components not implemented from {} to {}", a, b).into())
    }
}

impl<'a> CustomLoggerConnection<'a> for ComponentLogger<'a> {
    type ComponentType = ComponentModel<'a>;
    fn connect_logger(a: Self, b: Self::ComponentType) -> Result<(), Box<dyn Error>> {
        Err(format!("connect_logger not implemented from {} to {}", a, b).into())
    }
}

fn main() {
    println!("Hello, world!");
}