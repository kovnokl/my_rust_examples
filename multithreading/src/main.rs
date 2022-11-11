use std::sync::{Arc, Mutex};
use std::thread;

//Mutlithreading example

fn main() {
    //first create a Arc(atomic counted reference) to a mutex(mutable eclosure)
    let sum = Arc::new(Mutex::new(1));
    //vector of all threads
    let mut handles = vec![];
    for _ in 0..5 {
        //create a refrence to the sum variable for this closure
        let sum = Arc::clone(&sum);
        //create threads and choose to move(values by copy)
        let handle = thread::spawn( move || {
            //do something with the mutex
            dosomething(&sum);
            //can still do something with the mutex afterwards
            let mut num = sum.lock().unwrap();
            *num += 1;
            }
        );
        //push thread into vector
        handles.push(handle);
    }

    //join threads
    for handle in handles {
        handle.join().unwrap();
    }

    //print sum
    println!("sum: {}", *sum.lock().unwrap());

    return;
}

fn dosomething(n: &Mutex<i32>)
{
    //lock mutex, so it can be used
    let mut num = n.lock().unwrap();
    *num *= 2;
}
