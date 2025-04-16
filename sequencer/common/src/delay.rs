
use std::collections::BTreeMap;
use std::fmt::Debug;

use futures::select;
use futures::{stream::BoxStream, FutureExt, StreamExt};
use tokio::sync::mpsc::{Receiver, Sender};


#[tracing::instrument(skip_all)]
async fn delay_channel_impl<'a, T: Debug + Clone>(
    mut input: Receiver<T>,
    output: Sender<T>,
    mut ticker: BoxStream<'a, u128>,
    delay: u128,
) -> Result<(), ()> {
    tracing::info!("delay stream started with delay = {delay}");
    let mut buffer: BTreeMap<u128, Vec<T>> = BTreeMap::new();
    let current_block = ticker.next().await.expect("infinite stream");
    loop {
        select! {
            elem = input.recv().fuse() => {
                tracing::debug!("elem received from input stream {elem:?}");
                match elem{
                    Some(elem) => {
                        buffer.entry(current_block + delay).and_modify(|elems| elems.push(elem.clone())).or_insert(vec![elem]);
                        // buffer.insert(current_block + delay, elem);
                    },
                    None => {
                        tracing::error!("input channel closed");
                        break;
                    }
                }
            },
            block_nr = ticker.next().fuse() => {
                tracing::info!("new block received {block_nr:?}");
                match block_nr {
                    Some(block_nr) => {
                        let not_ready = buffer.split_off(&block_nr);
                        let ready = std::mem::replace(&mut buffer, not_ready);
                        for (id, elems) in ready {
                            tracing::debug!("found {} elems scheduled at {id}", elems.len());
                            for e in  elems{
                                output.send(e).await.expect("infinite");
                            }
                        }
                    },
                    None => {
                        tracing::error!("blocks channel closed");
                        break;
                    },
                }
            }
        }
    }
    tracing::info!("bye");
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn delay_channel<'a, T: Debug + Clone>(
    input: Receiver<T>,
    output: Sender<T>,
    ticker: BoxStream<'a, u128>,
    delay: u128,
) -> Result<(), ()> {
    if delay > 0 {
        delay_channel_impl(input, output, ticker, delay).await?;
    } else {
        forward_channel(input, output).await?;
    }
    Ok(())
}

// pub fn create_delay_channel<T: Debug + Clone>(stream: BoxStream<u128>, delay: u128) -> (Sender<T>, Receiver<T>, JoinHandle<Result<(), ()>>) {
//     let (input, receiver) = mpsc::channel(1_000_000);
//     let (sender, output) = mpsc::channel(1_000_000);
//     let task = tokio::spawn(delay_channel(receiver, sender, stream, delay));
//     (input, output, task)
// }
//
#[tracing::instrument(skip_all)]
async fn forward_channel<'a, T>(mut input: Receiver<T>, output: Sender<T>) -> Result<(), ()> {
    while let Some(elem) = input.recv().await {
        output.send(elem).await.expect("infinite");
    }
    tracing::info!("bye");
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use async_stream::stream;
    use futures::Stream;
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration};
    use tracing_test::traced_test;

    #[macro_export]
    macro_rules! assert_timeout {
        ($duration:expr, $future:expr) => {{
            use tokio::time::{timeout, Duration};
        match timeout(Duration::from_secs_f64($duration), $future).await {
        Ok(result) => panic!("assert_timeout! failed: operation should time out  but it provided value {:?}", result),
        Err(_) => {},
        }
    }};
    }

    fn into_stream<T>(mut receiver: Receiver<T>) -> impl Stream<Item = T> {
        stream! {
            while let Some(elem) = receiver.recv().await {
                yield elem;
            }
        }
    }

    #[tokio::test]
    async fn test_forward_blocks_when_no_delay() {
        let (_block_sender, stream_receiver) = mpsc::channel(100);
        let blocks_stream = into_stream(stream_receiver);

        let (input, receiver) = mpsc::channel(100);
        let (sender, mut output) = mpsc::channel(100);

        let t = tokio::spawn(async move {
            delay_channel(receiver, sender, blocks_stream.boxed(), 0)
                .await
                .unwrap();
        });

        let elem = 1u128;
        input.send(elem).await.unwrap();
        assert_eq!(output.recv().await, Some(elem));

        std::mem::drop(input);
        t.await.unwrap()
    }

    #[tokio::test]
    #[traced_test]
    async fn test_forward_elems_with_delay() {
        let (block_sender, stream_receiver) = mpsc::channel(100);
        let blocks_stream = into_stream(stream_receiver);
        let (input, receiver) = mpsc::channel(100);
        let (sender, mut output) = mpsc::channel(100);

        let t = tokio::spawn(async move {
            delay_channel(receiver, sender, blocks_stream.boxed(), 10)
                .await
                .unwrap();
        });

        let elem = "A".to_string();
        block_sender.send(1u128).await.unwrap();
        input.send("A".to_string()).await.unwrap();

        block_sender.send(10u128).await.unwrap();
        assert_timeout!(0.01, output.recv());

        block_sender.send(12u128).await.unwrap();
        assert_eq!(
            timeout(Duration::from_secs_f64(0.01), output.recv())
                .await
                .unwrap(),
            Some(elem)
        );

        std::mem::drop(input);
        std::mem::drop(block_sender);
        timeout(Duration::from_secs_f64(0.01), t)
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_forward_multiple_elems_with_delay_at_once() {
        let (block_sender, stream_receiver) = mpsc::channel(100);
        let blocks_stream = into_stream(stream_receiver);
        let (input, receiver) = mpsc::channel(100);
        let (sender, mut output) = mpsc::channel(100);

        let t = tokio::spawn(async move {
            delay_channel(receiver, sender, blocks_stream.boxed(), 10)
                .await
                .unwrap();
        });

        block_sender.send(1u128).await.unwrap();
        input.send("A".to_string()).await.unwrap();

        block_sender.send(2u128).await.unwrap();
        input.send("B".to_string()).await.unwrap();
        input.send("C".to_string()).await.unwrap();

        assert_timeout!(0.01, output.recv());
        block_sender.send(13u128).await.unwrap();
        assert_eq!(
            timeout(Duration::from_secs_f64(0.01), output.recv())
                .await
                .unwrap(),
            Some("A".to_string())
        );
        assert_eq!(
            timeout(Duration::from_secs_f64(0.01), output.recv())
                .await
                .unwrap(),
            Some("B".to_string())
        );
        assert_eq!(
            timeout(Duration::from_secs_f64(0.01), output.recv())
                .await
                .unwrap(),
            Some("C".to_string())
        );
        assert_timeout!(0.01, output.recv());

        std::mem::drop(input);
        std::mem::drop(block_sender);
        timeout(Duration::from_secs_f64(0.01), t)
            .await
            .unwrap()
            .unwrap();
        // t.await.unwrap()
    }
}
