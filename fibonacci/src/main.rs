
fn fibonacci (mut n: i32) -> u128 {
let mut a =0;
let mut b = 1;

while  n > 0 {
    let c = a+b;
    a = b;
    b = c;
    n -= 1;
};
b
}


fn main() {
    assert_eq!(fibonacci(0), 1);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 2);
    assert_eq!(fibonacci(3), 3);
    assert_eq!(fibonacci(4), 5);
    assert_eq!(fibonacci(5), 8);
}
