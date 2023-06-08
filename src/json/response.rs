use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub hits: Vec<Package>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub _index: Option<String>,
    pub _type: Option<String>,
    pub _score: f32,
    pub _source: Source,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub r#type: Option<String>,
    pub package_attr_name: Option<String>,
    pub package_pname: Option<String>,
    pub package_pversion: Option<String>,
    pub package_platforms: Vec<String>,
    pub package_outputs: Vec<String>,
    pub package_default_output: Option<String>,
    pub package_programs: Vec<String>,
    pub package_license: Vec<License>,
    pub package_license_set: Vec<String>,
    pub package_maintainers: Vec<String>,
    pub package_description: Option<String>,
    #[serde(rename = "package_longDescription")]
    pub package_long_description: Option<String>,
    pub package_hydra: Option<String>,
    pub package_system: Option<String>,
    pub package_homepage: Vec<String>,
    pub package_position: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct License {
    pub url: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
}
