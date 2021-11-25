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

  // パターンマッチっぽい処理
  let t1 = (500, 6.4, "Dummy");
  let (_x, _y, _z) = t1;
  println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

  // 基本ポインターの値が入る
  // 先頭に*をつけることで値にアクセスする
  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr = 5;
  *y1_ptr = -5;
  println!("{:?}", t2);
}
