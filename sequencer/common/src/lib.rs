mod metrics;
use itertools::Itertools;

pub use metrics::{report_account_balance, serve_metrics};

pub fn get_chunks(start: u128, end: u128, chunk_size: usize) -> Vec<(u128, u128)> {
    (start..=end)
        .chunks(chunk_size)
        .into_iter()
        .map(|elem| {
            let mut x = elem.into_iter();
            let first = x.nth(0).expect("at least on element in chunk");
            let last = x.last().unwrap_or(first);
            (first, last)
        })
        .collect::<Vec<_>>()
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
