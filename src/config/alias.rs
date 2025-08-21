use crate::config::config::get_config_folder;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_LOCATION_FILENAME: &str = "alias.json";

pub struct AliasManager {
    location: PathBuf,
}

impl Default for AliasManager {
    fn default() -> Self {
        let mut location = get_config_folder();
        location.push(CONFIG_LOCATION_FILENAME);

        AliasManager::new(location)
    }
}

impl AliasManager {
    fn new(alias_file: PathBuf) -> Self {
        Self {
            location: alias_file,
        }
    }

    fn ensure_file(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.location.parent() {
            fs::create_dir_all(parent)?;
        }

        // Only create file if it doesn't exist
        if !self.location.exists() {
            let empty_file = serde_json::to_string(&HashMap::<String, String>::default())?;
            fs::write(&self.location, empty_file)?;
        }
        Ok(())
    }

    pub(crate) fn load_aliases(&self) -> anyhow::Result<HashMap<String, String>> {
        self.ensure_file()?;
        let file_contents = std::fs::read(&self.location)?;
        let aliases: HashMap<String, String> = serde_json::from_slice(file_contents.as_slice())?;

        Ok(aliases)
    }

    pub(crate) fn get_alias(&self, alias: &str) -> Option<String> {
        let aliases = self.load_aliases().unwrap_or_default();
        aliases.get(alias).map(|s| s.to_string())
    }

    pub(crate) fn add_alias(
        &self,
        alias: String,
        device_url: String,
        overwrite: bool,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut aliases = self.load_aliases()?;

        if !aliases.contains_key(&alias) || overwrite {
            aliases.insert(alias, device_url);
        }

        let _ = self.write_alias_file(&aliases);

        Ok(aliases)
    }

    fn write_alias_file(&self, aliases: &HashMap<String, String>) -> anyhow::Result<()> {
        self.ensure_file()?;
        let json_str = serde_json::to_string(&aliases)?;
        fs::write(&self.location, json_str)?;

        Ok(())
    }

    pub(crate) fn delete_alias(&self, alias: String) -> anyhow::Result<HashMap<String, String>> {
        let mut aliases = self.load_aliases()?;
        aliases.remove(&alias);
        let _ = self.write_alias_file(&aliases);

        Ok(aliases)
    }

    pub(crate) fn resolve_alias(&self, alias: &str) -> String {
        let device_url = self.get_alias(alias);
        device_url.unwrap_or(alias.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_alias_manager() -> (AliasManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let alias_file = temp_dir.path().join("test_alias.json");
        let manager = AliasManager::new(alias_file);
        (manager, temp_dir)
    }

    #[test]
    fn test_ensure_file_creates_file_and_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir
            .path()
            .join("deep")
            .join("nested")
            .join("alias.json");
        let manager = AliasManager::new(nested_path.clone());

        assert!(!nested_path.exists());
        manager.ensure_file().unwrap();

        assert!(nested_path.exists());
        let content = fs::read_to_string(&nested_path).unwrap();
        assert_eq!(content, "{}");
    }

    #[test]
    fn test_ensure_file_does_not_overwrite_existing() {
        let (manager, _temp_dir) = create_test_alias_manager();

        // Create file with content
        let test_data = r#"{"existing":"value"}"#;
        fs::write(&manager.location, test_data).unwrap();

        // Ensure file should not overwrite
        manager.ensure_file().unwrap();

        let content = fs::read_to_string(&manager.location).unwrap();
        assert_eq!(content, test_data);
    }

    #[test]
    fn test_load_aliases_empty_file() {
        let (manager, _temp_dir) = create_test_alias_manager();

        let aliases = manager.load_aliases().unwrap();
        assert!(aliases.is_empty());
    }

    #[test]
    fn test_add_alias() {
        let (manager, _temp_dir) = create_test_alias_manager();

        let result = manager
            .add_alias("test".to_string(), "http://test.com".to_string(), false)
            .unwrap();
        assert_eq!(result.get("test"), Some(&"http://test.com".to_string()));

        // Verify persistence
        let aliases = manager.load_aliases().unwrap();
        assert_eq!(aliases.get("test"), Some(&"http://test.com".to_string()));
    }

    #[test]
    fn test_add_alias_no_overwrite() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias("test".to_string(), "http://original.com".to_string(), false)
            .unwrap();
        let result = manager
            .add_alias("test".to_string(), "http://new.com".to_string(), false)
            .unwrap();

        assert_eq!(result.get("test"), Some(&"http://original.com".to_string()));
    }

    #[test]
    fn test_add_alias_with_overwrite() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias("test".to_string(), "http://original.com".to_string(), false)
            .unwrap();
        let result = manager
            .add_alias("test".to_string(), "http://new.com".to_string(), true)
            .unwrap();

        assert_eq!(result.get("test"), Some(&"http://new.com".to_string()));
    }

    #[test]
    fn test_get_alias_existing() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias("test".to_string(), "http://test.com".to_string(), false)
            .unwrap();
        let result = manager.get_alias("test");

        assert_eq!(result, Some("http://test.com".to_string()));
    }

    #[test]
    fn test_get_alias_nonexistent() {
        let (manager, _temp_dir) = create_test_alias_manager();

        let result = manager.get_alias("nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete_alias() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias("test".to_string(), "http://test.com".to_string(), false)
            .unwrap();
        let result = manager.delete_alias("test".to_string()).unwrap();

        assert!(!result.contains_key("test"));
        assert_eq!(manager.get_alias("test"), None);
    }

    #[test]
    fn test_resolved_device_url_or_self_with_alias() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias("test".to_string(), "http://test.com".to_string(), false)
            .unwrap();
        let result = manager.resolve_alias("test");

        assert_eq!(result, "http://test.com");
    }

    #[test]
    fn test_resolved_device_url_or_self_without_alias() {
        let (manager, _temp_dir) = create_test_alias_manager();

        let result = manager.resolve_alias("nonexistent");
        assert_eq!(result, "nonexistent");
    }

    #[test]
    fn test_multiple_aliases() {
        let (manager, _temp_dir) = create_test_alias_manager();

        manager
            .add_alias(
                "alias1".to_string(),
                "http://device1.com".to_string(),
                false,
            )
            .unwrap();
        manager
            .add_alias(
                "alias2".to_string(),
                "http://device2.com".to_string(),
                false,
            )
            .unwrap();

        let aliases = manager.load_aliases().unwrap();
        assert_eq!(aliases.len(), 2);
        assert_eq!(
            aliases.get("alias1"),
            Some(&"http://device1.com".to_string())
        );
        assert_eq!(
            aliases.get("alias2"),
            Some(&"http://device2.com".to_string())
        );
    }
}
