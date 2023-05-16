mod requester;
use requester::Request;

#[tokio::main]
async fn main() {
    let tobias: Request = Request::new("http://127.0.0.1:5000", "Tobias");
    let null: Request = Request::new("http://127.0.0.1:5000", "");

    println!("POST TOBIAS: {}", tobias.post().await.expect("Could not fetch text from request").trim());
    println!("POST NULL:   {}", null.post().await.expect("Could not fetch text from request").trim());

    println!("GET TOBIAS: {}", tobias.get().await.expect("Could not fetch text from request").trim());
    println!("GET NULL:   {}", null.get().await.expect("Could not fetch text from request").trim());
}
