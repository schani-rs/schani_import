use std::env;
use std::net;

use amq_protocol::types::FieldTable;
use dotenv::dotenv;
use futures::future::{ok, Future};
use lapin;
use lapin::client::ConnectionOptions;
use lapin::channel::{BasicProperties, BasicPublishOptions, ConfirmSelectOptions,
                     ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
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
                lapin::client::Client::connect(
                    stream,
                    &ConnectionOptions {
                        frame_max: 65535,
                        ..Default::default()
                    },
                )
            })
            .and_then(|(client, heartbeat_future_fn)| {
                let heartbeat_client = client.clone();
                handle.spawn(heartbeat_future_fn(&heartbeat_client).map_err(|_| ()));

                client.create_confirm_channel(ConfirmSelectOptions::default())
            })
            .and_then(|channel| {
                let id = channel.id;
                println!("created channel with id: {}", id);

                channel
                    .queue_declare("raw", &QueueDeclareOptions::default(), &FieldTable::new())
                    .map(|_| channel)
            })
            .and_then(|channel| {
                println!("channel {} declared queue {}", channel.id, "raw");
                channel
                    .exchange_declare(
                        "raw_ex",
                        "direct",
                        &ExchangeDeclareOptions::default(),
                        &FieldTable::new(),
                    )
                    .map(|_| channel)
            })
            .and_then(|channel| {
                println!("channel {} declared exchange {}", channel.id, "raw_ex");
                channel
                    .queue_bind(
                        "raw",
                        "raw_ex",
                        "raw_2",
                        &QueueBindOptions::default(),
                        &FieldTable::new(),
                    )
                    .map(|_| channel)
            })
            .and_then(|channel| {
                println!("channel {} bound queue {}", channel.id, "raw");

                println!("attempting to push image_id {}", file_id);

                channel
                    .basic_publish(
                        "raw_ex",
                        "raw_2",
                        file_id.to_string().as_bytes(),
                        &BasicPublishOptions::default(),
                        BasicProperties::default(),
                    )
                    .join(ok(channel))
            })
            .map(|(confirmation, channel)| {
                println!("publish got confirmation: {:?}", confirmation);
                channel
            })
            .and_then(|channel| {
                println!("closing amqp connection …");
                channel.close(200, "Bye")
            }),
    ).expect("fail");
}
