use std::future::Future;

async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
  async {
    let x: u8 = foo().await;
    x + 5
  }
}

async fn async_main() {
  let result = foo();
}

fn main() {
  println!("Hello World");

  async_main();

  println!("After");
}