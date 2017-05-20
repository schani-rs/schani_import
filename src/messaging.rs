extern crate amq_protocol;
extern crate futures;
extern crate tokio_core;
extern crate lapin_futures as lapin;

use self::amq_protocol::types::FieldTable;
use self::futures::future::Future;
use self::tokio_core::reactor::Core;
use self::tokio_core::net::TcpStream;
use self::lapin::client::ConnectionOptions;
use self::lapin::channel::{BasicPublishOptions, QueueDeclareOptions, BasicProperties};

pub fn send_processing_message(file_id: u64) {
    // create the reactor
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:5672".parse().unwrap();

    core.run(TcpStream::connect(&addr, &handle)
                 .and_then(|stream| {
                               // connect() returns a future of an AMQP Client
                               // that resolves once the handshake is done
                               lapin::client::Client::connect(stream, &ConnectionOptions::default())
                           })
                 .and_then(|client| {

                               // create_channel returns a future that is resolved
                               // once the channel is successfully created
                               client.create_channel()
                           })
                 .and_then(|channel| {
            let id = channel.id;
            println!("created channel with id: {}", id);

            // we using a "move" closure to reuse the channel
            // once the queue is declared. We could also clone
            // the channel
            channel
                .queue_declare("raw", &QueueDeclareOptions::default(), FieldTable::new())
                .and_then(move |_| {
                              println!("channel {} declared queue {}", id, "hello");

                              channel.basic_publish("raw",
                                                    file_id.to_string().as_bytes(),
                                                    &BasicPublishOptions::default(),
                                                    BasicProperties::default())
                          })
        }))
        .unwrap();
}
