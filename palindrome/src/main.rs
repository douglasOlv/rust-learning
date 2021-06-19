
fn is_palindrome(phrase: &str)-> bool{
  let mut phrase = String::from(phrase);
  
  while phrase.len() > 1 {
      let first = phrase.remove(0);
      let last = phrase.pop().unwrap();
      if first != last {return  false}
  }
  true
}

fn main() {
  println!( "{}", is_palindrome("stop pots"));

}

#[test]
fn test_palindrome(){
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("pullup"));
    assert!(is_palindrome("pool loop"));
    assert!(is_palindrome("reward drawer"));
    assert!(is_palindrome("stop pots"));


}