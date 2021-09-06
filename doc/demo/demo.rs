fn main () {
  let s = "Hi!";
  let mut cl = s.chars();
  let head = cl.nth(0);
  // head == Some('H')
  println!("{:?}", head);
}