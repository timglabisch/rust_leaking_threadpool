extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {

    let n_workers = 8;
    let n_jobs = 100000000;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            let foo = (0..1000000).map(|_| "X").collect::<String>();
            tx.send(1).unwrap();
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), n_jobs);
}
