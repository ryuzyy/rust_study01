use rand::Rng;
use std::io;

pub fn func_v01() {
  println!("Guess the number!"); //数を当ててごらん！

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secretnumber is: {}", secret_number);
  println!("Please input your guess!"); //ほら、予想して入力してね

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line"); //行の読み込みに失敗しました

  println!("You guessed: {}", guess); //次のように予想しました。
}
