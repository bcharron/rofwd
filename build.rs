extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/opentelemetry/proto/metrics/v1/metrics.proto"], &["src/"]).unwrap();
}
