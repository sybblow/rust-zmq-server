extern crate zmq;
extern crate schedule_recv;

use schedule_recv::periodic_ms;

fn main() {
    let mut context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut count: usize = 0;
    let mut msg = zmq::Message::new().unwrap();
    let tick = periodic_ms(1000);
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        count += 1;

        let to_send = if tick.try_recv().is_ok() {
            format!("{}th visitor", count)
        }
        else {
            "busying".to_string()
        };
        responder.send_str(&to_send, 0).unwrap();
    }
}
