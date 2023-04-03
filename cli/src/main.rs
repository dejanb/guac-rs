

use guac_rs::{client::GuacClient, vuln::vulns2vex};
use anyhow::*;

use colored_json::prelude::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let purl = "pkg:maven/io.vertx/vertx-web@4.3.7";
    //let purl = "pkg:deb/debian";
    //let purl = "pkg:pypi/django";

    let guac = GuacClient::new("http://localhost:8080/query".to_string());

    //get dependencies
    let deps = guac.get_dependencies(purl).await?;
    let out = serde_json::to_string(&deps)?.to_colored_json_auto()?;
    println!("{}", out);

    //is dependent
    let deps = guac.is_dependent(purl).await?;
    let out = serde_json::to_string(&deps)?.to_colored_json_auto()?;
    println!("{}", out);

    //certify vulns
    let vulns = guac.certify_vuln(purl).await?;
    let vex = vulns2vex(vulns);
    let out = serde_json::to_string(&vex)?.to_colored_json_auto()?;
    println!("{}", out);

    Ok(())
}