#[derive(Clone)]
pub enum Bucket {
    Library,
}

impl Bucket {
    pub fn as_str(&self) -> String {
        let str = match self {
            Self::Library => "library",
        };
        str.to_string()
    }

    pub fn is_public(&self) -> bool {
        match self {
            Self::Library => true,
        }
    }
}

pub fn get_all() -> Vec<Bucket> {
    vec![Bucket::Library]
}
