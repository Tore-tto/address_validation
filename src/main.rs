use anyhow::{Context, Result};
use std::str::FromStr;


fn main() {
    
    let addr = parse_beldex_address("bxd5yEEWbj1R5ASz1ik1MtXWSfQ5wgRm99CAkiGidTCG4PAz2TohXUiTCD3rGYMg4f5gtiPvooy8wZJpaA3gSffZ2oPWmGQD5");
    println!("{addr:#?}");
}

fn parse_beldex_address(s: &str) -> Result<beldex::Address> {
    beldex::Address::from_str(s).with_context(|| {
        format!(
            "Failed to parse {} as a beldex address, please make sure it is a valid address",
            s
        )
    })
}

/*Example address
mainaddress
bxd5yEEWbj1R5ASz1ik1MtXWSfQ5wgRm99CAkiGidTCG4PAz2TohXUiTCD3rGYMg4f5gtiPvooy8wZJpaA3gSffZ2oPWmGQD5

integrated address
4H5NnMg5WrJgDFB9QpChA5HqWCSkXsWfqiy4N9EivmBhAUeiCLcUFYPSexUePr7tif28g8SL6kCM67XUsHdQAafzTv5LhqDmqdk2wFVVQT

subaddress
82rRSSY6CHT1s48Bz9Ny7HZX3HrAaurqeQMUNwckZexBRqn4rSR4BA1JZiHtmYteV1YJ1XUqFwuFc1hvReQyT6wFJWmN7tj 

*/