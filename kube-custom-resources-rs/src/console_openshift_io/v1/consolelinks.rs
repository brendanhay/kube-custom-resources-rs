// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/console.openshift.io/v1/consolelinks.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ConsoleLinkSpec is the desired console link configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "console.openshift.io", version = "v1", kind = "ConsoleLink", plural = "consolelinks")]
#[kube(schema = "disabled")]
pub struct ConsoleLinkSpec {
    /// applicationMenu holds information about section and icon used for the link in the application menu, and it is applicable only when location is set to ApplicationMenu.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationMenu")]
    pub application_menu: Option<ConsoleLinkApplicationMenu>,
    /// href is the absolute secure URL for the link (must use https)
    pub href: String,
    /// location determines which location in the console the link will be appended to (ApplicationMenu, HelpMenu, UserMenu, NamespaceDashboard).
    pub location: String,
    /// namespaceDashboard holds information about namespaces in which the dashboard link should appear, and it is applicable only when location is set to NamespaceDashboard. If not specified, the link will appear in all namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceDashboard")]
    pub namespace_dashboard: Option<ConsoleLinkNamespaceDashboard>,
    /// text is the display text for the link
    pub text: String,
}

/// applicationMenu holds information about section and icon used for the link in the application menu, and it is applicable only when location is set to ApplicationMenu.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleLinkApplicationMenu {
    /// imageUrl is the URL for the icon used in front of the link in the application menu. The URL must be an HTTPS URL or a Data URI. The image should be square and will be shown at 24x24 pixels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageURL")]
    pub image_url: Option<String>,
    /// section is the section of the application menu in which the link should appear. This can be any text that will appear as a subheading in the application menu dropdown. A new section will be created if the text does not match text of an existing section.
    pub section: String,
}

/// namespaceDashboard holds information about namespaces in which the dashboard link should appear, and it is applicable only when location is set to NamespaceDashboard. If not specified, the link will appear in all namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleLinkNamespaceDashboard {
    /// namespaceSelector is used to select the Namespaces that should contain dashboard link by label. If the namespace labels match, dashboard link will be shown for the namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ConsoleLinkNamespaceDashboardNamespaceSelector>,
    /// namespaces is an array of namespace names in which the dashboard link should appear.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
}

/// namespaceSelector is used to select the Namespaces that should contain dashboard link by label. If the namespace labels match, dashboard link will be shown for the namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleLinkNamespaceDashboardNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ConsoleLinkNamespaceDashboardNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleLinkNamespaceDashboardNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
