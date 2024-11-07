#[derive(Default)]
pub enum Clusters {
    #[default]
    General,
}

impl Clusters {
    pub fn cluster_name_as_str(&self) -> &str {
        match self {
            Clusters::General => "generalcluster",
        }
    }
    pub fn cluster_name(&self) -> String {
        self.cluster_name_as_str().to_string()
    }
    pub fn app_name_as_str(&self) -> &str {
        match self {
            Clusters::General => "GeneralCluster",
        }
    }
    pub fn app_name(&self) -> String {
        self.app_name_as_str().to_string()
    }
    pub fn cluster_url(&self, username: String, password: String) -> String {
        format!(
            "mongodb+srv://{username}:{password}@{cluster_name}.9uvli.mongodb.net/?retryWrites=true&w=majority&appName={app_name}",
            cluster_name = self.cluster_name(),
            app_name = self.app_name()
        )
    }
}
