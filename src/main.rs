mod structs;

use hyper::{Client, Uri};
use tokio;
use std::error::Error;
use hyper_tls::HttpsConnector;
use futures::future::join_all;
use anyhow::{Result, Context}; // Import Result and Context


async fn fetch_auctions(client: &Client<HttpsConnector<hyper::client::HttpConnector>>, page: u32) -> Result<structs::Auctions> {
    let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
    let uri: Uri = url.parse().context("Invalid URI")?;

    let resp = client.get(uri).await.context("Failed to fetch auctions")?;

    if !resp.status().is_success() {
        eprintln!("Failed to fetch data for page {}: {}", page, resp.status());
        return Ok(structs::Auctions { totalPages: 0, page: 0, auctions: vec![] , totalAuctions: 0}); // Return an empty Auctions struct on failure
    }

    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.context("Failed to read response body")?;
    let auctions_data: structs::Auctions = serde_json::from_slice(&body_bytes).context("Failed to parse JSON")?;

    Ok(auctions_data)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create an HTTPS connector
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Define the initial API endpoint to get total pages
    let initial_url = "https://api.hypixel.net/v2/skyblock/auctions?page=0";
    let initial_uri: Uri = initial_url.parse().context("Invalid initial URI")?;

    // Send the GET request to fetch totalPages
    let resp = client.get(initial_uri).await.context("Failed to fetch total pages")?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.context("Failed to read response body")?;

    // Parse the JSON response to get totalPages
    let auctions_response: structs::Auctions = serde_json::from_slice(&body_bytes).context("Failed to parse JSON response")?;
    let total_pages = auctions_response.totalPages;

    println!("Total Pages: {}", total_pages);

    // Create a vector to hold all the tasks
    let mut tasks = Vec::new();

    // Loop through all pages and create tasks
    for page in 0..total_pages {
        let client_clone = client.clone(); // Clone the client for the task
        tasks.push(tokio::spawn(async move {
            fetch_auctions(&client_clone, page).await
        }));
    }

    // Await all tasks concurrently
    let results = join_all(tasks).await;
    let mut totalAuctions = 0;
    // Process the results
    for result in results {
        match result {
            Ok(Ok(auctions_data)) => {
                //println!("Num Auctions Parsed: {:?}", auctions_data.auctions.len());
                totalAuctions += auctions_data.auctions.len();
                for auction in auctions_data.auctions {
                    //println!("Item Name: {}, Starting Bid: {}", auction.item_name, auction.starting_bid);
                }
            }
            Ok(Err(e)) => {
                eprintln!("Error fetching auctions: {}", e);
            }
            Err(e) => {
                eprintln!("Task failed: {}", e);
            }
        }
    }
    println!("Num Auctions Parsed: {:?}", totalAuctions);
    println!("Num Auctions Told: {:?}", auctions_response.totalAuctions);

    Ok(())
}