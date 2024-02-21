use little_hyper::{LittleServer, Router};

fn main() {
    let mut router = Router::new();

    router.get("/", |_req, res| {
        res.html("<h1>Hello world kahin hoilm.</h1>");
    });

    router.get("/hello", |_req, res| res.json("hello"));

    router.get("/users/:userId/:messageId", |req, res| {
        res.json(&format!("userId --> {:?}, {:?}", req.params, req.query));
    });

    let mut server = LittleServer::new(true);

    server.add_router(router);

    println!("Listening on http://127.0.0.1:3000");

    server.listen("127.0.0.1:3000").unwrap();
}
