// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/cilium/cilium/cilium.io/v2alpha1/ciliumbgppeeringpolicies.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec is a human readable description of a BGP peering policy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "cilium.io", version = "v2alpha1", kind = "CiliumBGPPeeringPolicy", plural = "ciliumbgppeeringpolicies")]
#[kube(schema = "disabled")]
pub struct CiliumBGPPeeringPolicySpec {
    /// NodeSelector selects a group of nodes where this BGP Peering Policy applies. 
    ///  If empty / nil this policy applies to all nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<CiliumBGPPeeringPolicyNodeSelector>,
    /// A list of CiliumBGPVirtualRouter(s) which instructs the BGP control plane how to instantiate virtual BGP routers.
    #[serde(rename = "virtualRouters")]
    pub virtual_routers: Vec<CiliumBGPPeeringPolicyVirtualRouters>,
}

/// NodeSelector selects a group of nodes where this BGP Peering Policy applies. 
///  If empty / nil this policy applies to all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeeringPolicyNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeeringPolicyNodeSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyNodeSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// CiliumBGPVirtualRouter defines a discrete BGP virtual router configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRouters {
    /// ExportPodCIDR determines whether to export the Node's private CIDR block to the configured neighbors.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportPodCIDR")]
    pub export_pod_cidr: Option<bool>,
    /// LocalASN is the ASN of this virtual router. Supports extended 32bit ASNs
    #[serde(rename = "localASN")]
    pub local_asn: i64,
    /// Neighbors is a list of neighboring BGP peers for this virtual router
    pub neighbors: Vec<CiliumBGPPeeringPolicyVirtualRoutersNeighbors>,
    /// PodIPPoolSelector selects CiliumPodIPPools based on labels. The virtual router will announce allocated CIDRs of matching CiliumPodIPPools. 
    ///  If empty / nil no CiliumPodIPPools will be announced.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIPPoolSelector")]
    pub pod_ip_pool_selector: Option<CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelector>,
    /// ServiceSelector selects a group of load balancer services which this virtual router will announce. The loadBalancerClass for a service must be nil or specify a class supported by Cilium, e.g. "io.cilium/bgp-control-plane". Refer to the following document for additional details regarding load balancer classes: 
    ///  https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class 
    ///  If empty / nil no services will be announced.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSelector")]
    pub service_selector: Option<CiliumBGPPeeringPolicyVirtualRoutersServiceSelector>,
}

/// CiliumBGPNeighbor is a neighboring peer for use in a CiliumBGPVirtualRouter configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighbors {
    /// AdvertisedPathAttributes can be used to apply additional path attributes to selected routes when advertising them to the peer. If empty / nil, no additional path attributes are advertised.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "advertisedPathAttributes")]
    pub advertised_path_attributes: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributes>>,
    /// AuthSecretRef is the name of the secret to use to fetch a TCP authentication password for this peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSecretRef")]
    pub auth_secret_ref: Option<String>,
    /// ConnectRetryTimeSeconds defines the initial value for the BGP ConnectRetryTimer (RFC 4271, Section 8).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectRetryTimeSeconds")]
    pub connect_retry_time_seconds: Option<i32>,
    /// EBGPMultihopTTL controls the multi-hop feature for eBGP peers. Its value defines the Time To Live (TTL) value used in BGP packets sent to the neighbor. The value 1 implies that eBGP multi-hop feature is disabled (only a single hop is allowed). This field is ignored for iBGP peers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eBGPMultihopTTL")]
    pub e_bgp_multihop_ttl: Option<i32>,
    /// Families, if provided, defines a set of AFI/SAFIs the speaker will negotiate with it's peer. 
    ///  If this slice is not provided the default families of IPv6 and IPv4 will be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamilies>>,
    /// GracefulRestart defines graceful restart parameters which are negotiated with this neighbor. If empty / nil, the graceful restart capability is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gracefulRestart")]
    pub graceful_restart: Option<CiliumBGPPeeringPolicyVirtualRoutersNeighborsGracefulRestart>,
    /// HoldTimeSeconds defines the initial value for the BGP HoldTimer (RFC 4271, Section 4.2). Updating this value will cause a session reset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "holdTimeSeconds")]
    pub hold_time_seconds: Option<i32>,
    /// KeepaliveTimeSeconds defines the initial value for the BGP KeepaliveTimer (RFC 4271, Section 8). It can not be larger than HoldTimeSeconds. Updating this value will cause a session reset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepAliveTimeSeconds")]
    pub keep_alive_time_seconds: Option<i32>,
    /// PeerASN is the ASN of the peer BGP router. Supports extended 32bit ASNs
    #[serde(rename = "peerASN")]
    pub peer_asn: i64,
    /// PeerAddress is the IP address of the peer. This must be in CIDR notation and use a /32 to express a single host.
    #[serde(rename = "peerAddress")]
    pub peer_address: String,
    /// PeerPort is the TCP port of the peer. 1-65535 is the range of valid port numbers that can be specified. If unset, defaults to 179.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerPort")]
    pub peer_port: Option<i32>,
}

