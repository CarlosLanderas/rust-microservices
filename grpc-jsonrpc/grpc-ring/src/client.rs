use grpc_ring::Remote;
use failure::Error;

fn main() -> Result<(), Error> {
    let client = Remote::new("127.0.0.1:8000".parse()?)?;

    for n in 1..10 {
        let reply = client.send_hello_request(format!("Carlos Landeras {}", n));
        println!("[Client] Reply from server: {}", reply.message);
    }

    Ok(())
}