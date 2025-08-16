use clap::{Arg, command};

fn main() {
    let match_result = command!()
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "first"]), //hidden alias
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname"]),
        )
        .get_matches();

    print!(
        "firstname: {:?}\n",
        match_result.get_one::<String>("firstname")
    );
    print!(
        "lastname: {:?}\n",
        match_result.get_one::<String>("lastname")
    );
}
