pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn day1_test() {
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(true)
            .with_level(true)
            .pretty()
            .finish();
        subscriber.init();
    }
}
