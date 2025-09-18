#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Directory {
    name: String,
    path: String
}

impl Directory {
    pub fn new(path: String) -> Option<Self> {
        let path_split = path.split("/");

        if path_split.count() < 1 {
            return None;
        }

        let name = path.split("/")
            .into_iter()
            .last()
            .take()
            .unwrap()
            .to_string();

        return Some(Self { name, path });
    }
}
