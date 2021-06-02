
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
    let var= fibonacci(2);
    println!("{}", var );
}
