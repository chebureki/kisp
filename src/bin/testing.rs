extern crate kisp;





/*
fn main() {
  let mut lexer = lexer::Lexer::from_text("\
    (fn nat_ops [n o]
      [
        (if (<= n 0)
          0
          (o n (nat_ops (- n 1)))
        )
      ]
    )
    (nat_ops 100 +)
    ");
  let mut iter = lexer.into_iter();

  let ast = parser::parse(&mut iter).expect("failed ast");
  let result = interpreter::eval(&ast, None);
  match result {
    Ok((data,_)) => {println!("result: {}", data);}
    Err(err) => {println!("error: {:?}", err);}
  }
}*/fn main() {}