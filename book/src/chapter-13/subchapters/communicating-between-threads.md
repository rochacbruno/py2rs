### Communicating between threads

Managing data context between threads.

**Python**

```python
from queue import Queue
queue = Queue()
# ...
# Send message from a thread
queue.put(353)


# ...
# Get message to a thread
val = queue.get()
```

**Rust**

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sender = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val.clone()).unwrap();
        println!("Sent {}", val);
    });

    let receiver = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    });

    sender.join();
    receiver.join();
}
```
