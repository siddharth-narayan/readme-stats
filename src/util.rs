use serde::Deserialize;
use typst::foundations::Dict;
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

use crate::world::World;
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

pub fn compile_svg(file_name: impl AsRef<str>, inputs: Dict) -> Result<String, ()> {
    let world = World::new(file_name, inputs);

    // This unwrap needs to go
    let document: PagedDocument = typst::compile(&world).output.map_err(|_| ())?;

    Ok(typst_svg::svg(&document.pages()[0], &SvgOptions::default()))
}