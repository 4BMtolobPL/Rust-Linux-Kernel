use std::{error::Error, thread, time::Duration};

use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() -> Result<(), Box<dyn Error>> {
    // SIGINT 신호를 처리하기 위한 Signals 객체를 생성합니다
    let mut signals = Signals::new([SIGINT])?;

    // 새로운 스레드를 생성해 SIGINT 신호를 수신 대기합니다
    thread::spawn(move || {
        // signal.forever()를 사용해 지속적으로 신호를 확인하고 대기합니다
        for _sig in signals.forever() {
            println!("SIGINT 수신");
            // 프로세스를 종료합니다
            std::process::exit(0);
        }
    });

    println!("SIGINT를 수신하거나 Ctrl+C를 입력하면 종료합니다");
    thread::sleep(Duration::from_secs(10));

    Ok(())
}
