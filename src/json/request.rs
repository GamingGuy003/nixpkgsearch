use serde::Serialize;

#[derive(Serialize)]
pub struct Request {
    pub query: Query,
}

#[derive(Serialize)]
pub struct Query {
    pub bool: Bool,
}

#[derive(Serialize)]
pub struct Bool {
    pub must: Vec<DisMaxElem>,
}

#[derive(Serialize)]
pub struct DisMaxElem {
    pub dis_max: DisMax,
}

#[derive(Serialize)]
pub struct DisMax {
    pub queries: Vec<WildcardElem>,
}

#[derive(Serialize)]
pub struct WildcardElem {
    pub wildcard: Wildcard,
}

#[derive(Serialize)]
pub struct Wildcard {
    pub package_attr_name: PackageAttrName,
}
#[derive(Serialize)]
pub struct PackageAttrName {
    pub value: String,
    pub case_insensitive: bool,
}
