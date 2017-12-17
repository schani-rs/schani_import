use std::env;
use std::net;

use amq_protocol::types::FieldTable;
use dotenv::dotenv;
use futures::future::Future;
use lapin;
use lapin::client::ConnectionOptions;
use lapin::channel::{BasicConsumeOptions, BasicGetOptions, BasicProperties, BasicPublishOptions,
                     ConfirmSelectOptions, ExchangeBindOptions, ExchangeDeclareOptions,
                     ExchangeDeleteOptions, ExchangeUnbindOptions, QueueBindOptions,
                     QueueDeclareOptions};
use resolve::resolve_host;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

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

    core.run(
        TcpStream::connect(&addr, &handle)
            .and_then(|stream| {
                // connect() returns a future of an AMQP Client
                // that resolves once the handshake is done
                lapin::client::Client::connect(stream, &ConnectionOptions::default())
            })
            .and_then(|(client, _)| {
                // create_channel returns a future that is resolved
                // once the channel is successfully created
                client.create_confirm_channel(ConfirmSelectOptions::default())
            })
            .and_then(|channel| {
                let id = channel.id;
                println!("created channel with id: {}", id);

                // we using a "move" closure to reuse the channel
                // once the queue is declared. We could also clone
                // the channel
                channel
                    .queue_declare("raw", &QueueDeclareOptions::default(), &FieldTable::new())
                    .and_then(move |_| {
                        println!("channel {} declared queue {}", id, "raw");

                        println!("attempting to push image_id {}", file_id);

                        channel
                            .basic_publish(
                                "raw",
                                "raw",
                                file_id.to_string().as_bytes(),
                                &BasicPublishOptions::default(),
                                BasicProperties::default(),
                            )
                            .map(|confirmation| {
                                println!("publish got confirmation: {:?}", confirmation)
                            })
                            .and_then(move |_| {
                                println!("closing amqp connection …");
                                channel.close(200, "Bye")
                            })
                    })
            }),
    ).expect("fail");
}
