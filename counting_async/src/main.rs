use counting_async::concurrency;

fn main() {
    // concurrency::counting();
    // concurrency::counting_join();
    // concurrency::channel();
    // concurrency::channel_dyn_futures();
    concurrency::race();
}
