use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = boundary_api::BoundaryClient::default();
    let res = client.authenticate().await?;

    println!("{:?}", res);
    
    Ok(())
}
