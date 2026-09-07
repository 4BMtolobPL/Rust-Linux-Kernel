// C와 호환되는 문자열을 다루기 위한 `CString` 타입을 사용합니다
use std::ffi::CString;

// 이전 단계에서 생성된 `bindings.rs` 파일을 포함시키기 위한 모듈을 정의합니다
mod bindings {
    // `OUT_DIR` 환경 변수를 사용해 `bindings.rs` 파일의 위치를 결정하고 포함시킵니다
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    // C와 호환되는 문자열을 생성합니다
    let c_to_print = CString::new("Hello rust").expect("CString::new failed");

    // `bindings` 모듈에서 자동 생성된 `hello` 함수를 안전하지 않은 블록 내에서 호출합니다
    // 이 함수는 원래 C 코드에서 정의됐으며, 그 정의는 `bindings.rs`에서 사용할수 있게 됐습니다
    unsafe {
        bindings::hello(c_to_print.as_ptr());
    }
}
