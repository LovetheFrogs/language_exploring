use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

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

/** We create a Mutex<i32>, and then we get the lock in the inner scope and change the value pointed to from 5 to 6. The `lock()` method will return a `LockResult`
 *  or fail if another thread hlding the lock failed. We've chosen to use the unwrap method to panic! in that situation. In case it unwrap doesn't fail, it returns
 *  a MutexGuard<i32> smart pointer, which implements the Deref trait to point at our inner data. It also has a Drop implementation that releases the lock when a 
 *  MutexGuard goes out of scope so we don't risk forgetting to release the lock and blocking the mutex from being used by other threads.
 */
#[test]
fn mutex_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

/** In the following example, we create 10 threads that will add one to the value of the lock so it gets to 10. To share a Mutex<T> between threads, we will have to 
 *  use a smart pointer. Rc<T> is what comes first into mind, but it is not htread safe, so we use Arc<T> instead, which stands for an atomically reference counted
 *  type. We have to use this because with the usage of the move keyword in the thread closure, we take the mutex out of scope for other threads to use. You can 
 *  realise that the usage of Arc<Mutex<T>> is similar to Rc<RefCell<T>>, and while the later alows us to create the dangerous cycles, the former makes the appereance
 *  of Deadlocks possible. This happens when a lock is called on two variables by different threads, and so each thread is waiting for the other to free the lock, so
 *  it just stays in that locked state.
 */
#[test]
fn use_threads_increment_value() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}