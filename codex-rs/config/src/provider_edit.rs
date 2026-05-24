use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use toml::Value as TomlValue;
use toml_edit::DocumentMut;
use toml_edit::Item as TomlItem;
use toml_edit::Table as TomlTable;
use toml_edit::Value as TomlEditValue;
use toml_edit::value;

use crate::CONFIG_TOML_FILE;
use codex_model_provider_info::ModelProviderInfo;

/// Add or replace a model provider in the user's config.toml.
pub fn add_model_provider(
    codex_home: &Path,
    provider_id: &str,
    info: &ModelProviderInfo,
) -> std::io::Result<()> {
    let config_path = codex_home.join(CONFIG_TOML_FILE);
    let mut doc = read_or_create_document(&config_path)?;
    upsert_model_provider(&mut doc, provider_id, info);
    fs::create_dir_all(codex_home)?;
    fs::write(config_path, doc.to_string())
}

/// Remove a model provider from the user's config.toml.
pub fn remove_model_provider(codex_home: &Path, provider_id: &str) -> std::io::Result<bool> {
    let config_path = codex_home.join(CONFIG_TOML_FILE);
    let mut doc = match fs::read_to_string(&config_path) {
        Ok(raw) => raw
            .parse::<DocumentMut>()
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };

    let removed = remove_model_provider_from_doc(&mut doc, provider_id);
    if removed {
        fs::create_dir_all(codex_home)?;
        fs::write(config_path, doc.to_string())?;
    }
    Ok(removed)
}

/// Load user-defined model providers from config.toml.
pub fn load_model_providers(
    codex_home: &Path,
) -> std::io::Result<HashMap<String, ModelProviderInfo>> {
    let config_path = codex_home.join(CONFIG_TOML_FILE);
    let raw = match fs::read_to_string(&config_path) {
        Ok(raw) => raw,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(err) => return Err(err),
    };
    let parsed = toml::from_str::<TomlValue>(&raw)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))?;
    let Some(providers_value) = parsed.get("model_providers") else {
        return Ok(HashMap::new());
    };
    toml::from_value::<HashMap<String, ModelProviderInfo>>(providers_value.clone())
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err))
}

fn read_or_create_document(config_path: &Path) -> std::io::Result<DocumentMut> {
    match fs::read_to_string(config_path) {
        Ok(raw) => raw
            .parse::<DocumentMut>()
            .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(DocumentMut::new()),
        Err(err) => Err(err),
    }
}

fn upsert_model_provider(doc: &mut DocumentMut, provider_id: &str, info: &ModelProviderInfo) {
    let root = doc.as_table_mut();
    if !root.contains_key("model_providers") {
        root.insert("model_providers", TomlItem::Table(new_implicit_table()));
    }

    let Some(providers_item) = root.get_mut("model_providers") else {
        return;
    };
    if !providers_item.is_table() {
        *providers_item = TomlItem::Table(new_implicit_table());
    }

    let Some(providers) = providers_item.as_table_mut() else {
        return;
    };

    let entry = serialize_model_provider(info);
    providers.insert(provider_id, entry);
}

fn remove_model_provider_from_doc(doc: &mut DocumentMut, provider_id: &str) -> bool {
    let root = doc.as_table_mut();
    let Some(providers_item) = root.get_mut("model_providers") else {
        return false;
    };

    let mut remove_providers = false;
    let found = match providers_item {
        TomlItem::Table(providers) => {
            let found = providers.remove(provider_id).is_some();
            remove_providers = providers.is_empty();
            found
        }
        TomlItem::Value(value) => {
            let Some(inline) = value.as_inline_table_mut() else {
                return false;
            };
            let found = inline.remove(provider_id).is_some();
            remove_providers = inline.is_empty();
            found
        }
        _ => false,
    };

    if found && remove_providers {
        root.remove("model_providers");
    }
    found
}

fn serialize_model_provider(info: &ModelProviderInfo) -> TomlItem {
    let mut entry = TomlTable::new();
    entry.set_implicit(false);

    if !info.name.is_empty() {
        entry["name"] = value(info.name.clone());
    }
    if let Some(base_url) = &info.base_url {
        entry["base_url"] = value(base_url.clone());
    }
    if let Some(env_key) = &info.env_key {
        entry["env_key"] = value(env_key.clone());
    }
    if let Some(instructions) = &info.env_key_instructions {
        entry["env_key_instructions"] = value(instructions.clone());
    }
    if let Some(token) = &info.experimental_bearer_token {
        entry["experimental_bearer_token"] = value(token.clone());
    }
    if let Some(auth) = &info.auth {
        entry["auth"] = toml_item_from_serialize(auth);
    }
    if let Some(aws) = &info.aws {
        entry["aws"] = toml_item_from_serialize(aws);
    }
    if !matches!(info.wire_api, codex_model_provider_info::WireApi::Responses) {
        let wire_api_str = match info.wire_api {
            codex_model_provider_info::WireApi::ChatCompletions => "chat_completions",
            _ => "responses",
        };
        entry["wire_api"] = value(wire_api_str);
    }
    if let Some(query_params) = &info.query_params {
        entry["query_params"] = toml_item_from_serialize(query_params);
    }
    if let Some(http_headers) = &info.http_headers {
        entry["http_headers"] = toml_item_from_serialize(http_headers);
    }
    if let Some(env_http_headers) = &info.env_http_headers {
        entry["env_http_headers"] = toml_item_from_serialize(env_http_headers);
    }
    if let Some(retries) = info.request_max_retries {
        entry["request_max_retries"] = value(i64::try_from(retries).unwrap_or_default());
    }
    if let Some(retries) = info.stream_max_retries {
        entry["stream_max_retries"] = value(i64::try_from(retries).unwrap_or_default());
    }
    if let Some(timeout) = info.stream_idle_timeout_ms {
        entry["stream_idle_timeout_ms"] = value(i64::try_from(timeout).unwrap_or_default());
    }
    if let Some(timeout) = info.websocket_connect_timeout_ms {
        entry["websocket_connect_timeout_ms"] =
            value(i64::try_from(timeout).unwrap_or_default());
    }
    if info.requires_openai_auth {
        entry["requires_openai_auth"] = value(true);
    }
    if info.supports_websockets {
        entry["supports_websockets"] = value(true);
    }

    TomlItem::Table(entry)
}

