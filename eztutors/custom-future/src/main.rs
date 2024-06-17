use std::{
    future::Future,
    task::Poll,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    println!("Hello before reding File");

    let h1 = tokio::spawn(async {
        let f1 = ReadFileFuture {};
        f1.await
    });
    let h2 = tokio::spawn(async {
        let f2 = read_from_file(Duration::new(2, 0), "Hello There");
        f2.await
    });

    let h3 = tokio::spawn(async {
        let f1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        };
        println!("{:?}", f1.await);
    });

    let _ = tokio::join!(h1, h2, h3);
}

async fn read_from_file(dur: Duration, message: &str) -> &str {
    sleep(dur);
    println!("{}", message);
    message
}

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>, //메모리에 특정 영역을 고정하는것을 보장
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("Tokio! Stop pooling me");
        cx.waker().wake_by_ref();
        std::task::Poll::Ready("Hello from poll function".to_owned())
    }
}

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future");
            Poll::Ready("Future has completed".to_owned())
        } else {
            println!("Hello, it's not yet time for Future. Going to sleep");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    sleep(expiration_time - current_time);
                    waker.wake();
                }
            });
            Poll::Pending
        }
    }
}
