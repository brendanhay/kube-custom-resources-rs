// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apache/apisix-ingress-controller/apisix.apache.org/v2/apisixroutes.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apisix.apache.org", version = "v2", kind = "ApisixRoute", plural = "apisixroutes")]
#[kube(namespaced)]
#[kube(status = "ApisixRouteStatus")]
#[kube(schema = "disabled")]
pub struct ApisixRouteSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<ApisixRouteHttp>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<Vec<ApisixRouteStream>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<ApisixRouteHttpAuthentication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<ApisixRouteHttpBackends>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<ApisixRouteHttpMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<ApisixRouteHttpPlugins>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ApisixRouteHttpTimeout>,
    /// Upstreams refer to ApisixUpstream CRD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<ApisixRouteHttpUpstreams>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpAuthentication {
    pub enable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtAuth")]
    pub jwt_auth: Option<ApisixRouteHttpAuthenticationJwtAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyAuth")]
    pub key_auth: Option<ApisixRouteHttpAuthenticationKeyAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ldapAuth")]
    pub ldap_auth: Option<ApisixRouteHttpAuthenticationLdapAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ApisixRouteHttpAuthenticationType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpAuthenticationJwtAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpAuthenticationKeyAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpAuthenticationLdapAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteHttpAuthenticationType {
    #[serde(rename = "basicAuth")]
    BasicAuth,
    #[serde(rename = "keyAuth")]
    KeyAuth,
    #[serde(rename = "jwtAuth")]
    JwtAuth,
    #[serde(rename = "wolfRBAC")]
    WolfRbac,
    #[serde(rename = "hmacAuth")]
    HmacAuth,
    #[serde(rename = "ldapAuth")]
    LdapAuth,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpBackends {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolveGranularity")]
    pub resolve_granularity: Option<ApisixRouteHttpBackendsResolveGranularity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicePort")]
    pub service_port: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteHttpBackendsResolveGranularity {
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "service")]
    Service,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exprs: Option<Vec<ApisixRouteHttpMatchExprs>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_func: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<String>>,
    pub paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteAddrs")]
    pub remote_addrs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpMatchExprs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<ApisixRouteHttpMatchExprsOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<ApisixRouteHttpMatchExprsSubject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteHttpMatchExprsOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    In,
    NotIn,
    RegexMatch,
    RegexNotMatch,
    RegexMatchCaseInsensitive,
    RegexNotMatchCaseInsensitive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpMatchExprsSubject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub scope: ApisixRouteHttpMatchExprsSubjectScope,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteHttpMatchExprsSubjectScope {
    Cookie,
    Header,
    Path,
    Query,
    Variable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpPlugins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpTimeout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
}

/// ApisixRouteUpstreamReference contains a ApisixUpstream CRD reference
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteHttpUpstreams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStream {
    pub backend: ApisixRouteStreamBackend,
    #[serde(rename = "match")]
    pub r#match: ApisixRouteStreamMatch,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<ApisixRouteStreamPlugins>>,
    pub protocol: ApisixRouteStreamProtocol,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStreamBackend {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolveGranularity")]
    pub resolve_granularity: Option<ApisixRouteStreamBackendResolveGranularity>,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "servicePort")]
    pub service_port: IntOrString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteStreamBackendResolveGranularity {
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "service")]
    Service,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStreamMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "ingressPort")]
    pub ingress_port: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStreamPlugins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixRouteStreamProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ApisixRouteStatusConditions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixRouteStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}
