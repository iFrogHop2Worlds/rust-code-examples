
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) =>{
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    
    impl Button {
        pub fn new(height: u32, width: u32, label: String) -> Button {
            Button {
                height,
                width,
                label,
            }
        }
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("We drew a button with a width: {}px, height: {}px, label: {}", self.width, self.height, self.label);
        }

    }
}