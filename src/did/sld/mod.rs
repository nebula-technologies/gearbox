use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SLD {
    #[serde(rename = "@context")]
    context: Context,
    #[serde(flatten)]
    document: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Context {
    Reference(String),
    Context(ContextData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextData {
    #[serde(flatten)]
    additional_fields: HashMap<String, Context>,

    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    context: Option<Box<Context>>, // Allows nested contexts

    #[serde(rename = "@base", skip_serializing_if = "Option::is_none")]
    base: Option<String>,

    #[serde(rename = "@vocab", skip_serializing_if = "Option::is_none")]
    vocab: Option<String>,

    #[serde(rename = "@language", skip_serializing_if = "Option::is_none")]
    language: Option<String>,

    #[serde(rename = "@protected", skip_serializing_if = "Option::is_none")]
    protected: Option<bool>,

    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    context_type: Option<String>,

    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    graph: Option<Vec<Context>>, // For handling graphs

    #[serde(rename = "@implements", skip_serializing_if = "Option::is_none")]
    implements: Option<Vec<String>>,

    #[serde(rename = "@schema", skip_serializing_if = "Option::is_none")]
    schema: Option<Schema>,

    #[serde(rename = "@container", skip_serializing_if = "Option::is_none")]
    container: Option<ContainerType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContainerType {
    #[serde(rename = "@list")]
    List,
    #[serde(rename = "@set")]
    Set,
    #[serde(rename = "@index")]
    Index,
    #[serde(rename = "@language")]
    Language,
    #[serde(rename = "@id")]
    Id,
    #[serde(rename = "@type")]
    Type,
    #[serde(rename = "@graph")]
    Graph,
    #[serde(rename = "@none")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Schema {
    #[serde(rename = "@versions")]
    Versions(HashMap<String, String>),

    References {
        #[serde(rename = "@ref")]
        reference: Vec<String>,
    },

    Value {
        #[serde(rename = "@item")]
        item: FieldDefinition,
    },
    Array {
        #[serde(rename = "@items")]
        items: FieldDefinition,
    },
    Object {
        #[serde(rename = "@properties")]
        properties: HashMap<String, FieldDefinition>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefinition {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub schema_type: String,
    pub format: String,
    pub pattern: String,
    pub examples: Vec<String>,
}

impl SLD {
    pub fn validate(&self) -> Result<(), String> {
        self.context.validate()
    }
}

impl Context {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Context(c) => c.validate(),
            Self::Reference(s) => Ok(()),
        }
    }
}

impl ContextData {
    pub fn validate(&self) -> Result<(), String> {
        // Validate the @version field
        if let Some(version) = &self.version {
            if version.is_empty() {
                return Err("Invalid context: version cannot be empty".to_string());
            }
            // Ensure version is a valid number
            if version.parse::<f64>().is_err() {
                return Err("Invalid context: version must be a number".to_string());
            }
        }

        // Validate @base as a valid URI
        if let Some(base) = &self.base {
            if !is_valid_uri(base) {
                return Err("Invalid context: base must be a valid URI".to_string());
            }
        }

        // Validate @vocab as a valid URI
        if let Some(vocab) = &self.vocab {
            if !is_valid_uri(vocab) {
                return Err("Invalid context: vocab must be a valid URI".to_string());
            }
        }

        // Validate additional fields as valid URIs or Contexts
        for (key, context) in &self.additional_fields {
            context
                .validate()
                .map_err(|e| format!("Error in field '{}': {}", key, e))?;
        }

        // Validate schema if present
        if let Some(schema) = &self.schema {
            schema.validate()?;
        }

        // Validate nested context
        if let Some(context) = &self.context {
            context.validate()?;
        }

        // No errors found
        Ok(())
    }
}

impl Schema {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Schema::Versions(versions) => {
                for (key, value) in versions {
                    if key.is_empty() || value.is_empty() {
                        return Err(format!(
                            "Invalid schema version: key or value is empty for {}",
                            key
                        ));
                    }
                    // Ensure that the version keys are valid URIs
                    if !is_valid_uri(key) {
                        return Err(format!(
                            "Invalid schema version key '{}': must be a valid URI",
                            key
                        ));
                    }
                }
            }
            Schema::References { reference } => {
                if reference.is_empty() {
                    return Err("Invalid schema reference: reference list is empty".to_string());
                }
                // Ensure that each reference is a valid URI
                for ref_uri in reference {
                    if !is_valid_uri(ref_uri) {
                        return Err(format!(
                            "Invalid schema reference '{}': must be a valid URI",
                            ref_uri
                        ));
                    }
                }
            }
            Schema::Value { item } | Schema::Array { items: item } => {
                item.validate()?;
            }
            Schema::Object { properties } => {
                for (key, field) in properties {
                    if key.is_empty() {
                        return Err("Invalid object property: key is empty".to_string());
                    }
                    // Ensure property keys are valid URIs
                    if !is_valid_uri(key) {
                        return Err(format!(
                            "Invalid object property key '{}': must be a valid URI",
                            key
                        ));
                    }
                    field.validate()?;
                }
            }
        }

        Ok(())
    }
}

impl FieldDefinition {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Field definition error: title cannot be empty".to_string());
        }

        if self.schema_type.is_empty() {
            return Err("Field definition error: type cannot be empty".to_string());
        }

        // Validate format and pattern if provided
        if !self.format.is_empty() && !is_valid_format(&self.format) {
            return Err("Field definition error: invalid format".to_string());
        }

        if !self.pattern.is_empty() && !is_valid_pattern(&self.pattern) {
            return Err("Field definition error: invalid pattern".to_string());
        }

        // Validate that examples match the schema type, format, and pattern
        for example in &self.examples {
            if !self.validate_example(example) {
                return Err(format!(
                    "Field definition error: example '{}' does not match schema constraints",
                    example
                ));
            }
        }

        Ok(())
    }

    fn validate_example(&self, example: &str) -> bool {
        // Example validation logic (simplified for illustration)
        if self.schema_type == "string" {
            if !self.pattern.is_empty() {
                let regex = regex::Regex::new(&self.pattern).unwrap();
                return regex.is_match(example);
            }
        }
        true
    }
}

