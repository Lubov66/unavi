const SCHEMAS: &str = "../../wired-protocol/world/schemas";

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix(SCHEMAS)
        .file(format!("{}/world_server.capnp", SCHEMAS))
        .file(format!("{}/datagram.capnp", SCHEMAS))
        .run()
        .expect("schema compiler command");
}