fn toml_item_from_serialize<T: serde::Serialize>(value: &T) -> TomlItem {
    match toml::Value::try_from(value) {
        Ok(toml_value) => toml_item_from_toml_value(toml_value),
        Err(_err) => {
            // Serialization should not fail for the types we store in
            // ModelProviderInfo; if it somehow does, we fall back to an
            // empty string to avoid crashing the CLI.
            TomlItem::Value(TomlEditValue::String("".into()))
        }
    }
}

fn toml_item_from_toml_value(value: TomlValue) -> TomlItem {
    match value {
        TomlValue::String(s) => TomlItem::Value(TomlEditValue::from(s)),
        TomlValue::Integer(i) => TomlItem::Value(TomlEditValue::from(i)),
        TomlValue::Float(f) => TomlItem::Value(TomlEditValue::from(f)),
        TomlValue::Boolean(b) => TomlItem::Value(TomlEditValue::from(b)),
        TomlValue::Datetime(d) => TomlItem::Value(TomlEditValue::from(d)),
        TomlValue::Array(arr) => {
            let mut new_arr = toml_edit::Array::new();
            for v in arr {
                new_arr.push(toml_edit_value_from_toml_value(v));
            }
            TomlItem::Value(TomlEditValue::Array(new_arr))
        }
        TomlValue::Table(table) => {
            let mut new_table = TomlTable::new();
            new_table.set_implicit(true);
            for (k, v) in table {
                new_table.insert(&k, toml_item_from_toml_value(v));
            }
            TomlItem::Table(new_table)
        }
    }
}

fn toml_edit_value_from_toml_value(value: TomlValue) -> TomlEditValue {
    match value {
        TomlValue::String(s) => TomlEditValue::from(s),
        TomlValue::Integer(i) => TomlEditValue::from(i),
        TomlValue::Float(f) => TomlEditValue::from(f),
        TomlValue::Boolean(b) => TomlEditValue::from(b),
        TomlValue::Datetime(d) => TomlEditValue::from(d),
        TomlValue::Array(arr) => {
            let mut new_arr = toml_edit::Array::new();
            for v in arr {
                new_arr.push(toml_edit_value_from_toml_value(v));
            }
            TomlEditValue::Array(new_arr)
        }
        TomlValue::Table(table) => {
            let mut inline = toml_edit::InlineTable::new();
            for (k, v) in table {
                inline.insert(&k, toml_edit_value_from_toml_value(v));
            }
            TomlEditValue::InlineTable(inline)
        }
    }
}

fn new_implicit_table() -> TomlTable {
    let mut table = TomlTable::new();
    table.set_implicit(true);
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_model_provider_info::WireApi;
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    #[test]
    fn add_and_remove_model_provider() {
        let codex_home = TempDir::new().unwrap();

        let info = ModelProviderInfo {
            name: "My Provider".into(),
            base_url: Some("http://localhost:1234/v1".into()),
            env_key: Some("MY_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        };

        add_model_provider(codex_home.path(), "my-provider", &info).unwrap();

        let providers = load_model_providers(codex_home.path()).unwrap();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers["my-provider"].name, "My Provider");
        assert_eq!(
            providers["my-provider"].base_url,
            Some("http://localhost:1234/v1".into())
        );
        assert_eq!(providers["my-provider"].env_key, Some("MY_API_KEY".into()));

        let removed = remove_model_provider(codex_home.path(), "my-provider").unwrap();
        assert!(removed);

        let providers = load_model_providers(codex_home.path()).unwrap();
        assert!(providers.is_empty());
    }

    #[test]
    fn remove_model_provider_not_found() {
        let codex_home = TempDir::new().unwrap();
        let removed = remove_model_provider(codex_home.path(), "missing").unwrap();
        assert!(!removed);
    }

    #[test]
    fn model_provider_with_nested_fields_roundtrips() {
        let codex_home = TempDir::new().unwrap();

        let mut info = ModelProviderInfo {
            name: "OpenRouter".into(),
            base_url: Some("https://openrouter.ai/api/v1".into()),
            env_key: Some("OPENROUTER_API_KEY".into()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        };
        info.http_headers = Some([("X-Custom".into(), "value".into())].into_iter().collect());

        add_model_provider(codex_home.path(), "openrouter", &info).unwrap();

        let providers = load_model_providers(codex_home.path()).unwrap();
        let provider = &providers["openrouter"];
        assert_eq!(provider.name, "OpenRouter");
        assert_eq!(
            provider.base_url,
            Some("https://openrouter.ai/api/v1".into())
        );
        assert_eq!(
            provider.http_headers.as_ref().unwrap()["X-Custom"],
            "value"
        );
    }
}
