
fn main() {
    let test = "()()()()()()()()()()()()()()()()()()()()";
    for _ in 0..100000000 {
        valid_pretheses::sol0(test);
    }
}