/// CiliumBGPPathAttributes can be used to apply additional path attributes to matched routes when advertising them to a BGP peer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributes {
    /// Communities defines a set of community values advertised in the supported BGP Communities path attributes. If nil / not set, no BGP Communities path attribute will be advertised.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communities: Option<CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesCommunities>,
    /// LocalPreference defines the preference value advertised in the BGP Local Preference path attribute. As Local Preference is only valid for iBGP peers, this value will be ignored for eBGP peers (no Local Preference path attribute will be advertised). If nil / not set, the default Local Preference of 100 will be advertised in the Local Preference path attribute for iBGP peers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPreference")]
    pub local_preference: Option<i64>,
    /// Selector selects a group of objects of the SelectorType resulting into routes that will be announced with the configured Attributes. If nil / not set, all objects of the SelectorType are selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelector>,
    /// SelectorType defines the object type on which the Selector applies: - For "PodCIDR" the Selector matches k8s CiliumNode resources (path attributes apply to routes announced for PodCIDRs of selected CiliumNodes. Only affects routes of cluster scope / Kubernetes IPAM CIDRs, not Multi-Pool IPAM CIDRs. - For "CiliumLoadBalancerIPPool" the Selector matches CiliumLoadBalancerIPPool custom resources (path attributes apply to routes announced for selected CiliumLoadBalancerIPPools). - For "CiliumPodIPPool" the Selector matches CiliumPodIPPool custom resources (path attributes apply to routes announced for allocated CIDRs of selected CiliumPodIPPools).
    #[serde(rename = "selectorType")]
    pub selector_type: CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorType,
}

/// Communities defines a set of community values advertised in the supported BGP Communities path attributes. If nil / not set, no BGP Communities path attribute will be advertised.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesCommunities {
    /// Large holds a list of the BGP Large Communities Attribute (RFC 8092) values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<Vec<String>>,
    /// Standard holds a list of "standard" 32-bit BGP Communities Attribute (RFC 1997) values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard: Option<Vec<String>>,
}

/// Selector selects a group of objects of the SelectorType resulting into routes that will be announced with the configured Attributes. If nil / not set, all objects of the SelectorType are selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// CiliumBGPPathAttributes can be used to apply additional path attributes to matched routes when advertising them to a BGP peer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersNeighborsAdvertisedPathAttributesSelectorType {
    #[serde(rename = "PodCIDR")]
    PodCidr,
    #[serde(rename = "CiliumLoadBalancerIPPool")]
    CiliumLoadBalancerIpPool,
    #[serde(rename = "CiliumPodIPPool")]
    CiliumPodIpPool,
}

/// CiliumBGPFamily represents a AFI/SAFI address family pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamilies {
    /// Advertisements selects group of BGP Advertisement(s) to advertise for this family. 
    ///  If not specified, no advertisements are sent for this family. 
    ///  This field is ignored in CiliumBGPNeighbor which is used in CiliumBGPPeeringPolicy. Use CiliumBGPPeeringPolicy advertisement options instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advertisements: Option<CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisements>,
    /// Afi is the Address Family Identifier (AFI) of the family.
    pub afi: CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAfi,
    /// Safi is the Subsequent Address Family Identifier (SAFI) of the family.
    pub safi: CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesSafi,
}

/// Advertisements selects group of BGP Advertisement(s) to advertise for this family. 
///  If not specified, no advertisements are sent for this family. 
///  This field is ignored in CiliumBGPNeighbor which is used in CiliumBGPPeeringPolicy. Use CiliumBGPPeeringPolicy advertisement options instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisements {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisementsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisementsMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisementsMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAdvertisementsMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// CiliumBGPFamily represents a AFI/SAFI address family pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesAfi {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
    #[serde(rename = "l2vpn")]
    L2vpn,
    #[serde(rename = "ls")]
    Ls,
    #[serde(rename = "opaque")]
    Opaque,
}

/// CiliumBGPFamily represents a AFI/SAFI address family pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersNeighborsFamiliesSafi {
    #[serde(rename = "unicast")]
    Unicast,
    #[serde(rename = "multicast")]
    Multicast,
    #[serde(rename = "mpls_label")]
    MplsLabel,
    #[serde(rename = "encapsulation")]
    Encapsulation,
    #[serde(rename = "vpls")]
    Vpls,
    #[serde(rename = "evpn")]
    Evpn,
    #[serde(rename = "ls")]
    Ls,
    #[serde(rename = "sr_policy")]
    SrPolicy,
    #[serde(rename = "mup")]
    Mup,
    #[serde(rename = "mpls_vpn")]
    MplsVpn,
    #[serde(rename = "mpls_vpn_multicast")]
    MplsVpnMulticast,
    #[serde(rename = "route_target_constraints")]
    RouteTargetConstraints,
    #[serde(rename = "flowspec_unicast")]
    FlowspecUnicast,
    #[serde(rename = "flowspec_vpn")]
    FlowspecVpn,
    #[serde(rename = "key_value")]
    KeyValue,
}

/// GracefulRestart defines graceful restart parameters which are negotiated with this neighbor. If empty / nil, the graceful restart capability is disabled.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersNeighborsGracefulRestart {
    /// Enabled flag, when set enables graceful restart capability.
    pub enabled: bool,
    /// RestartTimeSeconds is the estimated time it will take for the BGP session to be re-established with peer after a restart. After this period, peer will remove stale routes. This is described RFC 4724 section 4.2.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restartTimeSeconds")]
    pub restart_time_seconds: Option<i32>,
}

/// PodIPPoolSelector selects CiliumPodIPPools based on labels. The virtual router will announce allocated CIDRs of matching CiliumPodIPPools. 
///  If empty / nil no CiliumPodIPPools will be announced.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersPodIpPoolSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// ServiceSelector selects a group of load balancer services which this virtual router will announce. The loadBalancerClass for a service must be nil or specify a class supported by Cilium, e.g. "io.cilium/bgp-control-plane". Refer to the following document for additional details regarding load balancer classes: 
///  https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class 
///  If empty / nil no services will be announced.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersServiceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeeringPolicyVirtualRoutersServiceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeeringPolicyVirtualRoutersServiceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeeringPolicyVirtualRoutersServiceSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeeringPolicyVirtualRoutersServiceSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

