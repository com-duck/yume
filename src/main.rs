use yume::Token;

fn main() {
    let yume = Token::new(Token {
        id: "asd",
        category: "aasd",
        access: vec!["asd", "asd1", "asd2"],
        ..Default::default()
    });

    println!("{}", yume.token(1000))
}