// Helper functions to validate URIs, formats, and patterns
fn is_valid_uri(uri: &str) -> bool {
    // This is a simplified check. You might want to use a more robust URI validation logic.
    uri.starts_with("http://") || uri.starts_with("https://")
}

fn is_valid_format(format: &str) -> bool {
    // Validate against known formats (e.g., "date", "email", etc.)
    matches!(format, "date" | "email" | "uri" | "uuid")
}

fn is_valid_pattern(pattern: &str) -> bool {
    // This just ensures the pattern is a valid regex
    regex::Regex::new(pattern).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sld_validation() {
        let date_field = FieldDefinition {
            title: "Date".to_string(),
            description: "A string that represents a date in the ISO 8601 format.".to_string(),
            schema_type: "string".to_string(),
            format: "date".to_string(),
            pattern: "^\\d{4}-\\d{2}-\\d{2}$".to_string(),
            examples: vec!["2023-08-26".to_string(), "1999-12-31".to_string()],
        };

        let context = Context::Context(ContextData {
            additional_fields: HashMap::new(),
            context: None,
            base: None,
            vocab: None,
            language: None,
            protected: Some(true),
            version: Some("1.0".to_string()),
            context_type: Some("ExampleType".to_string()),
            graph: None,
            implements: None,
            schema: Some(Schema::Value { item: date_field }),
            container: None,
        });

        let sld = SLD {
            context,
            document: serde_json::json!({ "example": "data" }),
        };

        assert!(sld.validate().is_ok());
    }

    #[test]
    fn test_invalid_sld_validation() {
        let date_field = FieldDefinition {
            title: "".to_string(),
            description: "A string that represents a date in the ISO 8601 format.".to_string(),
            schema_type: "string".to_string(),
            format: "date".to_string(),
            pattern: "^\\d{4}-\\d{2}-\\d{2}$".to_string(),
            examples: vec!["2023-08-26".to_string(), "1999-12-31".to_string()],
        };

        let context = Context::Context(ContextData {
            additional_fields: HashMap::new(),
            context: None,
            base: None,
            vocab: None,
            language: None,
            protected: Some(true),
            version: Some("1.0".to_string()),
            context_type: Some("ExampleType".to_string()),
            graph: None,
            implements: None,
            schema: Some(Schema::Value { item: date_field }),
            container: None,
        });

        let sld = SLD {
            context,
            document: serde_json::json!({ "example": "data" }),
        };

        assert!(sld.validate().is_err());
    }

    #[test]
    fn test_json_ld() {
        let json = r#"{
  "@context": {
    "name": "http://rdf.data-vocabulary.org/#name",
    "ingredient": "http://rdf.data-vocabulary.org/#ingredients",
    "yield": "http://rdf.data-vocabulary.org/#yield",
    "instructions": "http://rdf.data-vocabulary.org/#instructions",
    "step": {
      "@id": "http://rdf.data-vocabulary.org/#step",
      "@type": "xsd:integer"
    },
    "description": "http://rdf.data-vocabulary.org/#description",
    "xsd": "http://www.w3.org/2001/XMLSchema#"
  },
  "name": "Mojito",
  "ingredient": [
    "12 fresh mint leaves",
    "1/2 lime, juiced with pulp",
    "1 tablespoons white sugar",
    "1 cup ice cubes",
    "2 fluid ounces white rum",
    "1/2 cup club soda"
  ],
  "yield": "1 cocktail",
  "instructions": [
    {
      "step": 1,
      "description": "Crush lime juice, mint and sugar together in glass."
    },
    {
      "step": 2,
      "description": "Fill glass to top with ice cubes."
    },
    {
      "step": 3,
      "description": "Pour white rum over ice."
    },
    {
      "step": 4,
      "description": "Fill the rest of glass with club soda, stir."
    },
    {
      "step": 5,
      "description": "Garnish with a lime wedge."
    }
  ]
}"#;

        let sld: SLD = serde_json::from_str(json).unwrap();

        println!("{:#?}", sld);

        let validated = sld.validate();
        println!("{:#?}", validated);

        assert!(validated.is_ok());
    }
}
