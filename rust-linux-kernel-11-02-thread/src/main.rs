use std::{thread, time::Duration};

fn main() {
    // 스레드를 생성하고 실행합니다
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread running: count {i}");
            thread::sleep(Duration::from_millis(500));
        }
    });

    // 부모 스레드가 자식 스레드의 종료를 기다립니다
    handle
        .join()
        .expect("Failed to wait for the thread to complete");
    println!("Thread completed");
}
