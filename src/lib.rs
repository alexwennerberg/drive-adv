pub mod auth;
pub mod schema;
pub mod db;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::{debug_query, insert_into};
use std::env;

use schema::{files, user_files, parents, permissions};

use reqwest;
use serde::{self, Deserialize, Serialize};
use serde_json::Value;
use diesel::pg::upsert::*;
use db::establish_connection;

// TODO -- implement
// There are a bunch of different possible log items. this shcema should encompass all of them,
// since there is a lot of overlap
// https://developers.google.com/admin-sdk/reports/v1/appendix/activity/drive
// https://developers.google.com/admin-sdk/reports/v1/reference/activities/list
#[derive(Debug, Default, Deserialize)]
pub struct LogEntry {
    // general
    ip_address: Option<String>,
    time: Option<String>,
    actor: Option<String>,
    actor_caller_type: Option<String>,
    // drive specific
    event_name: Option<String>,
    destination_folder_id: Option<String>,
    doc_id: Option<String>,
    doc_title: Option<String>,
    owner: Option<String>,
    originating_app_id: Option<String>,
    primary_event: Option<bool>,
    visibility: Option<String>,
    shared_drive_id: Option<String>,
    new_value: Option<String>,
    old_value: Option<String>,
    old_visibility: Option<String>,
    visibility_change: Option<String>,
}

// TODO -- implement
/// See https://developers.google.com/admin-sdk/reports/v1/reference/activities/list
/// user should be "all" to list all users
pub fn list_audit_log(user: &str, start_time: Option<&str>, end_time: Option<&str>) {
    // TODO -- combine duplicate code
    // TODO -- add more parameters
    let mut token = auth::AuthToken::default();
    let auth_token = token.get_token_string(None);
    let list_url = format!("https://www.googleapis.com/admin/reports/v1/activity/users/{}/applications/drive", user);
    let query: Vec<(&str, &str)> = vec![];
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(&list_url)
        .query(&query)
        .bearer_auth(&auth_token)
        .send()
        .unwrap();
}

