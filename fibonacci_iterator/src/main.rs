struct Fibonacci  {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1}
}

fn main() {
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }
    println!("----");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }
    
}
