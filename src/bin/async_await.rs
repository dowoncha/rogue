async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
  async {
    let x: u8 = foo.await();
    x + 5
  }
}

fn main() {
  let result = foo.await();

  println!("{}", result)
}