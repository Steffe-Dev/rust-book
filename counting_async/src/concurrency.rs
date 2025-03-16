use std::{
    pin::{Pin, pin},
    time::Duration,
};

pub fn counting() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}

pub fn counting_join() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

pub fn channel() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("Got: {val}");
            }
        };

        let tx1_fut = async move {
            let vals = vec![
                String::from("this"),
                String::from("is"),
                String::from("from"),
                String::from("tx1"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(900)).await;
            }
        };
        trpl::join!(tx_fut, tx1_fut, rx_fut);
    });
}

pub fn channel_dyn_futures() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("Got: {val}");
            }
        });

        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("this"),
                String::from("is"),
                String::from("from"),
                String::from("tx1"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(900)).await;
            }
        });
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx1_fut, rx_fut];

        trpl::join_all(futures).await;
    });
}

pub fn race() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        trpl::Either::Left(res) => Ok(res),
        trpl::Either::Right(_) => Err(max_time),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_counting() {
        counting();
    }

    #[test]
    fn timeout_works() -> Result<(), Duration> {
        trpl::run(async {
            let future = async {
                trpl::sleep(Duration::from_millis(10)).await;
                true
            };

            let result = timeout(future, Duration::from_millis(20)).await?;
            assert!(result);
            Ok(())
        })
    }
}
