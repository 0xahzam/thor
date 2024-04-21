use reqwest::Error;
use serde_json::Value;

pub struct GetKamino {
    addr: String,
}

impl GetKamino {
    pub fn new(addr: String) -> Self {
        GetKamino { addr }
    }

    pub async fn points_s2(&self) -> Result<String, Error> {
        let url = format!("https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season2", self.addr);
        let req = reqwest::get(&url).await?;
        let data: Value = req.json().await?;

        let avg_boost_s2: f64 = match data.get("avgBoost") {
            Some(value) => match value.as_str() {
                Some(s) => s.parse().unwrap(),
                None => 0.0,
            },
            None => 0.0,
        };

        let total_points_s2: f64 = match data.get("totalPointsEarned") {
            Some(value) => match value.as_str() {
                Some(s) => s.parse().unwrap(),
                None => 0.0,
            },
            None => 0.0,
        };

        let leaderboard_rank_s2 = match data.get("leaderboardRank") {
            Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
            None => 0,
        };

        println!("avg_boost --> {:?}", avg_boost_s2);
        println!("total_points --> {:?}", total_points_s2);
        println!("leaderboard_rank --> {:?}", leaderboard_rank_s2);

        Ok(String::new())
    }

    pub async fn points_s1(&self) -> Result<String, Error> {
        let url = format!("https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season1", self.addr);
        let req = reqwest::get(&url).await?;
        let data: Value = req.json().await?;

        let total_points_s1: f64 = match data.get("totalPointsEarned") {
            Some(value) => match value.as_str() {
                Some(s) => s.parse().unwrap(),
                None => 0.0,
            },
            None => 0.0,
        };

        let leaderboard_rank_s1 = match data.get("leaderboardRank") {
            Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
            None => 0,
        };

        println!("total_points --> {:?}", total_points_s1);
        println!("leaderboard_rank --> {:?}", leaderboard_rank_s1);
        Ok(String::new())
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let addr = "DeepCFDi2Whm3TjaVa6Hih6UNAsqr9aex21AhHJjE5TQ";
    let kamino = GetKamino::new(addr.to_string());
    println!("\nKAMINO DATA FOR {}", addr);
    println!("\nS2 POINTS");
    println!("============================================");
    kamino.points_s2().await?;
    println!("\nS1 POINTS");
    println!("============================================");
    kamino.points_s1().await?;
    Ok(())
}
