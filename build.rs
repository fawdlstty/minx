fn main() {
    if let Err(err) = tonic_build::configure().compile(
        &[
            std::path::Path::new("google/protobuf/empty.proto"),
            //std::path::Path::new("google/protobuf/wrappers.proto"),
            std::path::Path::new("pb.proto"),
        ],
        &[std::path::Path::new("./")],
    ) {
        panic!("{}", err);
    }
}
