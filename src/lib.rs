use serde_json::json;

pub struct Token {
    pub typ: String,
    pub sig: String,
    pub id: String,
    pub category: String,
    pub access: Vec<String>,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            typ: "yu-me".to_string(),
            sig: "sha256".to_string(),
            id: "1".to_string(),
            category: "neko".to_string(),
            access: vec!["neko-neko".to_string()],
        }
    }
}

impl Token {
    pub fn new(self: Token) -> Self {
        Self {
            typ: self.typ,
            sig: self.sig,
            id: self.id,
            category: self.category,
            access: self.access,
        }
    }
    pub fn token(&self, number: usize) -> String {
        let header1 =
            base64::encode(((chrono::Utc::now().timestamp() << 60) + 250419136931).to_string());
        let header2 = self.header2();
        let body = self.body(Token::session(&self.id, &header1, &number.to_string()));
        let tail = sha256::digest(rand::random::<usize>().to_string());

        format!("{}.{}.{}.{}", header1, header2, body, tail)
    }

    pub fn header2(&self) -> String {
        let header2_json = json!({
            "typ": self.typ,
            "sig": self.sig
        });

        base64::encode(header2_json.to_string())
    }

    pub fn session(id: &str, today: &str, number: &str) -> String {
        base64::encode(format!(
            "{}{}{}{}",
            id,
            today,
            rand::random::<usize>(),
            number
        ))
    }

    pub fn body(&self, session: String) -> String {
        if self.sig != "sha256" {
            panic!("")
        }

        let access = self
            .access
            .iter()
            .map(|f| sha256::digest(f.to_string()))
            .collect::<Vec<_>>();

        let body_json = json!({
            "category": sha256::digest(&self.category),
            "access": access,
            "session": sha256::digest(session)
        });

        let sha256 = sha256::digest(body_json.to_string());

        base64::encode(sha256)
    }
}
