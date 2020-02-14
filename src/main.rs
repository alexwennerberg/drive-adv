use clap::{App, Arg, SubCommand};
use log::{debug};
use env_logger;

fn main() {
    let matches = App::new("Drive Advanced")
        .version("0.1.0-alpha")
        .author("Alex Wennerberg <alex@alexwennerberg.com>")
        .about("Advanced tools to work with Google Drive")
        .subcommand(
            App::new("list")
            .about("List files to database")
            .arg(
                Arg::with_name("users")
                    .short("u")
                    .long("users")
                    .value_name("USERS")
                    .help("User(s) to search drive.")
                    .takes_value(true)
                    .required(true)
                    .multiple(true)
            )
            .arg(
                Arg::with_name("fields")
                    .short("f")
                    .long("fields")
                    .value_name("FIELDS")
                    .help("Fields required for search.\n See https://developers.google.com/drive/api/v3/fields-parameter. A comma separated list of everything inside files(), eg 'id,name,description'. Default is all that can be stored by our DB (may be slow)")
                    .takes_value(true)
            ).arg(
                Arg::with_name("query")
                    .short("q")
                    .long("query")
                    .value_name("QUERY")
                    .help("Add a drive query string: https://developers.google.com/drive/api/v3/search-files. By default no query.")
                    .takes_value(true)
        ).arg(
        Arg::with_name("page_token")
            .short("t")
            .long("page_token")
            .value_name("TOKEN")
            .help("Specify the page token to start from -- useful for very long running processes to start where you left off")
            .takes_value(true)
        )).get_matches();
        // TODO -- implement
        // .subcommand(
        //     App::new("list_activities")
        //     .about("Get list of activities. Requires scope https://www.googleapis.com/auth/admin.reports.audit.readonly") // TODO: take user
        //     .arg(
        //         Arg::with_name("users")
        //             .short("u")
        //             .long("users")
        //             .value_name("USERS")
        //             .help("User(s) to search audit log. By default all. Note that this MUSt be the primary email address  -- not an alias. Specify 'all' to get all users")
        //             .takes_value(true)
        //             .multiple(true)
        //     ).arg(
        //         Arg::with_name("start_time")
        //         .short("s")
        //         .long("start_time")
        //         .value_name("START_TIME")
        //         .help("Start time in RFC3339 format, eg 2010-10-28T10:26:35.000Z")
        //         .takes_value(true))
        //     .arg(Arg::with_name("end_time")
        //         .short("e")
        //         .long("end_time")
        //         .value_name("END_TIME")
        //         .help("end time in RFC3339 format, eg 2010-10-31:26:35.000Z")
        //         .takes_value(true))// TODO -- add more parameters. at leaest starttime and endtime
        // )
        // 
    
    env_logger::init();
    debug!("Logging initialized");

    if matches.is_present("list") {
        let list_subcommand = matches.subcommand_matches("list").unwrap();
        let users = list_subcommand.values_of("users").unwrap();
        let fields = list_subcommand.value_of("fields");
        let query = list_subcommand.value_of("query");
        let page_token = list_subcommand.value_of("page_token");
        for user in users {
            debug!("Listing files for {}", user);
            drive_adv::list_files(user, fields, query, page_token);
        }
    }
    // TODO -- implement
    if matches.is_present("list_activities") {
        let list_subcommand = matches.subcommand_matches("list_activities").unwrap();
        let users = list_subcommand.values_of("users").unwrap(); // TODO allow all
        let start_time = list_subcommand.value_of("start_time");
        let end_time = list_subcommand.value_of("end_time");
        for user in users {
            drive_adv::list_audit_log(user, start_time, end_time);
        }
    }
    debug!("Complete.")
}
