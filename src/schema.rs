table! {
    files (id) {
        id -> Varchar,
        name -> Nullable<Text>,
        mime_type -> Nullable<Varchar>,
        description -> Nullable<Text>,
        trashed -> Nullable<Bool>,
        starred -> Nullable<Bool>,
        explicitly_trashed -> Nullable<Bool>,
        trashing_user -> Nullable<Jsonb>,
        trashed_time -> Nullable<Text>,
        properties -> Nullable<Jsonb>,
        version -> Nullable<Integer>,
        web_content_link -> Nullable<Varchar>,
        web_view_link -> Nullable<Varchar>,
        created_time -> Nullable<Varchar>,
        modified_time -> Nullable<Varchar>,
        owners -> Nullable<Jsonb>,
        drive_id -> Nullable<Varchar>,
        last_modifying_user -> Nullable<Jsonb>,
        shared -> Nullable<Bool>,
        viewers_can_copy_content -> Nullable<Bool>,
        copy_requires_writer_permission -> Nullable<Bool>,
        writers_can_share -> Nullable<Bool>,
        has_augmented_permissions -> Nullable<Bool>,
        folder_color_rgb -> Nullable<Varchar>,
        original_filename -> Nullable<Text>,
        full_file_extension -> Nullable<Varchar>,
        file_extension -> Nullable<Varchar>,
        md5_checksum -> Nullable<Varchar>,
        size -> Nullable<BigInt>,
        quota_bytes_used -> Nullable<BigInt>,
        head_revision_id -> Nullable<Varchar>,
        image_media_metadata -> Nullable<Jsonb>,
        video_media_metadata -> Nullable<Jsonb>,
        is_app_authorized -> Nullable<Bool>,
    }
}

table! {
    user_files (id, user_email) {
        id -> Varchar,
        user_email -> Varchar,
        viewed_by_me-> Nullable<Bool>,
        viewed_by_me_time -> Nullable<Text>,
        modified_by_me -> Nullable<Bool>,
        modified_by_me_time -> Nullable<Varchar>,
        shared_with_me_time -> Nullable<Varchar>,
        sharing_user -> Nullable<Jsonb>,
        capabilities -> Nullable<Jsonb>,
    }
}

table! {
    parents (file_id, parent_id) {
        file_id -> Varchar,
        parent_id -> Varchar,
    }
}

table! {
    permissions (id, file_id) {
        file_id -> Varchar,
        id -> Varchar,
        perm_type -> Nullable<Varchar>,
        email_address -> Nullable<Varchar>,
        domain -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        allow_file_discovery-> Nullable<Bool>,
        display_name -> Nullable<Varchar>,
        expiration_time-> Nullable<Varchar>,
    }
}

joinable!(parents -> files (file_id));
joinable!(permissions -> files (file_id));
joinable!(user_files -> files (id));

allow_tables_to_appear_in_same_query!(
    files,
    parents,
    permissions,
    user_files,
);
