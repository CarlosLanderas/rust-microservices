use futures::sync::{oneshot, mpsc};
use futures::{future, Future, IntoFuture};


fn to_box<T>(fut: T) -> Box<dyn Future<Item=(), Error=()> + Send>
 where
    T: IntoFuture,
    T::Future: Send + 'static,
    T::Item: 'static,
    T::Error: 'static
{
    let fut = fut.into_future().map(drop).map_err(drop);
    Box::new(fut)
}

fn single() {
    let (tx_sender, rx_future) = oneshot::channel::<u8>();
    let receiver = rx_future.map(|x| {
        println!("Received: {}", x);
    });

    let sender = tx_sender.send(8);
    let execute_all = future::join_all(vec![
        to_box(receiver),
        to_box(sender),
    ]).map(drop);

    tokio::run(execute_all);
}


fn main() {
    single();

}