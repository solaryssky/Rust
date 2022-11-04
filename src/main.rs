use std::fs;
use filetime::FileTime;
use std::time::{SystemTime};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Write};

// get current unixtime in second
fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}


//fn main() -> Result<(), Error> {
fn main(){
    
    //second for duration
    let dur: u64  = 86400 * 31;
    //current unixtime 
    let cur_time: u64 = get_sys_time_in_secs();
    //get file list in directory
    let paths = fs::read_dir("/app/ftpUpload/data").unwrap();

        let mut vec = Vec::new();

        let file_name = "/tmp/agent.json";
        let mut _f = File::create(file_name).unwrap();

    for path in paths {

    //get mtime of stat.json file
        let fpath = path.unwrap().path();
        let _strfpath = fpath.as_path().display().to_string();
        let _tmp = &_strfpath.clone();
        let _slice = _tmp[20..].to_string();
        let len = _slice.len() - 10; // cut ".stat.json"
        let _slice_2 = _slice[..len].to_string();

             vec.push(_slice_2);
        
        let metadata = fs::metadata(fpath).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let mtime_i64 = mtime.unix_seconds();
        let mtime_u64 = mtime_i64 as i64 as u64;
        let diff_second = cur_time - mtime_u64;  

     if diff_second > dur{
        //println!("{:?} {} {} {} {} {}", mtime_i64, cur_time, mtime_u64, diff_second, dur, _strfpath);
         fs::remove_file(&_tmp).expect("File delete failed");
    }
    
    println!("{:?} {} {} {} {} {}", mtime_i64, cur_time, mtime_u64, diff_second, dur, &_tmp);
    

        //println!("{}",unix_t.type_name());
        //println!("{}",unix_t_unsigned.type_name());

    }


   let mut _file = OpenOptions::new().write(true).append(true).open(file_name).unwrap();
   let _vec_length:i64 = *&vec.len() as i64;

   writeln!(_file, "{{").unwrap();   


let mut inc:i32 = 1;

for i in &vec {       
    if _vec_length == inc.into(){
        writeln!(_file, "{{\"name\": \"{}\"}}", i).unwrap();
      }
      else{
        writeln!(_file, "{{\"name\": \"{}\"}},", i).unwrap(); 
      }
      
inc += 1;      
}

  writeln!(_file, "}}").unwrap(); 


   


}