use reqwest::Error;
use serde_json::Value;

pub struct GetKamino {
    addr: String,
}

impl GetKamino {
    pub fn new(addr: String) -> Self {
        GetKamino { addr }
    }

    pub async fn current_positions(&self) -> Result<String, Error> {
        let market_addr = "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF";
        let url: String = format!(
            "https://api.hubbleprotocol.io/kamino-market/{}/users/{}/obligations",
            market_addr, self.addr
        );
        let req = reqwest::get(&url).await?;
        let data: Value = req.json().await?;

        match data.as_array().and_then(|arr| arr.get(0)) {
            Some(obligation) => match obligation.get("refreshedStats").cloned() {
                Some(refreshed_stats_json) => {
                    if let Some(refreshed_stats) = refreshed_stats_json.as_object() {
                        let user_total_deposit: f64 = refreshed_stats
                            .get("userTotalDeposit")
                            .and_then(Value::as_str)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or_default();
                        let user_total_borrow: f64 = refreshed_stats
                            .get("userTotalBorrow")
                            .and_then(Value::as_str)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or_default();
                        let net_account_value: f64 = refreshed_stats
                            .get("netAccountValue")
                            .and_then(Value::as_str)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or_default();

                        println!("user_total_deposit :: {}", user_total_deposit);
                        println!("user_total_borrow :: {}", user_total_borrow);
                        println!("net_account_value :: {}", net_account_value);
                    }
                }
                None => println!("refreshed stats not found"),
            },
            None => println!("no obligations found"),
        }

        Ok(String::new())
    }

    pub async fn points_s2(&self) -> Result<String, Error> {
        let url: String = format!("https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season2", self.addr);
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

        println!("avg_boost :: {:?}", avg_boost_s2);
        println!("total_points :: {:?}", total_points_s2);
        println!("leaderboard_rank :: {:?}", leaderboard_rank_s2);

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

        println!("total_points :: {:?}", total_points_s1);
        println!("leaderboard_rank :: {:?}", leaderboard_rank_s1);
        Ok(String::new())
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let addr = "DeepCFDi2Whm3TjaVa6Hih6UNAsqr9aex21AhHJjE5TQ";
    let kamino = GetKamino::new(addr.to_string());
    println!("\nKAMINO DATA FOR {}", addr);
    println!("\nCURRENT POSITIONS");
    println!("============================================");
    kamino.current_positions().await?;
    println!("\nS2 POINTS");
    println!("============================================");
    kamino.points_s2().await?;
    println!("\nS1 POINTS");
    println!("============================================");
    kamino.points_s1().await?;
    Ok(())
}
