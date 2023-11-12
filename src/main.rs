use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex}; // mpsc = multiple producer single consumer

fn main() {
    /*
        using threads to run concurrent code.
     */
    // let v = vec![1,2,3,4,5,6,7,8,9,10];

    // let handle = thread::spawn( move || {
    //     for i in 0..10 {
    //         println!("hiho number {:?} from the spawned thread!", v[i]);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // Changing the placement of the join function will change the behaviour of our output.
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("HeyHo number {} from the main thread!!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // //handle.join().unwrap();


    /*
         using message passing w/ MPSC example. Cloning out transmitter
         to send multple messages to a single receiver 
     */ 
//     let (_transmitter, _receiver) = mpsc::channel();
//     let _tr_clone = _transmitter.clone();

//     thread::spawn( move || {
//        let vals = vec![
//         String::from("Hi there"),
//         String::from("I am"),
//         String::from("coming from"),
//         String::from("another thread"),
//         String::from("hoho")
//        ];

//        for v in vals {
//         _tr_clone.send(v).unwrap();
//         thread::sleep(Duration::from_secs(1));
//        }
//     });

//     thread::spawn( move || {
//         let vals = vec![
//          String::from("Now"),
//          String::from("I am"),
//          String::from("sending more"),
//          String::from("from another another"),
//          String::from("another thread..")
//         ];
 
//         for v in vals {
//          _transmitter.send(v).unwrap();
//          thread::sleep(Duration::from_secs(1));
//         }
//      });

//    for _returned_message in _receiver{
//     println!("We got back the message: {}", _returned_message);
//    }
   
   /*
        Shared state concurrency - using mutexs / mutual exclusion primitive
    */
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
 