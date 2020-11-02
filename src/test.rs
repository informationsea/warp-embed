use super::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data"]
struct Data;

#[tokio::test]
async fn test_embed_file() {
    let serve = embed(&Data);
    let res = warp::test::request().path("/foo.txt").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.headers().get("content-type").unwrap(), "text/plain");
    assert_eq!(res.body(), "foo");

    let res = warp::test::request().path("/bar.txt").reply(&serve).await;
    assert_eq!(res.status(), 404);

    let res = warp::test::request()
        .path("/bar/hoge.txt")
        .reply(&serve)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.headers().get("content-type").unwrap(), "text/plain");
    assert_eq!(res.body(), "hoge");

    let res = warp::test::request()
        .path("/index.html")
        .reply(&serve)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/index.html"));
    assert_eq!(res.headers().get("content-type").unwrap(), "text/html");

    let res = warp::test::request().path("/").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/index.html"));
    assert_eq!(res.headers().get("content-type").unwrap(), "text/html");

    let res = warp::test::request().path("/bar/").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/bar/index.htm"));
    assert_eq!(res.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn test_embed_file2() {
    let serve = warp::path("hoge").and(embed(&Data));

    let res = warp::test::request().path("/hoge").reply(&serve).await;
    assert_eq!(res.status(), 301);
    assert_eq!(
        res.headers().get("Location").unwrap().to_str().unwrap(),
        "/hoge/"
    );

    let res = warp::test::request().path("/hoge/").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/index.html"));

    let res = warp::test::request().path("/hoge/bar").reply(&serve).await;
    assert_eq!(res.status(), 301);
    assert_eq!(
        res.headers().get("Location").unwrap().to_str().unwrap(),
        "/hoge/bar/"
    );

    let res = warp::test::request().path("/hoge/bar/").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/bar/index.htm"));

    let res = warp::test::request().path("/hoge/hoge").reply(&serve).await;
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn test_embed_one() {
    let serve = embed_one(&Data, "index.html");

    let res = warp::test::request().path("/hoge").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/index.html"));

    let res = warp::test::request().path("/xx").reply(&serve).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), include_str!("../data/index.html"));
}
