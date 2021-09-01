# Rust + Actix Web + Askama Starter

Generate SSL files
```bash
$ `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
```

Develop
```bash
$ cargo watch -x run
```
