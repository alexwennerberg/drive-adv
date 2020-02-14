/* Info on character limits for each field
 https:docs.google.com/document/d/1E7sT_BMkFOXR4EsDMUUhOgeD6BrEjFzS8fHv8tNrfPU/edit
 id -- not documented. Overshot with 255.
 name -- This blog post says file names can be 32767. Since it's not official google
 documentation, I'll just make it a text field. The file linked above says 255.
 https:www.aurelp.com/tag/maximum-file-name-google-drive/
 mime_type -- 255 per RFC: https:stackoverflow.com/questions/643690/maximum-mimetype-length-when-storing-type-in-db

 permissions

 id -- same deal
 perm_type -- user, group, domain, anyone. Could be replaced with ID for efficiency
 email -- 255 https:stackoverflow.com/questions/386294/what-is-the-maximum-length-of-a-valid-email-address
 role -- owner, organizer, fileOrganizer, writer, commenter, reader
 Domain -- 255 https:en.wikipedia.org/wiki/Hostname#Restrictions_on_valid_hostnames
*/

/* user table */
CREATE TABLE IF NOT EXISTS files (
 id VARCHAR(255),
 name TEXT,
 mime_type VARCHAR(255),
 description TEXT,
 trashed BOOLEAN,
 starred BOOLEAN,
 explicitly_trashed BOOLEAN,
 trashing_user jsonb,
 trashed_time VARCHAR(255),
 properties jsonb,
 web_content_link VARCHAR(255),
 web_view_link VARCHAR(255),
 /* spaces VARCHAR(255)[], */
 version INTEGER,
 created_time VARCHAR(255),
 modified_time VARCHAR(255),
 owners jsonb,
 drive_id VARCHAR(255),
 last_modifying_user jsonb,
 shared BOOLEAN,
 viewers_can_copy_content BOOLEAN,
 copy_requires_writer_permission BOOLEAN,
 writers_can_share BOOLEAN,
 has_augmented_permissions BOOLEAN,
 folder_color_rgb VARCHAR(255),
 original_filename TEXT,
 full_file_extension VARCHAR(255),
 file_extension VARCHAR(255),
 md5_checksum VARCHAR(255),
 size BIGINT,
 quota_bytes_used BIGINT,
 head_revision_id VARCHAR(255),
 image_media_metadata jsonb,
 video_media_metadata jsonb,
 is_app_authorized BOOLEAN,
 db_id SERIAL,
 PRIMARY KEY(id)
);
create index if not exists db_id on files(db_id);

CREATE TABLE IF NOT EXISTS user_files (
 id VARCHAR(255) REFERENCES files(id),
 user_email VARCHAR(255),
 viewed_by_me BOOLEAN,
 viewed_by_me_time VARCHAR(255), 
 modified_by_me BOOLEAN,
 modified_by_me_time VARCHAR(255),
 shared_with_me_time VARCHAR(255),
 sharing_user jsonb,
 capabilities jsonb,
 CONSTRAINT user_file UNIQUE (id, user_email)
);

CREATE TABLE IF NOT EXISTS permissions (
 file_id VARCHAR(255) REFERENCES files(id),
 id VARCHAR(255),
 perm_type VARCHAR(6), 
 email_address VARCHAR(255),
 domain VARCHAR(255),
 role VARCHAR(13),
 deleted BOOLEAN,
 allow_file_discovery BOOLEAN,
 display_name VARCHAR(255),
 expiration_time VARCHAR(255),
 PRIMARY KEY (id, file_id)
 );
CREATE TABLE IF NOT EXISTS parents (
  file_id VARCHAR(255) REFERENCES files(id),
  parent_id VARCHAR(255),
  PRIMARY KEY (file_id, parent_id)
);
