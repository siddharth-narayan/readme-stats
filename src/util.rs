use serde::Deserialize;
#[derive(Deserialize)]
pub struct SharedParams {
    pub theme: Option<Theme>
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    GithubDark,
    #[default]
    GithubLight
}

impl Theme {
    pub fn to_str(self) -> &'static str {
        match self {
            Theme::GithubLight => "github-light",
            Theme::GithubDark => "github-dark"
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GraphQLNodes<T> {
    pub nodes: Vec<T>
}