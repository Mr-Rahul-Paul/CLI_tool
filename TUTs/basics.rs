use clap::{Arg, ArgGroup, command};
// this just takes the input and assigns them to the variables (or ids??)

fn main() {
    let match_result = command!()
        .about("this is a test program")
        .group(ArgGroup::new("person-register")
        .args(["firstname", "lastname"])) 
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "first"]) //hidden alias
                .required(true), // make the argument required
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"]),
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
