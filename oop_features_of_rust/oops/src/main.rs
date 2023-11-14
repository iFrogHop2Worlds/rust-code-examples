use oops::gui::{Draw, Button, Screen};

fn main() {
    // If someone using our library decides to implement a SelectBox struct that has 
    // width, height, and options fields, they implement the Draw trait on the SelectBox 
    // type as well
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl SelectBox {
        pub fn new(height: u32, width: u32, options: Vec<String>) -> SelectBox {
            SelectBox {
                height,
                width,
                options,
            }
        }
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("We drew a select box with a width: {}px, height: {}px, options: {:?}", self.width, self.height, self.options);
        }
    }

    let button = Button::new(1211, 420, "Hi Ho".to_owned());
    button.draw();

    let options = vec![String::from("69"), String::from("1111"), String::from("1212"), String::from("crymore")];
    let select_bpx = SelectBox::new(69, 69, options);
    select_bpx.draw();

    /*
    Our library’s user can now write their main function to create a Screen instance. 
    To the Screen instance, they can add a SelectBox and a Button by putting each in 
    a Box<T> to become a trait object. They can then call the run method on the Screen 
    instance, which will call draw on each of the components
     */
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 69,
                height: 69,
                options: vec![
                    String::from("Yes"),
                    String::from("Yas"),
                    String::from("Hard Yes"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };
    screen.run();
}
