pub fn try_ownership() {
    let mut text = String::from("Hello World");
    println!("{}", text);

    text = move_and_return_ownership(text);
    println!("{}", text);

    borrowing_ownership(&text);
    println!("{}", text);

    let move_ownership = move || {
      println!("{}", text);
    };
    move_ownership();
    //println!("{}", text); // value borrowed here after move

    let mut count = count_up();
    println!("{}", count());
    println!("{}", count());
    println!("{}", count());
}

fn move_and_return_ownership(text: String) -> String {
  println!("{}", text);
  text
}

fn borrowing_ownership(text: &str) {
  println!("{}", text);
}

fn count_up() -> impl FnMut() -> usize {
  let mut counter: usize = 0;
  move || -> usize {
    counter += 1;
    counter
  }
}
