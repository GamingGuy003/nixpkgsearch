use json::request::{Bool, DisMax, PackageAttrName, Query, Wildcard};
use log::trace;
use reqwest::header::AUTHORIZATION;

use crate::json::{
    request::{DisMaxElem, Request, WildcardElem},
    response::Response,
};

mod json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let args = std::env::args();
    let args_string = args.into_iter().collect::<Vec<String>>();
    let pkgname = args_string.get(1).cloned().unwrap_or_default();
    trace!("Looking for {}", pkgname);
    let query = Request {
        query: Query {
            bool: Bool {
                must: vec![DisMaxElem {
                    dis_max: DisMax {
                        queries: vec![WildcardElem {
                            wildcard: Wildcard {
                                package_attr_name: PackageAttrName {
                                    value: format!("*{}*", pkgname.clone()),
                                    case_insensitive: true,
                                },
                            },
                        }],
                    },
                }],
            },
        },
    };

    let client = reqwest::Client::new();
    let url = "https://search.nixos.org/backend/latest-40-nixos-23.05/_search";

    let response = client
        .post(url)
        .header(
            AUTHORIZATION,
            "Basic YVdWU0FMWHBadjpYOGdQSG56TDUyd0ZFZWt1eHNmUTljU2g=",
        )
        .json(&query)
        .send()
        .await?
        .text()
        .await?;
    trace!("Response: {}", response);
    let hits = &serde_json::from_str::<Response>(&response)?.hits;
    if !hits.hits.is_empty() {
        println!("Matches for {}:\n{}", pkgname, hits);
    } else {
        println!("No packages found for {}", pkgname);
    }
    Ok(())
}
