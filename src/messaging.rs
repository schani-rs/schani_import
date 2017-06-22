extern crate amq_protocol;
extern crate dotenv;
extern crate futures;
extern crate resolve;
extern crate tokio_core;
extern crate lapin_futures as lapin;

use std::env;
use std::net;
use self::amq_protocol::types::FieldTable;
use self::dotenv::dotenv;
use self::futures::future::Future;
use self::lapin::client::ConnectionOptions;
use self::lapin::channel::{BasicPublishOptions, QueueDeclareOptions, BasicProperties};
use self::resolve::resolve_host;
use self::tokio_core::reactor::Core;
use self::tokio_core::net::TcpStream;

pub fn send_processing_message(file_id: i32) {
    dotenv().ok();

    // create the reactor
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let host = env::var("AMQP_ADDRESS").expect("AMQP_ADDRESS must be set");
    let host_addr = resolve_host(&host)
        .expect("could not lookup host")
        .last()
        .unwrap();
    let addr = net::SocketAddr::new(host_addr, 5672);

    println!("connecting to AMQP service at {}", host_addr);

    core.run(TcpStream::connect(&addr, &handle)
                 .and_then(|stream| {
                               // connect() returns a future of an AMQP Client
                               // that resolves once the handshake is done
                               lapin::client::Client::connect(stream, &ConnectionOptions::default())
                           })
                 .and_then(|client| {

                               // create_channel returns a future that is resolved
                               // once the channel is successfully created
                               client.create_confirm_channel()
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
                    println!("channel {} declared queue {}", id, "raw");

                    println!("attempting to push image_id {}", file_id);

                    channel
                        .basic_publish("raw",
                                       file_id.to_string().as_bytes(),
                                       &BasicPublishOptions::default(),
                                       BasicProperties::default())
                        .map(|confirmation| {
                                 println!("publish got confirmation: {:?}", confirmation)
                             })
                        .and_then(move |_| {
                                      println!("closing amqp connection …");
                                      channel.close(200, "Bye".to_string())
                                  })
                })
        }))
        .expect("what what");
}
