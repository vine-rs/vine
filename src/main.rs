use errors::Status;

fn main() {
    let s = Status::internal_server_error("io.vine", "internal");
    println!("{}", s);
}
