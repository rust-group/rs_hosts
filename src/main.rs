use std::env;
use std::fs;

fn main() {
    print_hosts();
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn print_hosts() {
    let mut hosts_file_name = String::from("/etc/hosts");
    if is_windows() {
        match env::var_os("SYSTEMROOT") {
            Some(val) => {
                hosts_file_name =
                    format!("{}\\System32\\drivers\\etc\\hosts", val.to_str().unwrap())
            }
            None => panic!("SYSTEMROOT is not defined in the environment."),
        }
    }
    println!("hosts[{}]", hosts_file_name);
    println!("-------------------------------------------------");
    let hosts_str = fs::read_to_string(hosts_file_name);
    println!("{}", hosts_str.unwrap());
}