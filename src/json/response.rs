use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub took: i32,
    pub timed_out: bool,
    pub _shards: Shard,
    pub hits: Hits,
}

#[derive(Deserialize, Debug)]
pub struct Hits {
    pub total: Total,
    pub max_score: Option<f32>,
    pub hits: Vec<Package>,
}

impl Display for Hits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.hits
                .iter()
                .map(|hit| format!("{hit}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Total {
    pub value: i32,
    pub relation: String,
}

#[derive(Deserialize, Debug)]
pub struct Shard {
    pub total: i32,
    pub successful: i32,
    pub skipped: i32,
    pub failed: i32,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub _index: String,
    pub _type: String,
    pub _score: f32,
    pub _source: Source,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self._source)
    }
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub r#type: String,
    pub package_attr_name: String,
    pub package_pname: String,
    pub package_pversion: String,
    pub package_platforms: Vec<String>,
    pub package_outputs: Vec<String>,
    pub package_default_output: String,
    pub package_programs: Vec<String>,
    pub package_license: Vec<License>,
    pub package_license_set: Vec<String>,
    pub package_maintainers: Vec<Maintainer>,
    pub package_description: Option<String>,
    #[serde(rename = "package_longDescription")]
    pub package_long_description: Option<String>,
    pub package_hydra: Option<String>,
    pub package_system: String,
    pub package_homepage: Vec<String>,
    pub package_position: String,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = format!(
            "Info:\t\t{} ({}) [{}]\n",
            self.package_attr_name, self.package_pversion, self.r#type
        );
        let description = self.package_description.is_some().then(|| {
            format!(
                "Description:\t{}\n",
                self.package_description.clone().unwrap_or_default()
            )
        });
        let description_long = self.package_long_description.is_some().then(|| {
            format!(
                "Description:\t{}\n",
                self.package_long_description.clone().unwrap_or_default()
            )
        });
        let platforms = format!(
            "Platforms: [\n{}\n]\n",
            self.package_platforms
                .clone()
                .iter()
                .map(|platform| format!("\t\t{}", platform))
                .collect::<Vec<String>>()
                .join("\n")
        );
        let licenseset = (!self.package_license_set.is_empty())
            .then(|| format!("Licenseset:\t{}\n", self.package_license_set.join(", ")));

        let licenses = (!self.package_license.is_empty()).then(|| {
            format!(
                "Licenses: [\n{}\n]\n",
                self.package_license
                    .clone()
                    .iter()
                    .map(|license| format!("\t\t{license}"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        });

        let maintainers = (!self.package_maintainers.is_empty()).then(|| {
            format!(
                "Maintainers: [\n{}\n]\n",
                self.package_maintainers
                    .clone()
                    .iter()
                    .map(|maintainer| format!("\t\t{maintainer}"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        });

        let homepage = format!("Homepage:\t{}\n", self.package_homepage.join(", "));

        let github = format!(
            "Github:\t\thttps://github.com/NixOS/nixpkgs/blob/master/{}\n",
            self.package_position
                .split(":")
                .collect::<Vec<&str>>()
                .first()
                .unwrap_or(&"/")
        );

        let provides = (!self.package_programs.is_empty()).then(|| {
            format!(
                "Provides: [\n{}\n]\n",
                self.package_programs
                    .clone()
                    .iter()
                    .map(|program| format!("\t\t{program}"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        });

        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}\n\n",
            info,
            description.unwrap_or_default(),
            description_long.unwrap_or_default(),
            provides.unwrap_or_default(),
            platforms,
            licenseset.unwrap_or_default(),
            licenses.unwrap_or_default(),
            maintainers.unwrap_or_default(),
            homepage,
            github,
            (0..40).map(|_| "-").collect::<String>()
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Maintainer {
    pub name: String,
    pub github: String,
    pub email: String,
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.github, self.email)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct License {
    pub url: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.full_name, self.url)
    }
}
