mod kamino;
use kamino::GetKamino;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // user's public key
    let addr = "DeepCFDi2Whm3TjaVa6Hih6UNAsqr9aex21AhHJjE5TQ";

    // new kamino instance
    let kamino = GetKamino::new(addr.to_string());
    println!("\nKAMINO DATA FOR {}", addr);

    // retrieve and output current borrowing and lending positions
    println!("\nCURRENT POSITIONS");
    println!("============================================");
    match kamino.current_positions().await {
        Ok(current_positions) => {
            println!("Total Deposit: {}", current_positions.total_deposit);
            println!("Total Borrow: {}", current_positions.total_borrow);
            println!("Net Account Value: {}", current_positions.net_account_value);
        }
        Err(err) => eprintln!("Error retrieving current positions: {}", err),
    }

    // retrieve and output season 2 points data
    println!("\nS2 POINTS");
    println!("============================================");
    match kamino.points_s2().await {
        Ok(points_s2) => {
            println!("Avg Boost: {}", points_s2.avg_boost);
            println!("Total Points: {}", points_s2.total_points);
            println!("Leaderboard Rank: {}", points_s2.leaderboard_rank);
        }
        Err(err) => eprintln!("Error retrieving S2 points: {}", err),
    }

    // retrieve and output season 1 points data
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
