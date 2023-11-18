use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    //overloading the ops::add() for our Point method
    fn add(self, other: Point) -> Point {
        Point { 
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
    implementing the Add trait where we want to customize the Rhs type rather than using the default
*/
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/*
    Using a thing wrapper to implement traits on types defined outside our crate
    in this case the Display trait on a Vec<T>
*/
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point {x: 3, y: 3 }
    );
    let p = Point {
        x: 1,
        y: 0,
    };
    let result = p.add(Point{x:5, y:9});
    println!("p plus [5, 9] === {:?}", result);

    let mili = Millimeters(980);
    let meter = Meters(2);
    let result = mili.add(meter);
    assert_eq!(
        mili.add(meter),
        Millimeters(2980)
    );
    println!("{:?} + {:?} = {:?}", mili, meter, result);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    /*
    The downside of using this technique is that Wrapper is a new type, 
    so it doesn’t have the methods of the value it’s holding. We would 
    have to implement all the methods of Vec<T> directly on Wrapper such 
    that the methods delegate to self.0, which would allow us to treat 
    Wrapper exactly like a Vec<T>. If we wanted the new type to have every 
    method the inner type has, implementing the Deref trait (discussed in 
    Chapter 15 in the “Treating Smart Pointers Like Regular References 
    with the Deref Trait” section) on the Wrapper to return the inner 
    type would be a solution. If we don’t want the Wrapper type to 
    have all the methods of the inner type—for example, to restrict the
    Wrapper type’s behavior—we would have to implement just the methods we do want manually.
     */
}
