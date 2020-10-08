use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("/tmp/is.found");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_prop() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("/tmp/is.found")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    fs::read_to_string("/tmp/is.found")
}

fn main() -> Result<(), Box<dyn Error>> {
    // match error handling
    let open_result = File::open("not.found");
    let _file = match open_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("/tmp/is.found") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // use unwrap_or_else type functions instead of match patterns
    let _open_result = File::open("not.found").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("/tmp/is.found").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // use unwrap to get OK value otherwise panic
    let _unwrap_file = File::open("/tmp/is.found").unwrap();

    // use expect for custom error messages to panic
    let _expect_file = File::open("/tmp/is.found").expect("/tmp/is.found was not found");

    // propagate errors
    let _read_username = read_username_from_file();

    // propagate errors using ?
    let _read_username_prop = read_username_from_file_prop();

    // use std lib to shorten logic
    let _read_username_short = read_username_from_file_short();

    // return type for ? operator must be Result<T, E>
    let _f = File::open("/tmp/is.found")?;

    Ok(())
}
