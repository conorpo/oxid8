use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::io;
use std::path::Path;
use std::random::random;

pub fn save_file(path: &Path, data: &[u8]) -> io::Result<()> {
    fs::write(path, data)?;
    Ok(())
}

pub fn read_file(path: &Path) -> io::Result<String> {
    let data: Vec<u8> = fs::read(path)?;
    let string = String::from_utf8(data).expect("Failed to parse data.");
    
    Ok(string)
}


//creates a new file and u
pub fn update_file(path: &Path, data: &[u8]) -> Result<String,()> {
    let my_num: u8 = random();
    let temp_name = format!("{}.tmp.{}", path.to_str().unwrap(), my_num);
    fs::write(&temp_name, data).unwrap();
    Ok(temp_name)
}

pub fn log(path: &Path, entry: &[u8]) -> io::Result<usize>{
    let file = OpenOptions::new().write(true).append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);

    buf_writer.write(entry)
}

#[cfg(test)]
mod tests {
    use std::{fmt::format, fs::File, path::Path};

    use crate::file::read_file;

    use super::{log, save_file, update_file};

    #[test]
    fn test_alot() {
        let path = Path::new("./test.txt");
        let log_path = Path::new("./log.txt");

        let data = "Test data".to_owned();
        save_file(&path, &data.as_bytes()).unwrap();
        assert_eq!(read_file(&path).unwrap(), data);

        let data2 = "More data".to_owned();
        save_file(&path, &data2.as_bytes()).unwrap();
        assert_eq!(read_file(&path).unwrap(), data2);
        
        save_file(&log_path, &[]).unwrap();
        for i in 0..100 {
            let data_string = format(format_args!("Data {:3}\n",i));
            
            log(&log_path, &data_string.as_bytes()).unwrap();
        }
        let file = File::open(&log_path).unwrap();
        assert_eq!(file.metadata().unwrap().len(), ("Data 000\n".as_bytes().len() * 100) as u64);
    }

    // #[test]
    // fn test_update() {
    //     let path = Path::new("./test.txt");

    //     let data = "The text file has just been updated.".to_owned();

    //     let res = update_file(path, data.as_bytes()).unwrap();

    //     let res_path = Path::new(&res);
    //     assert_eq!(read_file(&res_path).unwrap(), data);
        
    // }
}