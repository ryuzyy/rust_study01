pub fn run() {
  // 7MB
  // let a1: [u8; 7000000] = [1; 7000000];
  // 9MB(stack overflow)
  // let a1: [u8; 9000000] = [1; 9000000];

  // ベクター型 (動的に使えるよ)
  let v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  // let mut v3 = vec![9, 10];

  println!("Stack address of v1 is: {:p}", &v1);
  println!("Stack address of v2 is: {:p}", &v2);
  println!("Heap memory address of v1 is: {:p}", &v1.as_ptr());
}
