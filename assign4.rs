use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut producer_handles = vec![];
    let tx_clone1 = tx.clone();
    producer_handles.push(thread::spawn(move || {
        producer(1, tx_clone1, ITEM_COUNT / 2);
    }));

    let tx_clone2 = tx.clone();
    producer_handles.push(thread::spawn(move || {
        producer(2, tx_clone2, ITEM_COUNT / 2);
    })); 
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();

    for i in 0..item_count {
        let value = rng.gen_range(1..100);
        println!("Producer {} sending value: {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished producing.", id);
}


// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let received = rx.lock().unwrap().recv().unwrap();

        if received == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }

        println!("Consumer {} processing value: {}", id, received);
        thread::sleep(Duration::from_millis(200));
    }

    println!("Consumer {} exiting.", id);
}