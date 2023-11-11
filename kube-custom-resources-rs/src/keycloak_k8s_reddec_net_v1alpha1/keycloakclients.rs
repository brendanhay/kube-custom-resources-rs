// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/reddec/keycloak-ext-operator/keycloak.k8s.reddec.net/v1alpha1/keycloakclients.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// KeycloakClientSpec defines the desired state of KeycloakClient
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keycloak.k8s.reddec.net", version = "v1alpha1", kind = "KeycloakClient", plural = "keycloakclients")]
#[kube(namespaced)]
#[kube(status = "KeycloakClientStatus")]
#[kube(schema = "disabled")]
pub struct KeycloakClientSpec {
    /// Domain which will be used for redirect callback.
    pub domain: String,
    /// Realm name.
    pub realm: String,
    /// Secret name where to store credentials. Optional, if not set - CRD name will be used. Contains: clientID, clientSecret, realm, discoveryURL, realmURL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// KeycloakClientStatus defines the observed state of KeycloakClient
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientStatus {
}

