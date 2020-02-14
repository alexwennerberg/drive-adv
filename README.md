# Google Drive Advanced

Advanced developer and auditing tools for working with Google Drive.

### Features:

* *Auditing all files* -- including file info, permissions, and parent relationships between files. Writes to configurable postgres database.
* Getting tree folder structure and allowing tree structure to be queried via SQL.

TO ADD:
* List activity log items
* Listing across all users in a domain / multiple domains
* Async calls for improved performance
* Potentially tools to perform certain actions on drive files (TBD)

## About Google Drive

Google Drive is not exactly a tree structure. Unlike a traditional Unix or Windowsfilesystem, files can have multiple parents, ie, they can live in multiple locations simultaneously.

Also unlike a traditional filesystem, permissions are not immediately inherited from the parent folder. This means if I add a permission to a file, it takes some time to propagate permissions down to its children.

Google Drive must be thought of as the whole Drive universe, not just individual user drives. Files can inherit permissions from files that users don't necessarily know about. Files can have parents specific to an individual user account.

## Installation

Install the following dependencies:

* openssl
* postgresql

[Install Cargo/Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

`cargo install --git https://github.com/Cloudbakers/drive-adv` 

## Configuration

Drive Advanced works with both a Service Account, which provides you access to all users in a domain, and an individual user account, using "Offline OAuth" (There isn't really a better term for this).

Enable Drive API in project.

For service acccount auth, give the service account access to your Google Drive domain [here](https://developers.google.com/admin-sdk/directory/v1/guides/delegation#delegate_domain-wide_authority_to_your_service_account)

The scope we need is the drive.readonly scope.

Configuration is done via environment variables. See sample_env for examples
Path to service acct in env variable

To use individual user offline OAuth, get an offline OAuth refresh token, eg with [AGM](https://github.com/Cloudbakers/agm) (`~agm --run-oauth --scopes [scopes] --email [your email]` then copy the path `/home/.agm/oauth_credentials/[your email].json` to `DRIVE_ADV_OFFLINE_OAUTH` env variable). Setting this variable will break service account auth -- choose one or the other when running a command.

Create and setup your database. We'll need to add the Ltree extension to run queries.

```
sudo -u postgres createdb drive_adv

sudo -u postgres drive_adv -d drive_sandbox -c "create extension ltree"
```

Set `DATABASE_URL` to the postgresql connection string URL. The file `sample_env` provides an example of the environment variables you'll need to set.  

Drive Advanced allows you to query the Drive file tree -- run the sql script `sql/create_filetree` to set this up. This may take some time on a large file system.

## Usage 

Use `drive_adv --help` to get the command line interface

First, list all the files for all the relevant users to the databse using the `drive_adv list` command. This will write to the postgres db you set up. Then, you can query this database to get any information you need about the Drive environment. Some sample queries exist in `sql/`


## Contributing

Contributions, bug reports, questions and feedback from users of all levels are greatly appreciated! :)
