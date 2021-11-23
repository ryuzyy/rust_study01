pub mod sub_a;
pub mod sub_b;

// 定数はconst 基本大文字かな？ Elixirと一緒で_つかえそう
const MAX_POINTS: u32 = 100_000;

pub fn run() {
  println!("Here is vars module!!");
  // sub_a::func_a();
  // sub_b::func_b();
  // Rustはイミュータブルだけど、mutをつければ解除できる
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // 型推論　_つけると意図的に使用しないってできる
  let _i1 = 3;
  let _f2 = 0.1;
}
