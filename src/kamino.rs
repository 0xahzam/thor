use reqwest::Error;
use serde::{Deserialize, Serialize};

// Struct to interact with Kamino data
pub struct GetKamino {
    addr: String, // Address of the user
}

// Struct representing current positions
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPositions {
    pub total_deposit: f64,     // User's Total deposited amount
    pub total_borrow: f64,      // User' Total borrowed amount
    pub net_account_value: f64, // Net account value
    pub current_ltv: f64,       // Current Loan to Value
    pub liquidation_ltv: f64,   // Liquidaiton LTV
    pub health_factor: f64,     // Health factor of the position (%)
}

// Struct representing Season 2 points data
#[derive(Debug, Serialize, Deserialize)]
pub struct PointsS2Data {
    pub avg_boost: f64,        // Average points boost (eg. 5x)
    pub total_points: f64,     // Total points earned
    pub leaderboard_rank: i32, // Leaderboard rank
}

// Struct representing Season 1 points data
#[derive(Debug, Serialize, Deserialize)]
pub struct PointsS1Data {
    pub total_points: f64,     // Total points earned
    pub leaderboard_rank: i32, // Leaderboard rank
}

impl GetKamino {
    // Constructor to create a new instance of GetKamino
    pub fn new(addr: String) -> Self {
        GetKamino { addr }
    }

    // function to retrieve current borrowing and lending positinos
    pub async fn current_positions(&self) -> Result<CurrentPositions, Error> {
        // Construct URL for retrieving current positions
        let market_addr = "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"; // Default (Main Liquidity Pool)
        let url: String = format!(
            "https://api.hubbleprotocol.io/kamino-market/{}/users/{}/obligations",
            market_addr, self.addr
        );

        // Send request
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        // Extract user obligation data
        let obligation = match data.as_array().and_then(|arr| arr.get(0)) {
            Some(obligation) => obligation,
            None => {
                println!("No obligations found");
                return Ok(CurrentPositions {
                    total_deposit: 0.0,
                    total_borrow: 0.0,
                    net_account_value: 0.0,
                    current_ltv: 0.0,
                    liquidation_ltv: 0.0,
                    health_factor: 0.0,
                });
            }
        };

        // Extract refreshed stats
        let refreshed_stats_json = match obligation.get("refreshedStats").cloned() {
            Some(refreshed_stats_json) => refreshed_stats_json,
            None => {
                println!("Refreshed stats not found");
                return Ok(CurrentPositions {
                    total_deposit: 0.0,
                    total_borrow: 0.0,
                    net_account_value: 0.0,
                    current_ltv: 0.0,
                    liquidation_ltv: 0.0,
                    health_factor: 0.0,
                });
            }
        };

        // Parse values from JSON
        let total_deposit = get_f64_from_json(&refreshed_stats_json, "userTotalDeposit");
        let total_borrow = get_f64_from_json(&refreshed_stats_json, "userTotalBorrow");
        let net_account_value = get_f64_from_json(&refreshed_stats_json, "netAccountValue");

        // Risk adjust debt value
        let user_total_borrow_borrow_factor_adjusted =
            get_f64_from_json(&refreshed_stats_json, "userTotalBorrowBorrowFactorAdjusted");

        // Represents the threshold at which the position becomes eligible for liquidation if borrowing exceeds this limit
        let borrow_liquidation_limit =
            get_f64_from_json(&refreshed_stats_json, "borrowLiquidationLimit");

        // Current Loan to Value
        let current_ltv = user_total_borrow_borrow_factor_adjusted / total_deposit;

        // Liquidation LTV
        let liquidation_ltv = borrow_liquidation_limit / total_deposit;

        // Calculating position's health score
        let health_factor = (liquidation_ltv - current_ltv) / liquidation_ltv * 100.0;

        // Return current positions data
        Ok(CurrentPositions {
            total_deposit,
            total_borrow,
            net_account_value,
            current_ltv,
            liquidation_ltv,
            health_factor,
        })
    }

    // function to retrieve Season 2 points data
    pub async fn points_s2(&self) -> Result<PointsS2Data, Error> {
        let url: String = format!(
            "https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season2",
            self.addr
        );

        // Send request
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        // Parse values from JSON
        let avg_boost = get_f64_from_json(&data, "avgBoost");
        let total_points = get_f64_from_json(&data, "totalPointsEarned");
        let leaderboard_rank = get_i32_from_json(&data, "leaderboardRank");

        // Return Season 2 points data
        Ok(PointsS2Data {
            avg_boost,
            total_points,
            leaderboard_rank,
        })
    }

    // Async function to retrieve Season 1 points data
    pub async fn points_s1(&self) -> Result<PointsS1Data, Error> {
        let url = format!(
            "https://api.hubbleprotocol.io/points/users/{}/breakdown?env=mainnet-beta&source=Season1",
            self.addr
        );

        // Send request
        let req = reqwest::get(&url).await?;
        let data: serde_json::Value = req.json().await?;

        // Parse values from JSON
        let total_points = get_f64_from_json(&data, "totalPointsEarned");
        let leaderboard_rank = get_i32_from_json(&data, "leaderboardRank");

        // Return Season 1 points data
        Ok(PointsS1Data {
            total_points,
            leaderboard_rank,
        })
    }
}

// Helper function to extract f64 value from JSON
fn get_f64_from_json(json: &serde_json::Value, key: &str) -> f64 {
    match json.get(key) {
        Some(value) => match value.as_str() {
            Some(s) => s.parse().unwrap_or_default(),
            None => 0.0,
        },
        None => 0.0,
    }
}

// Helper function to extract i32 value from JSON
fn get_i32_from_json(json: &serde_json::Value, key: &str) -> i32 {
    match json.get(key) {
        Some(value) => serde_json::from_value(value.clone()).unwrap_or_default(),
        None => 0,
    }
}
