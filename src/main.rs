mod kamino;
use kamino::GetKamino;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // User's public key
    let addr = "t8QjVwh8eWYx8osp8JQiUXUHbcvqusM2u8RZyaX6VJC";

    // Create kamino instance
    let kamino = GetKamino::new(addr.to_string());

    println!("\nEXTRACTING KAMINO DATA");
    println!("\nADDRESS :: {}", addr);

    // Retrieve and output current borrowing and lending positions
    println!("\nCURRENT POSITIONS");
    println!("============================================");

    match kamino.current_positions().await {
        Ok(current_positions) => {
            println!("Total Deposit: ${}", current_positions.total_deposit);
            println!("Total Borrow: ${}", current_positions.total_borrow);
            println!(
                "Net Account Value: ${}",
                current_positions.net_account_value
            );
            println!("Current LTV: {}", current_positions.current_ltv);
            println!("Liquidation LTV: {}", current_positions.liquidation_ltv);
            println!("Health Factor : {}%", current_positions.health_factor);
        }
        Err(err) => eprintln!("Error retrieving current positions: {}", err),
    }

    // Retrieve and output season 2 points data
    println!("\nS2 POINTS");
    println!("============================================");

    match kamino.points_s2().await {
        Ok(points_s2) => {
            println!("Avg Boost: {}x", points_s2.avg_boost);
            println!("Total Points: {}", points_s2.total_points);
            println!("Leaderboard Rank: {}", points_s2.leaderboard_rank);
        }
        Err(err) => eprintln!("Error retrieving S2 points: {}", err),
    }

    // Retrieve and output season 1 points data
    println!("\nS1 POINTS");
    println!("============================================");

    match kamino.points_s1().await {
        Ok(points_s1) => {
            println!("Total Points: {}", points_s1.total_points);
            println!("Leaderboard Rank: {}", points_s1.leaderboard_rank);
        }
        Err(err) => eprintln!("Error retrieving S1 points: {}", err),
    }

    Ok(())
}
