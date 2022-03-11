use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

// "In some cases Rust will require you to acknowledge the possibility of 
// an error and take some action before your code will compile." LOL

fn main() {
    
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file, 
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    //
    // Same code as above written by "a more seasoned Rustacean":
    //
    fn main() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the File: {:?}", error);
            }
        });
    } // so cool


    fn read_username_from_file() -> Result<String, std::io::Error> {

        let f = File::open("hello.txt");

        // Rewritten for the Propagating Errors section
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
    
    let mystring = read_username_from_file();
    println!("String from file: {:?}", mystring);

    // And now, a totally cool way to do the above with shorthand!!
    //
    fn read_username_from_file2() -> Result<String, std::io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let this_time = read_username_from_file2();
    println!("This shorthand version should be the same: {:?}", this_time);


    // And Even More Succinct!!
    //
    fn read_username_from_file3() -> Result<String, std::io::Error> {
        let mut s2 = String::new();

        File::open("hello.txt")?.read_to_string(&mut s2)?;

        Ok(s2)

    }

    let and_now = read_username_from_file3();
    println!("This microhand version should also be the same: {:?}", and_now);

    // Finally, since this happens so much there's a function built-in :-P
    //
    fn read_username_from_file4() -> Result<String, std::io::Error> {
        use std::fs;

        fs::read_to_string("hello.txt")
    }

    let and_func = read_username_from_file4();
    println!("Finally, using std::fs::read_to_string, should also be the same: {:?}", and_func);

}
