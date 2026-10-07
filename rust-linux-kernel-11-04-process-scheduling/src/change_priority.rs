use std::{io, process, thread, time::Duration};

use libc::{
    SCHED_RR, sched_get_priority_max, sched_get_priority_min, sched_getscheduler, sched_param,
    sched_setscheduler,
};
use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, fork, getpid},
};

fn main() {
    println!("프로그램 시작");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            let child_pid = child.as_raw();

            println!("부모: 자식 PID = {child_pid}");

            let before = unsafe { sched_getscheduler(child_pid) };

            if before == -1 {
                eprintln!("sched_getscheduler 실패: {}", io::Error::last_os_error());
                process::exit(1);
            }

            println!("부모: 변경 전 정책 = {before}");

            let min = unsafe { sched_get_priority_min(SCHED_RR) };
            let max = unsafe { sched_get_priority_max(SCHED_RR) };

            println!("SCHED_RR priority 범위: {min} ~ {max}");

            let priority = (min + max) / 2;

            let param = sched_param {
                sched_priority: priority,
            };

            println!("부모: SCHED_RR priority {priority}로 변경 시도");

            let result = unsafe { sched_setscheduler(child_pid, SCHED_RR, &param) };

            if result == -1 {
                let error = io::Error::last_os_error();

                eprintln!("부모: sched_setscheduler 실패");
                eprintln!("부모: errno = {error}");

                if error.raw_os_error() == Some(libc::EPERM) {
                    eprintln!("부모: EPERM - 권한 부족");
                }
            } else {
                println!("부모: sched_setscheduler 성공");
                println!("부모: 반환값 = {result}");

                let after = unsafe { sched_getscheduler(child_pid) };

                println!("부모: 변경 후 정책 = {after}");

                if after == SCHED_RR {
                    println!("부모: SCHED_OTHER -> SCHED_RR 변경 확인");
                }
            }

            println!("부모: 자식 종료 대기");

            if let Err(err) = waitpid(child, None) {
                eprintln!("waitpid 실패: {err}");
            }

            println!("부모: 종료");
        }

        Ok(ForkResult::Child) => {
            let pid = getpid();

            println!("자식: PID = {pid}");
            println!("자식: 현재 정책 확인 대기");

            thread::sleep(Duration::from_secs(3));

            let policy = unsafe { sched_getscheduler(pid.as_raw()) };

            if policy == -1 {
                eprintln!(
                    "자식: sched_getscheduler 실패: {}",
                    io::Error::last_os_error()
                );
                process::exit(1);
            }

            println!("자식: 현재 정책 = {policy}");

            thread::sleep(Duration::from_secs(2));

            println!("자식: 종료");
        }

        Err(err) => {
            eprintln!("fork 실패: {err}");
            process::exit(1);
        }
    }
}
