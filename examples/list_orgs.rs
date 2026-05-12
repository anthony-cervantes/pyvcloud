use pyvcloud::{Client, Query, SortDirection};

fn main() -> pyvcloud::Result<()> {
    let base_url =
        std::env::var("VCD_URL").expect("VCD_URL must point at a VMware Cloud Director endpoint");
    let token = std::env::var("VCD_TOKEN").expect("VCD_TOKEN must contain a bearer token");
    let client = Client::builder(base_url)?.bearer_token(token).build()?;
    let orgs = Query::new("org")
        .sort("name", SortDirection::Asc)
        .execute_xml(&client)?;
    println!("{}", orgs.body);
    Ok(())
}
