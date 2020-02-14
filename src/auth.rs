extern crate jsonwebtoken as jwt;

use chrono::prelude::*;
use jwt::{encode, Algorithm, Header};
use openssl::rsa;
use reqwest;
use reqwest::header;
use serde::{self, Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use log::{info, debug};

const TOKEN_LIFETIME: i64 = 3600; // 1 hour. Max 1 hour

// See https://developers.google.com/identity/protocols/OAuth2ServiceAccount
//
#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SublessClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceAccount {
    #[serde(rename = "type")]
    auth_type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
}

// TODO break up
fn generate_jwt_string(user: Option<&str>, scope: &str) -> String {
    let service_account_file = env::var("DRIVE_ADV_SERVICE_ACCOUNT").expect(
        "DRIVE_ADV_SERVICE_ACCOUNT env variable not set. Please reference path to service account",
    );
    let service_account_file =
        fs::File::open(service_account_file).expect("Service account file does not exist at path");
    let service_account: ServiceAccount = serde_json::from_reader(service_account_file).unwrap();
    let now = Utc::now().timestamp();
    let mut scope_url: String = "https://www.googleapis.com/auth/".to_string();
    scope_url.push_str(scope);
    let sub_user = match user {
        Some(u) => Some(u.to_string()),
        None => None,
    };
    let claims = Claims {
        iss: service_account.client_email,
        scope: scope_url,
        iat: now,
        exp: now + TOKEN_LIFETIME, // Expires 1 hour later
        sub: sub_user,
        aud: "https://oauth2.googleapis.com/token".to_string(),
    };
    let private_key =
        rsa::Rsa::private_key_from_pem(service_account.private_key.as_bytes()).unwrap();
    let mut header = Header::default();
    header.alg = Algorithm::RS256;
    let der = &private_key.private_key_to_der().unwrap();
    let token = encode(&header, &claims, der).unwrap();
    token
}

#[derive(Default, Debug)]
pub struct AuthToken {
    access_token: String, // TODO: change to &str
    expiration: i64,
    initialized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineToken {
    access_token: String,
    client_secret: String,
    client_id: String,
    refresh_token: String,
}

impl AuthToken {
    pub fn get_token_string(&mut self, user: Option<&str>) -> String {
        let now = Utc::now().timestamp();
        // Does not match the jwt token expiration
        if !self.initialized || now > self.expiration {
            if let Some(path) = env::var("DRIVE_ADV_OFFLINE_OAUTH").ok() {
                self.access_token = get_authentication_token_offline(&path);
            }
            else {
                self.access_token = get_authentication_token(user);
            }
            self.expiration = now + TOKEN_LIFETIME / 2;
            self.initialized = true;
        } else {
            debug!("Authentication token already exists")
        }
        self.access_token.clone() // TODO figure out lfietimes
    }
}

pub fn get_authentication_token_offline(path: &str) -> String {
    let offline_token_file =
        fs::File::open(path).expect("Service account file does not exist at path");
    let offline_token: OfflineToken = serde_json::from_reader(offline_token_file).unwrap();
    let access_token_url = "https://oauth2.googleapis.com/token";
    let client = reqwest::blocking::Client::new();
    let body = format!("client_id={}&client_secret={}&refresh_token={}&grant_type=refresh_token", offline_token.client_id, offline_token.client_secret, offline_token.refresh_token);

    let res = client
        .post(access_token_url)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .unwrap();
    let result_json: Value = res.json().unwrap();
    return result_json.get("access_token").unwrap().to_string();
}

/// Either uses a service account or oauth offline refresh token
pub fn get_authentication_token(user: Option<&str>) -> String {
    debug!("Getting authentication token");
    let scope = env::var("DRIVE_SCOPE").expect("DRIVE_SCOPE not set. Should be drive or drive.readonly");
    let access_token_url = "https://oauth2.googleapis.com/token";
    let jwt_string = generate_jwt_string(user, &scope);
    let client = reqwest::blocking::Client::new();
    let body = format!(
        "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={}",
        jwt_string
    ); // No real reason I'm using this over json except it was in the tutorial
    let res = client
        .post(access_token_url)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .unwrap();
    let result_json: Value = res.json().unwrap();
    return result_json.get("access_token").unwrap().to_string()
}
