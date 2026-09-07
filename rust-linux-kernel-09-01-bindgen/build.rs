use std::{env, path::PathBuf};

fn main() {
    // 현재 디렉토리를 컴파일러에 라이브러리 검색 경로로 알립니다
    println!("cargo:rustc-link-search=.");

    // `hello` 라이브러리를 링크합니다
    println!("cargo:rustc-link-lib=hello");

    // `c_src/hello.h` 파일이 변경될 경우 다시 빌드하도록 지시합니다
    println!("cargo:rerun-if-changed=c_src/hello.h");

    // `bindgen`을 사용해 `c_src/hello.h` 헤더 파일에 대한 러스트 바인딩을 생성합니다
    let bindings = bindgen::Builder::default()
        .header("c_src/hello.h") // 바인딩할 헤더 파일 지정
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // Cargo 콜백 등록
        .generate() // 바인딩 생성
        .expect("Unable to generate bindings");

    // 출력 경로를 환경 변수 "OUT_DIR"에서 가져옵니다
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 생성된 바인딩을 `bindings.rs` 파일에 작성합니다
    bindings
        .write_to_file(out_path.join("bindings.rs")) // 자동 생성된 러스트 파일을 작성
        .expect("Couldn't write bindings!");
}
