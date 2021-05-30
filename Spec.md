> 유메 (yu-me)

AAA.BBB.CCC.DDD

BASE64 <- sig value(JSON & JSON VALUE)

* header
(생성날자기준(UNIX)<<60)+250419136931)

BASE64
* header2
```json
{
    "typ": "yu-me",
    "sig": "sha256",
}
```

* body
```json
{
    "category": "path name",
    "access": ["neko-api"],
    "session" : "user_id(Discord)+TODAY(UNIX)+RANDOM_NUMBER+고정숫자",
}
```

* tail

무의미한 숫자+영어