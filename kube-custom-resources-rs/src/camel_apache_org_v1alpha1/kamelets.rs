// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/apache/camel-k/camel.apache.org/v1alpha1/kamelets.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// the desired specification.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "camel.apache.org", version = "v1alpha1", kind = "Kamelet", plural = "kamelets")]
#[kube(namespaced)]
#[kube(status = "KameletStatus")]
pub struct KameletSpec {
    /// data specification types for the events consumed/produced by the Kamelet
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataTypes")]
    pub data_types: Option<BTreeMap<String, KameletDataTypes>>,
    /// defines the formal configuration of the Kamelet
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<KameletDefinition>,
    /// Camel dependencies needed by the Kamelet
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    /// sources in any Camel DSL supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<KameletSources>>,
    /// the main source in YAML DSL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<BTreeMap<String, serde_json::Value>>,
    /// data specification types for the events consumed/produced by the Kamelet Deprecated: In favor of using DataTypes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<BTreeMap<String, KameletTypes>>,
}

/// data specification types for the events consumed/produced by the Kamelet
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypes {
    /// the default data type for this Kamelet
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// one to many header specifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, KameletDataTypesHeaders>>,
    /// one to many data type specifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<BTreeMap<String, KameletDataTypesTypes>>,
}

/// one to many header specifications
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// one to many data type specifications
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesTypes {
    /// the list of Camel or Maven dependencies required by the data type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    /// optional description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the data type format name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// one to many header specifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, KameletDataTypesTypesHeaders>>,
    /// media type as expected for HTTP media types (ie, application/json)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaType")]
    pub media_type: Option<String>,
    /// the expected schema for the data type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<KameletDataTypesTypesSchema>,
    /// the data type component scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// one to many header specifications
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesTypesHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// the expected schema for the data type
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesTypesSchema {
    /// JSONSchemaURL represents a schema url.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    /// ExternalDocumentation allows referencing an external resource for extended documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<KameletDataTypesTypesSchemaExternalDocs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, KameletDataTypesTypesSchemaProperties>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesTypesSchemaExternalDocs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDataTypesTypesSchemaProperties {
    /// default is a default value for undefined object fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enum")]
    pub r#enum: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated: 
    ///  - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 [RFC1034]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?3[0-9a-f]{3}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?4[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?5[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}[- ]?\\d{2}[- ]?\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF" following the regex ^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,255)" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxItems")]
    pub max_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProperties")]
    pub max_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minItems")]
    pub min_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProperties")]
    pub min_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    /// XDescriptors is a list of extended properties that trigger a custom behavior in external systems
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "x-descriptors")]
    pub x_descriptors: Option<Vec<String>>,
}

/// defines the formal configuration of the Kamelet
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDefinition {
    /// JSONSchemaURL represents a schema url.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    /// ExternalDocumentation allows referencing an external resource for extended documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<KameletDefinitionExternalDocs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, KameletDefinitionProperties>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDefinitionExternalDocs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletDefinitionProperties {
    /// default is a default value for undefined object fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enum")]
    pub r#enum: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated: 
    ///  - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 [RFC1034]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?3[0-9a-f]{3}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?4[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?5[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}[- ]?\\d{2}[- ]?\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF" following the regex ^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,255)" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxItems")]
    pub max_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProperties")]
    pub max_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minItems")]
    pub min_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProperties")]
    pub min_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    /// XDescriptors is a list of extended properties that trigger a custom behavior in external systems
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "x-descriptors")]
    pub x_descriptors: Option<Vec<String>>,
}

/// SourceSpec defines the configuration for one or more routes to be executed in a certain Camel DSL language.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletSources {
    /// if the content is compressed (base64 encrypted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<bool>,
    /// the source code (plain text)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// the confimap key holding the source content
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentKey")]
    pub content_key: Option<String>,
    /// the confimap reference holding the source content
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentRef")]
    pub content_ref: Option<String>,
    /// the content type (tipically text or binary)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentType")]
    pub content_type: Option<String>,
    /// True if the spec is generated from a Kamelet
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "from-kamelet")]
    pub from_kamelet: Option<bool>,
    /// Interceptors are optional identifiers the org.apache.camel.k.RoutesLoader uses to pre/post process sources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interceptors: Option<Vec<String>>,
    /// specify which is the language (Camel DSL) used to interpret this source code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Loader is an optional id of the org.apache.camel.k.RoutesLoader that will interpret this source at runtime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader: Option<String>,
    /// the name of the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the path where the file is stored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// List of property names defined in the source (e.g. if type is "template")
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "property-names")]
    pub property_names: Option<Vec<String>>,
    /// the source code (binary)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawContent")]
    pub raw_content: Option<String>,
    /// Type defines the kind of source described by this object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// data specification types for the events consumed/produced by the Kamelet Deprecated: In favor of using DataTypes
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletTypes {
    /// media type as expected for HTTP media types (ie, application/json)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaType")]
    pub media_type: Option<String>,
    /// the expected schema for the event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<KameletTypesSchema>,
}

/// the expected schema for the event
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletTypesSchema {
    /// JSONSchemaURL represents a schema url.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    /// ExternalDocumentation allows referencing an external resource for extended documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<KameletTypesSchemaExternalDocs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, KameletTypesSchemaProperties>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletTypesSchemaExternalDocs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletTypesSchemaProperties {
    /// default is a default value for undefined object fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enum")]
    pub r#enum: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated: 
    ///  - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 [RFC1034]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?3[0-9a-f]{3}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?4[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?5[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}[- ]?\\d{2}[- ]?\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF" following the regex ^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,255)" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxItems")]
    pub max_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProperties")]
    pub max_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minItems")]
    pub min_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProperties")]
    pub min_properties: Option<i64>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    /// A Number represents a JSON number literal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    /// XDescriptors is a list of extended properties that trigger a custom behavior in external systems
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "x-descriptors")]
    pub x_descriptors: Option<Vec<String>>,
}

/// the actual status of the resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletStatus {
    /// Conditions --
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<KameletStatusConditions>>,
    /// ObservedGeneration is the most recent generation observed for this Kamelet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Phase --
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Properties --
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<KameletStatusProperties>>,
}

/// KameletCondition describes the state of a resource at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// The last time this condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// A human-readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of kamelet condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// KameletProperty specify the behavior of a property in a Kamelet.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct KameletStatusProperties {
    /// the default value of the property (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// the name of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

