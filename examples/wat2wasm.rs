use std::env;
use std::process::Command;
use std::fs;

fn main() {
  let args: Vec<String>  = env::args().collect();

  match args.len() {
      1 => {
        println!("wat2wasm xxx.wat or wasm2wat xxx.wasm. Try passing some arguments!");
      },
      3 => {
        match args[1].as_str() {
            "wat2wasm" => {
              execute("wat2wasm", args[2].as_str());
            },
            "wasm2wat" => {

            }
            _ => {
              help()
            }
        }
      },
      _ => {
        help()
      }
  }
}

fn execute(cmd: &str, path_name: &str) {
  let dir = fs::read_dir(path_name).unwrap();
  
  for entry in dir {
    let entry = entry.unwrap();
    let metadata = entry.metadata().unwrap();

    if metadata.is_file() {
     let extension_name = entry.path().extension().unwrap().to_ascii_uppercase();
     
      if cmd == "wat2wasm" {
        if extension_name == "WAT" {
          let in_path =  entry.path().to_string_lossy().to_string();
          let out_path = in_path.replace("wat","wasm");
          Command::new("wat2wasm")
            .args([in_path, "-o".to_string(), out_path])
            .output()
            .expect("failed to execute process");
        }
      } else {
        // TODO
      }
    }
    
  }

}

fn help() {
  // TODO
}