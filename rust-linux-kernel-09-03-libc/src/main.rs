use std::{
    mem,
    time::{Duration, UNIX_EPOCH},
};

fn main() {
    let message = c"Hello Rust\n".as_ptr() as *const libc::c_char; // 문자열 끝에 null 종료 문자를 추가하고 C 문자열로 반환합니다

    unsafe {
        libc::printf(message); // libc crate의 printf 함수를 안전하지 않는 블록 내에서 호출합니다
    }

    // libc의 timeval 구조체를 초기화합니다
    // tv_sec와 tv_usec는 초와 마이크로초를 나타냅니다
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    // 현재 시스템 시간을 가져옵니다
    // unsafe 블록 내에서 호출되어야 합니다
    unsafe {
        libc::gettimeofday(&mut tv, mem::zeroed());
    }

    // tv_sec와 tv_usec을 사용해 Duration 객체를 생성합니다
    let duration = Duration::new(tv.tv_sec as u64, tv.tv_usec as u32 * 1000);
    // UNIX_EPOCH(1970-01-01 00:00:00 UTC)로부터의 경과 시간을 계산합니다
    let system_time = UNIX_EPOCH + duration;

    // libc의 tm 구조체를 초기화합니다
    let mut tm = unsafe { mem::zeroed() };
    // 시간을 현지 시간대로 변환합니다
    unsafe {
        libc::localtime_r(&system_time as *const _ as *const libc::time_t, &mut tm);
    }

    // 변환된 현지 시간에서 날짜와 시간 구성 요소를 추출합니다
    let day = tm.tm_mday;
    let mounth = tm.tm_mon + 1;
    let year = tm.tm_year + 1900;
    let hour = tm.tm_hour;
    let min = tm.tm_min;
    let sec = tm.tm_sec;

    // 현지 시간을 출력합니다
    println!("지금은: {year}년 {mounth}월 {day}일 {hour}:{min}:{sec}");
}
