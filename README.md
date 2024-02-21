# Little Hyper

![Rust](https://img.shields.io/badge/Rust-DD3515?style=for-the-badge&logo=rust&logoColor=white)

Hyper text transfar protocol written in rust.

## Example

```rs
use little_hyper::{LittleServer, Router};

fn main() {
    let mut router = Router::new();

    router.get("/", |_req, res| {
        res.html("<h1>Hello world kahin hoilm.</h1>");
    });

    router.get("/hello", |_req, res| res.json("hello"));

    router.get("/users/:userId", |req, res| {
        res.json(&format!("userId --> {:?}, {:?}", req.params, req.query));
    });

    let mut server = LittleServer::new(true);

    server.add_router(router);

    println!("Listening on http://127.0.0.1:3000");

    server.listen("127.0.0.1:3000").unwrap();
}
```

## Contributing

Contributions are welcome! I would like you to contribute in this project.

## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](https://github.com/mdmahikaishar/little-hyper/issues) section for a list of features, enhancements, and bug fixes that are planned.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/mdmahikaishar/little-hyper/LICENSE) file for details.
