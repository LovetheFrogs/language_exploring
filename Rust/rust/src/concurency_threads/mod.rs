use std::thread;
use std::time::Duration;
use std::sync::mpsc;

/** To create a new thread, you just call the thread::spawn function, while passing a closure with the code we want to run as an argument. This code prints some text
 *  from a main thread and other text from a new thread. Note that when the main thread ends, all spawned threads are shut down. To avoid this, we save the return value
 *  of the thread::spawn method to a variable of type `JoinHandle`. Then, you can call join() on this variable, wich waits for the thread to finish. It will also block
 *  the thread currently running until the handled one is done.
 */
#[test]
fn print_from_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

/** To capture variables from the enviroment and use them inside threads, the `move` keyword is needed, as the compiler does not know if the variable borrowed will
 *  be valid for the whole lifetime of the thread.
 */
#[test]
fn move_in_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/* Data Transfer between threads (mpsc -> Multiple Producer, Single Consumer) */

/** tx and rx names are commonly used for channels. Bote that the `send()` method returns an Result<T, E> because the receiving end may beclosed, in wich case, the 
 *  Err variant is returned. The `recv()` method blocks the thread it is called in until something is received. We could call `try_recv()` instead to avoid this 
 *  blocking. */
#[test]
fn communicate_with_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/** Here, we don't call `rx.recv()` explicitly anymore, we instead treat rx as an iterator. We print each value received and, when the channel is closed, iteration 
 *  will end. */
#[test]
fn send_messages_and_see_receiver_waiting() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/** This is an example of multiple producers sending to one consumer through one channel.
 */
#[test]
fn multiple_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}