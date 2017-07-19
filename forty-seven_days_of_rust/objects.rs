#![crate_type="lib"]
#![crate_name="objects"]
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

pub struct SelectBox {
    pub width: i32,
    pub height: i32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self.width * self.height);
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
#[derive(Debug, Clone)]
pub struct AveragedCollection {
    pub list: Vec<i32>,
    pub average: f64,
}

impl AveragedCollection {
    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        
    }
   
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }
}
