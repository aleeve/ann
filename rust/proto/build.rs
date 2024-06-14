fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "./src/flwr/proto/fleet.proto",
                "./src/flwr/proto/driver.proto",
                "./src/flwr/proto/transport.proto",
            ],
            &["./src/"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
