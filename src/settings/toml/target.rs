use super::kv_namespace::KvNamespace;
use super::site::Site;
use super::target_type::TargetType;

use std::collections::HashMap;
use std::env;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Target {
    pub account_id: String,
    #[serde(rename = "kv-namespaces")]
    pub kv_namespaces: Option<Vec<KvNamespace>>,
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub webpack_config: Option<String>,
    pub site: Option<Site>,
    pub vars: Option<HashMap<String, String>>,
}

impl Target {
    pub fn kv_namespaces(&self) -> Vec<KvNamespace> {
        self.kv_namespaces.clone().unwrap_or_else(Vec::new)
    }

    pub fn add_kv_namespace(&mut self, kv_namespace: KvNamespace) {
        let mut updated_namespaces = self.kv_namespaces();
        updated_namespaces.push(kv_namespace);
        self.kv_namespaces = Some(updated_namespaces);
    }

    pub fn build_dir(&self) -> Result<PathBuf, std::io::Error> {
        // if `site` is configured, we want to isolate worker code
        // and build artifacts away from static site application code.
        match &self.site {
            Some(site_config) => site_config.entry_point(),
            None => {
                let current_dir = env::current_dir()?;
                Ok(current_dir)
            }
        }
    }
}
