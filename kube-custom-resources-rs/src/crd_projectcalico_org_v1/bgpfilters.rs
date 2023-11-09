// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/bgpfilters.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// BGPFilterSpec contains the IPv4 and IPv6 filter rules of the BGP Filter.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "BGPFilter", plural = "bgpfilters")]
#[kube(schema = "disabled")]
pub struct BGPFilterSpec {
    /// The ordered set of IPv4 BGPFilter rules acting on exporting routes to a peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportV4")]
    pub export_v4: Option<Vec<BGPFilterExportV4>>,
    /// The ordered set of IPv6 BGPFilter rules acting on exporting routes to a peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportV6")]
    pub export_v6: Option<Vec<BGPFilterExportV6>>,
    /// The ordered set of IPv4 BGPFilter rules acting on importing routes from a peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "importV4")]
    pub import_v4: Option<Vec<BGPFilterImportV4>>,
    /// The ordered set of IPv6 BGPFilter rules acting on importing routes from a peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "importV6")]
    pub import_v6: Option<Vec<BGPFilterImportV6>>,
}

/// BGPFilterRuleV4 defines a BGP filter rule consisting a single IPv4 CIDR block and a filter action for this CIDR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPFilterExportV4 {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchOperator")]
    pub match_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// BGPFilterRuleV6 defines a BGP filter rule consisting a single IPv6 CIDR block and a filter action for this CIDR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPFilterExportV6 {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchOperator")]
    pub match_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// BGPFilterRuleV4 defines a BGP filter rule consisting a single IPv4 CIDR block and a filter action for this CIDR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPFilterImportV4 {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchOperator")]
    pub match_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// BGPFilterRuleV6 defines a BGP filter rule consisting a single IPv6 CIDR block and a filter action for this CIDR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPFilterImportV6 {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchOperator")]
    pub match_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

