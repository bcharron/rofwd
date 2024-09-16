use std::fs;
use std::io::Cursor;
use prost::Message;
use rofwd::opentelemetry::proto::metrics;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::config::ClientConfig;

fn main() {
    // let data = fs::read("data/metrics.pb").unwrap();

    // println!("Read {} bytes", data.len());

    // let cursor = Cursor::new(data);
    // let r = metrics::v1::MetricsData::decode(cursor);
    let topics = ["metrics"];

    // if r.is_ok() {
    //     println!("Successfully decoded");
    // } else {
    //     eprintln!("Failed to decode metrics: {}", r.unwrap_err())
    // }

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "mygroup")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create consumer");

    consumer.subscribe(&topics).expect("Consumer Subscribe failed");   
}
