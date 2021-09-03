fn main () {
  let s = "Hi!";
  let mut cl = s.chars();
  let head = cl.nth(1);
  // head == Some('i')
  println!("{:?}", head);
}