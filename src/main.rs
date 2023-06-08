use json::request::{Bool, DisMax, PackageAttrName, Query, Wildcard};
use reqwest::header::AUTHORIZATION;

use crate::json::{
    request::{DisMaxElem, Request, WildcardElem},
    response::Response,
};

mod json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let query = Request {
        query: Query {
            bool: Bool {
                must: vec![DisMaxElem {
                    dis_max: DisMax {
                        queries: vec![WildcardElem {
                            wildcard: Wildcard {
                                package_attr_name: PackageAttrName {
                                    value: "*code*".to_owned(),
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

    println!("{:#?}", &serde_json::from_str::<Response>(&response)?);
    Ok(())
}
