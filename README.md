# Rust로 백준 풀어보기

이게 뭘까

## 세팅
  
|**Software**|**Version**|
|---|---|
|**OS**|**Ubuntu 24.04.4 LTS**|
|**Rustup**|**1.26.0**|
|**RustC**|**1.81.0**|
|**Cargo**|**1.81.0**|

러스트C랑 카고 둘 다 표시할 필요가 있나???

src에 있는 rs 파일에 코드를 작성한 뒤, f5를 눌러 실행시키면 io/(코드파일명).input.txt 에 있는 내용을 stdin으로 입력받은 뒤, output.txt에 stdout으로 출력한다.

## Rust 입출력

```rust
fn main()
{
    let mut a: String = String::new();
    std::io::stdin().read_line(&mut a).expect("Read failed");
    println!("{}", a)
}
```
