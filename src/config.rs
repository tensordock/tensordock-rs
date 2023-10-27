/// Contains configuration information for the TensorDock client. The
/// authorization keys and tokens can be retrieved from
/// `https://marketplace.tensordock.com/api`.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// The authorization key uuid.
    pub key: String,
    /// The authorization token uuid.
    pub token: String,
    /// An optional description describing the purpose of this config /
    /// authorization set.
    pub description: Option<String>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

/// Utility struct used in intermediary construction of a `Config` via the
/// builder creational pattern.
#[derive(Default)]
pub struct ConfigBuilder {
    key: Option<String>,
    token: Option<String>,
    description: Option<String>,
}

impl ConfigBuilder {
    /// Produces a new `ConfigBuilder` with the `key` field set.
    pub fn key(self, key: String) -> Self {
        Self {
            key: Some(key),
            ..self
        }
    }

    /// Produces a new `ConfigBuilder` with the `token` field set.
    pub fn token(self, token: String) -> Self {
        Self {
            token: Some(token),
            ..self
        }
    }

    /// Produces a new `ConfigBuilder` with the `description` field set.
    pub fn description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    /// Build the `ConfigBuilder`, producing a `Config`, and assigning empty string
    /// values to the `key` and `token` values if not set.
    pub fn build(self) -> Config {
        Config {
            key: self.key.unwrap_or_else(|| String::from("")),
            token: self.token.unwrap_or_else(|| String::from("")),
            description: self.description,
        }
    }
}

#[test]
fn config_builder_test() {
    let expected = Config {
        key: String::from("abcd"),
        token: String::from("efgh"),
        description: None,
    };
    let result = Config::builder()
        .key(String::from("abcd"))
        .token(String::from("efgh"))
        .build();

    assert_eq!(expected, result);
}
