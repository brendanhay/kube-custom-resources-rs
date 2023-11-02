// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/nginxinc/kubernetes-ingress/k8s.nginx.org/v1/policies.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// PolicySpec is the spec of the Policy resource. The spec includes multiple fields, where each field represents a different policy. Only one policy (field) is allowed.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "k8s.nginx.org", version = "v1", kind = "Policy", plural = "policies")]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(schema = "disabled")]
pub struct PolicySpec {
    /// AccessControl defines an access policy based on the source IP of a request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControl")]
    pub access_control: Option<PolicyAccessControl>,
    /// BasicAuth holds HTTP Basic authentication configuration policy status: preview
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<PolicyBasicAuth>,
    /// EgressMTLS defines an Egress MTLS policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressMTLS")]
    pub egress_mtls: Option<PolicyEgressMtls>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    /// IngressMTLS defines an Ingress MTLS policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressMTLS")]
    pub ingress_mtls: Option<PolicyIngressMtls>,
    /// JWTAuth holds JWT authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt: Option<PolicyJwt>,
    /// OIDC defines an Open ID Connect policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc: Option<PolicyOidc>,
    /// RateLimit defines a rate limit policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rateLimit")]
    pub rate_limit: Option<PolicyRateLimit>,
    /// WAF defines an WAF policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waf: Option<PolicyWaf>,
}

/// AccessControl defines an access policy based on the source IP of a request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyAccessControl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

/// BasicAuth holds HTTP Basic authentication configuration policy status: preview
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyBasicAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// EgressMTLS defines an Egress MTLS policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyEgressMtls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionReuse")]
    pub session_reuse: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslName")]
    pub ssl_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustedCertSecret")]
    pub trusted_cert_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyDepth")]
    pub verify_depth: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyServer")]
    pub verify_server: Option<bool>,
}

/// IngressMTLS defines an Ingress MTLS policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyIngressMtls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertSecret")]
    pub client_cert_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crlFileName")]
    pub crl_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyClient")]
    pub verify_client: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyDepth")]
    pub verify_depth: Option<i64>,
}

/// JWTAuth holds JWT authentication configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyJwt {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwksURI")]
    pub jwks_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyCache")]
    pub key_cache: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// OIDC defines an Open ID Connect policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyOidc {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessTokenEnable")]
    pub access_token_enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authEndpoint")]
    pub auth_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authExtraArgs")]
    pub auth_extra_args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwksURI")]
    pub jwks_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectURI")]
    pub redirect_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenEndpoint")]
    pub token_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneSyncLeeway")]
    pub zone_sync_leeway: Option<i64>,
}

/// RateLimit defines a rate limit policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyRateLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub burst: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noDelay")]
    pub no_delay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rejectCode")]
    pub reject_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneSize")]
    pub zone_size: Option<String>,
}

/// WAF defines an WAF policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyWaf {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apBundle")]
    pub ap_bundle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apPolicy")]
    pub ap_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// SecurityLog defines the security log of a WAF policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityLog")]
    pub security_log: Option<PolicyWafSecurityLog>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityLogs")]
    pub security_logs: Option<Vec<PolicyWafSecurityLogs>>,
}

/// SecurityLog defines the security log of a WAF policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyWafSecurityLog {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apLogConf")]
    pub ap_log_conf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logDest")]
    pub log_dest: Option<String>,
}

/// SecurityLog defines the security log of a WAF policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyWafSecurityLogs {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apLogConf")]
    pub ap_log_conf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logDest")]
    pub log_dest: Option<String>,
}

/// PolicyStatus is the status of the policy resource
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