/// Fields -- comma-separated list of fields inside the files() object to get
/// See https://developers.google.com/drive/api/v3/reference/files/list
pub fn list_files(user: &str, fields: Option<&str>,  drive_query: Option<&str>,page_token: Option<&str>) {
    let mut token = auth::AuthToken::default();
    let conn = establish_connection();
    let list_url = "https://www.googleapis.com/drive/v3/files";
    // use succession maybe
    let mut page_token = match page_token {
        Some(p) => p.to_string(),
        None => String::new(),
    };
    loop {
        let auth_token = token.get_token_string(Some(user)); // should auto refresh
        let mut query = vec![
            ("pageSize", "1000"), // limited to 100 usually when you get a large number of fields
        ];
        // By default -- all fields
        let f = format!("nextPageToken,files({})", &fields.unwrap_or("*"));
        query.push(( "fields", &f));
        let mut qu = String::new();
        if let Some(q) = drive_query {
            qu = format!("{}", q);
            query.push(("q", &qu));
        }
        if page_token != "" {
            query.push(("pageToken", &page_token));
        }
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(list_url)
            .query(&query)
            .bearer_auth(&auth_token)
            .send()
            .unwrap();
        // TODO: handle errors
        let response: Value = res.json().unwrap();
        // Look out for https://github.com/diesel-rs/diesel/issues/860
        // This is a mess because diesel lacks that feature
        if response.get("error") != None {
            println!("{:?}", response);
        }
        let files: Vec<File> = serde_json::from_value(response["files"].clone()).unwrap();
        // all this is Horrible.
        let iterfiles: Vec<Value> = serde_json::from_value(response["files"].clone()).unwrap();
        insert_into(files::dsl::files)
            .values(&files)
            .on_conflict_do_nothing()
            .execute(&conn)
            .unwrap();
        for file in iterfiles {
            let mut user_file: UserFile = serde_json::from_value(file.clone()).unwrap();
            user_file.user_email = Some(user.to_string());
            insert_into(user_files::dsl::user_files)
                .values(&user_file)
                .on_conflict_do_nothing()
                .execute(&conn)
                .unwrap();

            if file.get("permissions") != None {
                let mut permissions: Vec<Permission> =
                    serde_json::from_value(file["permissions"].clone()).unwrap();
                for mut perm in &mut permissions {
                    perm.file_id = file["id"].as_str().unwrap().to_string();
                }
                insert_into(permissions::dsl::permissions)
                    .values(&permissions)
                    .on_conflict_do_nothing() // TODO upsert
                    .execute(&conn)
                    .unwrap();
            }
            let iterparents: Vec<Value> =
                serde_json::from_value(file["parents"].clone()).unwrap_or(vec![]);
            let parents: Vec<Parent> = iterparents
                .iter()
                .map(|p| Parent {
                    file_id: file["id"].as_str().unwrap().to_string(),
                    parent_id: p.as_str().unwrap().to_string(),
                })
                .collect();
            insert_into(parents::dsl::parents)
                .values(&parents)
                .on_conflict_do_nothing() // TODO upsert
                .execute(&conn)
                .unwrap();
        }
        if let Some(r) = response.get("nextPageToken") {
            page_token = r.as_str().unwrap().to_string();
        }
        else { break };
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
    files: Vec<File>,
}

// File metadata specific to a user
#[derive(Debug, Default, Deserialize, Insertable)]
#[table_name = "user_files"]
pub struct UserFile {
    id: String,
    user_email: Option<String>,
    #[serde(rename = "viewedByMe")]
    viewed_by_me: Option<bool>,
    #[serde(rename = "viewedByMeTime")]
    viewed_by_me_time: Option<String>,
    #[serde(rename = "modifiedByMeTime")]
    modified_by_me_time: Option<String>,
    #[serde(rename = "sharedWithMeTime")]
    shared_with_me_time: Option<String>,
    #[serde(rename = "sharingUser")]
    sharing_user: Option<serde_json::Value>,
    capabilities: Option<serde_json::Value>
}

// TODO: find a way to make all fields default maybe
// https://developers.google.com/drive/api/v3/reference/files
#[derive(Debug, Default, Deserialize, Insertable)]
#[table_name = "files"]
pub struct File {
    id: String,
    name: Option<String>,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
    description: Option<String>,
    trashed: Option<bool>,
    starred: Option<bool>,
    #[serde(rename = "explicitlyTrashed")]
    explicitly_trashed: Option<bool>,
    #[serde(rename = "trashingUser")]
    trashing_user: Option<serde_json::Value>,
    #[serde(rename = "trashedTime")]
    trashed_time: Option<String>,
    properties: Option<serde_json::Value>,
    // app properties
   // spaces: Option<Vec<String>>, broken for some reason   
    // version: Option<i32>, BROKEN
    //
    // web_view_link -> Nullable<Varchar>,
    #[serde(rename = "webContentLink")]
    web_content_link: Option<String>,
    #[serde(rename = "webViewLink")]
    web_view_link: Option<String>,
    // iconLink: String,
    // hasThumbnail: bool,
    // thumbnailLink: String,
    // thumbnailVersion: u32,
    #[serde(rename = "createdTime")]
    created_time: Option<String>,
    #[serde(rename = "modifiedTime")]
    modified_time: Option<String>,
    owners: Option<serde_json::Value>,
    #[serde(rename = "driveId")]
    drive_id: Option<String>,
    #[serde(rename = "lastModifyingUser")]
    last_modifying_user: Option<serde_json::Value>,
    shared: Option<bool>,
    #[serde(rename = "viewersCanCopyContent")]
    viewers_can_copy_content: Option<bool>,
    #[serde(rename = "copyRequiresWriterPermission")]
    copy_requires_writer_permission: Option<bool>,
    #[serde(rename = "writersCanShare")]
    writers_can_share: Option<bool>,
    #[serde(rename = "hasAugmentedPermissions")]
    has_augmented_permissions: Option<bool>,
    #[serde(rename = "folderColorRgb")]
    folder_color_rgb: Option<String>,
    #[serde(rename = "originalFilename")]
    original_filename: Option<String>,
    #[serde(rename = "fullFileExtension")]
    full_file_extension: Option<String>,
    #[serde(rename = "fileExtension")]
    file_extension: Option<String>,
    #[serde(rename = "md5Checksum")]
    md5_checksum: Option<String>,
    // size: Option<i64>, BROKEN
    // #[serde(rename = "quotaBytesUsed")]
    // quota_bytes_used:  Option<i64>, // TBROKEN
    #[serde(rename = "headRevisionId")]
    head_revision_id: Option<String>,
    // "contentHints": {
    #[serde(rename = "imageMediaMetadata")]
    image_media_metadata: Option<serde_json::Value>,
    #[serde(rename = "videoMediaMetadata")]
    video_media_metadata: Option<serde_json::Value>,
    #[serde(rename = "isAppAuthorized")]
    is_app_authorized: Option<bool>,
    // exportLinks
}

// https://developers.google.com/drive/api/v3/reference/permissions
#[derive(Debug, Insertable, Deserialize)]
#[table_name = "permissions"]
struct Permission {
    #[serde(default)]
    file_id: String,
    id: String,
    #[serde(rename = "type")]
    perm_type: Option<String>,
    #[serde(rename = "emailAddress")]
    email_address: Option<String>,
    domain: Option<String>,
    role: Option<String>,
    #[serde(rename = "allowFileDiscovery")]
    allow_file_discovery: Option<bool>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "expirationTime")]
    expiration_time: Option<String>,
    deleted: Option<bool>,
    // permissionDetails[]  
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "parents"]
struct Parent {
    file_id: String,
    parent_id: String,
}
