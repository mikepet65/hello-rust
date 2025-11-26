use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

const FILE_PATH: &str = "target/hello.txt";
const ERROR_OPEN: &str = "Problem opening the file";
const ERROR_CREATE: &str = "Problem creating the file";

pub fn open_file() {
    let io_result = File::open(FILE_PATH);

    let file_handle = match io_result {
        Ok(file) => file,
        Err(error) => panic!("{ERROR_OPEN}: {:?}", error),
    };
}

pub fn open_or_create_file() {
    let io_result = File::open(FILE_PATH);

    let file_handle = match io_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(file) => file,
                Err(error) => panic!("{ERROR_CREATE}: {:?}", error),
            },
            other_errors => panic!("{ERROR_OPEN}: {:?}", other_errors),
        }
    };
}

pub fn open_file_or_panic() {
    let file_handle = File::open(FILE_PATH).unwrap();
    let file_handle = File::open(FILE_PATH).expect(ERROR_OPEN);
}

pub fn long_read_from_file() -> Result<String, io::Error> {
    let io_result = File::open(FILE_PATH);

    let mut file_handle = match io_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut content = String::new();

    match file_handle.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}

pub fn short_read_from_file() -> Result<String, io::Error> {
    let mut file_handle = File::open(FILE_PATH)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    
    Ok(content)
}

pub fn shortest_read_from_file() -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(FILE_PATH)?.read_to_string(&mut content)?;

    Ok(content)
}

pub fn read_from_file() -> Result<String, io::Error> {
    fs::read_to_string(FILE_PATH)
}

pub fn write_to_file(content: &str) -> Result<(), io::Error> {
    fs::write(FILE_PATH, content)?;
    Ok(())
}

pub fn delete_the_file() -> Result<(), io::Error> {
    fs::remove_file(FILE_PATH)?;
    Ok(())
}
