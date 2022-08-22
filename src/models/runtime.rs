use super::ListKey;

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RuntimeId(pub String);

impl RuntimeId {
    pub fn new(id: String) -> Self {
        RuntimeId(id)
    }

    pub fn node(version: &str) -> Self {
        RuntimeId(format!("node-{}", version))
    }

    pub fn php(version: &str) -> Self {
        RuntimeId(format!("php-{}", version))
    }

    pub fn python(version: &str) -> Self {
        RuntimeId(format!("python-{}", version))
    }

    pub fn ruby(version: &str) -> Self {
        RuntimeId(format!("ruby-{}", version))
    }

    pub fn deno(version: &str) -> Self {
        RuntimeId(format!("deno-{}", version))
    }

    pub fn dart(version: &str) -> Self {
        RuntimeId(format!("dart-{}", version))
    }

    pub fn dotnet(version: &str) -> Self {
        RuntimeId(format!("dotnet-{}", version))
    }

    pub fn java(version: &str) -> Self {
        RuntimeId(format!("java-{}", version))
    }

    pub fn swift(version: &str) -> Self {
        RuntimeId(format!("swift-{}", version))
    }

    pub fn kotlin(version: &str) -> Self {
        RuntimeId(format!("kotlin-{}", version))
    }

    pub fn cpp(version: &str) -> Self {
        RuntimeId(format!("cpp-{}", version))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
    #[serde(rename = "$id")]
    pub id: RuntimeId,
    pub name: String,
    pub version: String,
    pub base: String,
    pub image: String,
    pub logo: String,
    pub supports: Vec<String>,
}

impl ListKey for Runtime {
    fn list_key() -> &'static str {
        "runtimes"
    }
}
