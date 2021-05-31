use yume::Token;

fn main() {
    let yume = Token::new(Token {
        id: "asd".to_string(),
        category: "aasd".to_string(),
        access: vec!["asd".to_string(), "asd1".to_string(), "asd2".to_string()],
        ..Default::default()
    });

    println!("{}", yume.token(1000))
}
