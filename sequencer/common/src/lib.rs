mod metrics;
use itertools::Itertools;
pub use metrics::{report_account_balance, serve_metrics};
pub fn get_chunks(start: u128, end: u128, chunk_size: usize) -> Vec<(u128, u128)> {
    (start..=end)
        .chunks(chunk_size)
        .into_iter()
        .map(|elem| {
            let mut x = elem.into_iter();
            let first = x.next().expect("at least on element in chunk");
            let last = x.last().unwrap_or(first);
            (first, last)
        })
        .collect::<Vec<_>>()
}

pub fn parse_addr(s: &str) -> Result<[u8; 20], ::hex::FromHexError> {
    let mut result = [0u8; 20];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 42) => hex::decode(&s[2..]),
        (false, 40) => hex::decode(s),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(parse_result.as_ref());
    Ok(result)
}

pub fn parse_pkey(s: &str) -> Result<[u8; 32], ::hex::FromHexError> {
    let mut result = [0u8; 32];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 66) => hex::decode(&s[2..]),
        (false, 64) => hex::decode(s),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(parse_result.as_ref());
    Ok(result)
}

// mockall does not allow for async acion withing calls like `returning` or `return_once`
// in the sam time in tests annotated with #[tokio::test] we want to avoid blocking execution
// thread, therefore its nice wrapper for polling based async checking for oneshot channel
pub trait TryReceiveAsync {
    fn recv_timeout_async(
        &self,
        timeout: std::time::Duration,
    ) -> impl std::future::Future<Output = Result<(), oneshot::TryRecvError>>;
}

impl TryReceiveAsync for oneshot::Receiver<()> {
    async fn recv_timeout_async(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), oneshot::TryRecvError> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            match self.try_recv() {
                Ok(_) => break,
                Err(e) if std::time::Instant::now() > deadline => return Err(e),
                _ => {
                    tokio::time::sleep(tokio::time::Duration::from_secs_f64(0.05)).await;
                }
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_chunks() {
        let chunks = get_chunks(0, 10, 3);
        assert_eq!(chunks, vec![(0, 2), (3, 5), (6, 8), (9, 10)]);
    }
}
