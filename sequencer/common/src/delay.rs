use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::future::Future;

use futures::select;
use futures::{stream::BoxStream, FutureExt, StreamExt};
use tokio::sync::mpsc::{Receiver, Sender};

#[tracing::instrument(skip_all)]
pub async fn delay_channel_impl<'a, T: Debug + Clone, E: Display>(
    mut input: Receiver<T>,
    output: Sender<T>,
    mut ticker: BoxStream<'a, Result<u128, E>>,
    delay: u128,
) -> Result<(), E> {
    tracing::info!("delay stream started with delay = {delay}");
    let mut buffer: BTreeMap<u128, Vec<T>> = BTreeMap::new();
    let current_block = ticker.next().await.expect("infinite stream")?;
    loop {
        select! {
            elem = input.recv().fuse() => {
                match elem{
                    Some(elem) => {
                        buffer.entry(current_block + delay).and_modify(|elems| elems.push(elem.clone())).or_insert(vec![elem]);
                    },
                    None => {
                        tracing::error!("input channel closed");
                        break;
                    }
                }
            },
            block_nr = ticker.next().fuse() => {
                match block_nr {
                    Some(Ok(block_nr)) => {
                        let not_ready = buffer.split_off(&block_nr);
                        let ready = std::mem::replace(&mut buffer, not_ready);
                        for (id, elems) in ready {
                            tracing::debug!("forwarding {} elems from delay queue", elems.len());
                            for e in  elems{
                                output.send(e).await.expect("infinite");
                            }
                        }
                    },
                    None => {
                        tracing::error!("blocks channel closed");
                        break;
                    },
                    Some(Err(e)) =>{
                        tracing::error!("could not get block number err: {e} ... ignoring");
                    }
                }
            }
        }
    }
    tracing::info!("bye");
    Ok(())
}

use futures::future::BoxFuture;
#[tracing::instrument(skip_all)]
pub fn delay_channel<'a, T, E>(
    input: Receiver<T>,
    output: Sender<T>,
    ticker: BoxStream<'a, Result<u128, E>>,
    delay: u128,
) -> BoxFuture<Result<(), E>>
where
    T: Debug + Clone + Sync + Send + 'a,
    E: std::fmt::Display + Sync + Send + 'a,
{
    if delay > 0 {
        tracing::debug!("starting delay channel with delay set to {delay}");
        delay_channel_impl(input, output, ticker, delay).boxed()
    } else {
        forward_channel(input, output).boxed()
    }
}

pub fn create_delay_channel<'a, T, E>(
    stream: BoxStream<'a, Result<u128, E>>,
    delay: u128,
) -> (
    Sender<T>,
    Receiver<T>,
    impl Future<Output = Result<(), E>> + use<'_, T, E>,
)
where
    T: Debug + Clone + Sync + Send + 'a,
    E: std::fmt::Display + Sync + Send + 'a,
{
    use tokio::sync::mpsc;
    let (input, receiver) = mpsc::channel(1_000_000);
    let (sender, output) = mpsc::channel(1_000_000);

    let task = delay_channel(receiver, sender, stream, delay);
    (input, output, task)
}

#[tracing::instrument(skip_all)]
async fn forward_channel<'a, T, E>(mut input: Receiver<T>, output: Sender<T>) -> Result<(), E> {
    while let Some(elem) = input.recv().await {
        output.send(elem).await.expect("infinite");
    }
    tracing::info!("bye");
    Ok(())
}

#[cfg(test)]
pub mod test {
    use std::convert::Infallible;

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

    #[macro_export]
    macro_rules! assert_resolves_within {
        ($duration:expr, $future:expr) => {{
            use tokio::time::{timeout, Duration};
            match timeout(Duration::from_secs_f64($duration), $future).await {
                Ok(_) => {}
                Err(_) => panic!("assert_timeout!"),
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
    #[tracing_test::traced_test]
    async fn test_forward_blocks_when_no_delay() {
        let (_block_sender, stream_receiver) = mpsc::channel::<Result<u128, Infallible>>(100);
        let blocks_stream = into_stream(stream_receiver).boxed();
        let (input, mut output, task) =
            create_delay_channel::<u128, Infallible>(blocks_stream, 0u128);

        let _t = tokio::spawn(async move {
            task.await.unwrap();
        });

        assert_timeout!(0.1, output.recv());
        assert_resolves_within!(0.1, input.send(1u128));
        assert_resolves_within!(0.1, output.recv());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_forward_elems_with_delay() {
        let (block_sender, stream_receiver) = mpsc::channel::<Result<u128, Infallible>>(100);
        let blocks_stream = into_stream(stream_receiver).boxed();
        let (input, mut output, task) =
            create_delay_channel::<String, Infallible>(blocks_stream, 10u128);

        let t = tokio::spawn(async move { task.await.unwrap() });

        let elem = "A".to_string();
        block_sender.send(Ok(1u128)).await.unwrap();
        input.send("A".to_string()).await.unwrap();

        block_sender.send(Ok(10u128)).await.unwrap();
        assert_timeout!(0.01, output.recv());

        block_sender.send(Ok(12u128)).await.unwrap();
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
        let (block_sender, stream_receiver) = mpsc::channel::<Result<u128, Infallible>>(100);
        let blocks_stream = into_stream(stream_receiver).boxed();
        let (input, mut output, task) =
            create_delay_channel::<String, Infallible>(blocks_stream, 10u128);

        let t = tokio::spawn(async move {
            task.await.unwrap();
        });

        block_sender.send(Ok(1u128)).await.unwrap();
        input.send("A".to_string()).await.unwrap();

        block_sender.send(Ok(2u128)).await.unwrap();
        input.send("B".to_string()).await.unwrap();
        input.send("C".to_string()).await.unwrap();

        assert_timeout!(0.01, output.recv());
        block_sender.send(Ok(13u128)).await.unwrap();
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
