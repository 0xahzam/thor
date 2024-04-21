use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct GetKamino {
    addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPositions {
    pub total_deposit: f64,
    pub total_borrow: f64,
    pub net_account_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointsS2Data {
    pub avg_boost: f64,
    pub total_points: f64,
    pub leaderboard_rank: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointsS1Data {
    pub total_points: f64,
    pub leaderboard_rank: i32,
}

impl GetKamino {
    pub fn new(addr: String) -> Self {
        GetKamino { addr }
    }

    pub async fn current_positions(&self) -> Result<CurrentPositions, Error> {
        let market_addr = "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF";
        let url: String = format!(
            "https://api.hubbleprotocol.io/kamino-market/{}/users/{}/obligations",
            market_addr, self.addr
        );
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        let obligation = match data.as_array().and_then(|arr| arr.get(0)) {
            Some(obligation) => obligation,
            None => {
                println!("No obligations found");
                return Ok(CurrentPositions {
                    total_deposit: 0.0,
                    total_borrow: 0.0,
                    net_account_value: 0.0,
                });
            }
        };

        let refreshed_stats_json = match obligation.get("refreshedStats").cloned() {
            Some(refreshed_stats_json) => refreshed_stats_json,
            None => {
                println!("Refreshed stats not found");
                return Ok(CurrentPositions {
                    total_deposit: 0.0,
                    total_borrow: 0.0,
                    net_account_value: 0.0,
                });
            }
        };

        let user_total_deposit = get_f64_from_json(&refreshed_stats_json, "userTotalDeposit");
        let user_total_borrow = get_f64_from_json(&refreshed_stats_json, "userTotalBorrow");
        let net_account_value = get_f64_from_json(&refreshed_stats_json, "netAccountValue");

        Ok(CurrentPositions {
            total_deposit: user_total_deposit,
            total_borrow: user_total_borrow,
            net_account_value: net_account_value,
        })
    }

    pub async fn points_s2(&self) -> Result<PointsS2Data, Error> {
        let url: String = format!(
            "https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season2",
            self.addr
        );
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        let avg_boost = get_f64_from_json(&data, "avgBoost");
        let total_points = get_f64_from_json(&data, "totalPointsEarned");
        let leaderboard_rank = get_i32_from_json(&data, "leaderboardRank");

        Ok(PointsS2Data {
            avg_boost,
            total_points,
            leaderboard_rank,
        })
    }

    pub async fn points_s1(&self) -> Result<PointsS1Data, Error> {
        let url = format!(
            "https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season1",
            self.addr
        );
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        let total_points = get_f64_from_json(&data, "totalPointsEarned");
        let leaderboard_rank = get_i32_from_json(&data, "leaderboardRank");

        Ok(PointsS1Data {
            total_points,
            leaderboard_rank,
        })
    }
}

fn get_f64_from_json(json: &serde_json::Value, key: &str) -> f64 {
    match json.get(key) {
        Some(value) => match value.as_str() {
            Some(s) => s.parse().unwrap_or_default(),
            None => 0.0,
        },
        None => 0.0,
    }
}

fn get_i32_from_json(json: &serde_json::Value, key: &str) -> i32 {
    match json.get(key) {
        Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
        None => 0,
    }
}
