use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (send1, recv1) = mpsc::channel();
    let (send2, recv2) = mpsc::channel();

    let handle = thread::spawn(move || {
        for val in recv1 {
            println!("t1: {}", val);
            if val > 100 {
                break;
            }
            thread::sleep(Duration::from_millis(100));
            send2.send(val + 1).unwrap();
        }
    });

    thread::spawn(move || {
        send1.send(0).unwrap();
        for val in recv2 {
            println!("t2: {}", val);
            if val > 100 {
                break;
            }
            thread::sleep(Duration::from_millis(100));
            send1.send(val + 1).unwrap();
        }
    });

    handle.join().unwrap();
}
