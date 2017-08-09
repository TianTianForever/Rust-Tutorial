#![crate_type="lib"]
#![crate_name="future"]
use std::option::Option;
use std::result::Result;
use std::rc::Rc;
use std::cell::Cell;

// Concrete type.
pub struct MyWorld;
pub struct Land;

#[derive(Debug)]
pub struct River {
    pub water: Rc<Box<Option<String>>>,   
}

impl River {
    fn new() -> Self {
        Self {
            water: Rc::new(Box::new(Some(String::from("water.")))),
        }
    }
}

#[derive(Debug)]
pub struct Sky {
    pub blue_sky: String,
    pub clouds: Box<String>,
}

impl Sky {
    fn new() -> Self {
        Self {
            blue_sky: String::from("Blue sky!!!"),
            clouds: Box::new(String::from("There are some clouds in the sky.")),
        }
    }

    fn take_sky(self) -> String {
        self.blue_sky
    }

    fn take_clounds(self) -> Box<String> {
        self.clouds
    }
}
pub struct Plant;
pub struct Animal;
pub struct Yesterday;
pub struct Today;
pub struct Tomorrow;
pub struct Future;
pub struct Tree {
    pub node: Box<u32>,
    pub branches: Cell<Box<u32>>, 
}
