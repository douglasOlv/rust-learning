pub fn public_function() {
    println!("called first-lib's `public_function()`");
}

fn private_function() {
    println!("called firs-lib's `private_function()`");
}

pub fn indirect_access() {
    print!("called first-lib's `indirect_access()`, that\n> ");

    private_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
