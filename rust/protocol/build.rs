// #[cfg(feature = "grpc")]
// fn main() {
//     println!("cargo:rerun-if-changed=src/protos");
//     tonic_build::configure()
//         .build_server(false)
//         .build_client(false)
//         .out_dir(&"src/flwr")
//         .compile(
//             &[
//                 "./src/flwr/proto/fleet.proto",
//                 "./src/flwr/proto/driver.proto",
//                 "./src/flwr/proto/transport.proto",
//             ],
//             &["./src/"],
//         )
//         .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
// }

// #[cfg(not(feature = "grpc"))]
fn main() {
    println!("cargo:rerun-if-changed=src/flwr/proto");

    use prost_build::Config;

    Config::new()
        .bytes(["."])
        // .type_attribute(".", "#[derive(PartialOrd)]")
        .compile_protos(
            &[
                "./src/flwr/proto/fleet.proto",
                "./src/flwr/proto/driver.proto",
                "./src/flwr/proto/transport.proto",
            ],
            &["./src/"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
