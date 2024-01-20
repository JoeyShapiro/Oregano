use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel for communication between threads
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread for reading data into the buffer
    thread::spawn(move || {
        // Replace this with your actual data source (e.g., MIDI input)
        let mut data_source = vec![1, 2, 3, 4, 5];

        loop {
            // Read data into the buffer
            let buffer = data_source.clone(); // Clone or handle ownership appropriately
            sender.send(buffer).unwrap();

            // Simulate some delay (replace with actual data reading logic)
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // Main thread processing the received data
    loop {
        // Receive data from the buffer
        let received_data = receiver.recv().unwrap();

        // Process the received data (replace with your processing logic)
        println!("Received: {:?}", received_data);
    }
}
