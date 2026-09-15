use std::process::Command;

fn main() {
    // 부모 프로세스에서 자식 프로세스를 생성합니다
    // 여기서는 "ls" 명령어를 실행하는 프로세스를 생성합니다
    let mut child_process = Command::new("ls")
        .arg("-l")
        .spawn()
        .expect("Failed to spawn child process");

    // 부모 프로세스가 자식 프로세스의 종료를 기다립니다
    let status = child_process
        .wait()
        .expect("Failed to wait for child process to complete");

    // 자식 프로세스의 종료 상태를 출력합니다
    println!("Child process exited with status: {status:?}");
}
