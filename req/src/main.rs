mod request;
mod logger;

use request::requester;
use logger::complex::Logger;

#[tokio::main]
async fn main() {
    let mut logger_tobias = Logger::new();
    logger_tobias.set_debug(Some(true));

    let mut logger_null = Logger::new();
    logger_null.set_debug(Some(false));

    let mut tobias: requester::Request = requester::Request::new("http://127.0.0.1:5000", Some(logger_tobias), "Tobias");
    let mut null: requester::Request = requester::Request::new("http://127.0.0.1:5000", Some(logger_null), "");

    println!("POST TOBIAS:      {}", tobias.post().await.expect("Could not fetch text from request").trim());
    println!("POST NULL:        {}", null.post().await.expect("Could not fetch text from request").trim());

    println!("GET TOBIAS:       {}", tobias.get().await.expect("Could not fetch text from request").trim());
    println!("GET NULL:         {}", null.get().await.expect("Could not fetch text from request").trim());

    tobias.use_curl();
    null.use_curl();

    println!("CURL POST TOBIAS: {}", tobias.post().await.expect("Could not fetch text from request").trim());
    println!("CURL POST NULL:   {}", null.post().await.expect("Could not fetch text from request").trim());

    println!("CURL GET TOBIAS:  {}", tobias.get().await.expect("Could not fetch text from request").trim());
    println!("CURL GET NULL:    {}", null.get().await.expect("Could not fetch text from request").trim());
}
