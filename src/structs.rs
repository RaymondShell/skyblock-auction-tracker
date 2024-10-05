use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Auctions {
    pub totalPages: u32,
    pub page: i32,
    pub auctions: Vec<Auction>,
    pub totalAuctions: u32,

}

#[derive(Serialize, Deserialize)]
pub struct Auction {
    pub uuid: String,
    pub item_name: String,
    pub category: String,
    pub tier: String,
    pub starting_bid: u64,
    pub item_bytes: String,
    pub bin: bool,

}