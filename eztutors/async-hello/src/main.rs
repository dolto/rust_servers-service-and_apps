use std::{
    thread::{sleep, spawn},
    time::Duration,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // 네이티브 환경에서 멀티스레딩 프로그램을 작성하는 방법이지만, 비동기가 아니라 멀티스레딩일 뿐이다.
    // let handle1 = spawn(|| {
    //     let file_contents = read_from_file(Duration::new(4, 0), "Hello There".to_owned());
    //     println!("{:?}", file_contents);
    // });
    // let handle2 = spawn(|| {
    //     let file_contents = read_from_file(Duration::new(2, 0), "Hello There2".to_owned());
    //     println!("{:?}", file_contents);
    // });
    // let handle3 = spawn(|| {
    //     let file_contents = read_from_file(Duration::new(4, 0), "Hello There3".to_owned());
    //     println!("{:?}", file_contents);
    // });

    // handle1.join().unwrap();
    // handle2.join().unwrap();
    // handle3.join().unwrap();

    // tokio를 이용한 비동기 멀티스레딩 프로그래밍을 하는 방법은 다음과같다
    let handle1 = tokio::spawn(async {
        let file_contents = read_from_file(Duration::new(4, 0), "Hello There".to_owned()).await;
        println!("{:?}", file_contents);
    });
    let handle2 = tokio::spawn(async {
        let file_contents = read_from_file(Duration::new(2, 0), "Hello There2".to_owned()).await;
        println!("{:?}", file_contents);
    });
    let handle3 = tokio::spawn(async {
        let file_contents = read_from_file(Duration::new(4, 0), "Hello There3".to_owned()).await;
        println!("{:?}", file_contents);
    });

    let _ = tokio::join!(handle1, handle2, handle3);
}

async fn read_from_file(secs: Duration, message: String) -> String {
    sleep(secs);
    message
}
