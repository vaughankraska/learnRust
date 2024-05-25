use futures::{stream, FutureExt};


async fn number_stream() -> impl futures::Stream<Item = u32> {
    stream::iter(1..=10)
}

async fn process_number(number: u32) -> u32 {
    // Simulate an async operation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    number * 2
}

#[tokio::main]
async fn main() {
    let ns = number_stream();
    ns .map(|number| process_number(number))
        .buffer_unordered(5)
        .for_each(|processed_number| {
            println!("Processed number: {:?}", processed_number);
            futures::future::ready(())
        })
    .await;
}
