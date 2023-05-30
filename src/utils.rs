use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
pub fn touch(path: &PathBuf) {
    if !path.exists() {
        // touch
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();
    }
}
pub fn load_env_file(path: &str) -> Result<(), anyhow::Error> {
    let env_path = Path::new(path);
    if !env_path.exists() {
        return Err(anyhow::Error::msg("Env file path does not exist"));
    }
    match File::open(env_path) {
        Ok(f) => {
            let b = BufReader::new(f);
            for line in b.lines() {
                let line = line?;
                if !line.starts_with('#') {
                    let line = line.trim_matches('\"');
                    let split: Vec<&str> = line.splitn(2, '=').collect();
                    if split.len() != 2 {
                        return Err(anyhow::Error::msg(
                            "Invalid key/value pair in env file. Not able to split on '='",
                        ));
                    }
                    let key = split[1];
                    let val = split[2];
                    std::env::set_var(key, val);
                }
            }
        }
        _ => return Err(anyhow::Error::msg("Env file path does not exist")),
    };
    Ok(())
}
