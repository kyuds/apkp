use std::process::{self, Command};
use std::io::{self, Write};

fn main() {
    // (TODO): add windows support for checking for adb.
    if !cfg!(windows) && !check_adb_exists() {
        println!("adb doesn't exist. Exiting");
        process::exit(1);
    }

    // get keyword.
    print!("Enter a keyword for search: ");
    io::stdout().flush().unwrap();
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("read line failed");
    let key = key.trim();
    
    // search keyword.
    let found = search_apks(key);

    // determine apk to pull
    let apk = 
    if found.len() == 0 {
        println!("No package matched the keyword");
        return;
    } else if found.len() == 1 {
        &found[0]
    } else {
        for i in 0..found.len() {
            println!("({}) {}", i, found[i]);
        }
        print!("Found {} matching packages.\nSelect package using number: ", found.len());
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("read line failed");
        let num = num.trim().parse::<usize>().unwrap();
        &found[num]
    };

    // pull apks
    pull_apks_for_package(apk);
}

fn check_adb_exists() -> bool {
    Command::new("which")
        .arg("adb")
        .status()
        .expect("'which adb' failed to run")
        .success()
}

fn search_apks(key: &str) -> Vec<String> {
    let pkgs = Command::new("adb")
                    .args(["shell", "pm list packages"])
                    .output()
                    .expect("adb package cmd failed to run");
    if !pkgs.status.success() {
        println!("adb package cmd returned a non-zero exit code");
        process::exit(1);
    }
    let pkgs = String::from_utf8(pkgs.stdout)
                    .expect("conversion of adb package cmd output failed");
    // we remove the first 8 characters to remove "package:"
    pkgs.lines()
        .filter(|pkg| pkg.contains(key))
        .map(|pkg| (&pkg[8..pkg.len()]).to_string())
        .collect()
}

fn pull_apks_for_package(pkg: &String) {
    let apks = Command::new("adb")
                    .args(["shell", &format!("pm path {}", pkg)])
                    .output()
                    .expect("adb pm path cmd failed to run");
    if !apks.status.success() {
        println!("adb pm path cmd returned a non-zero exit code");
        process::exit(1);
    }
    let apks = String::from_utf8(apks.stdout)
                    .expect("conversion of adb pm path cmd output failed");
    let mut success = true;
    apks.lines()
        .map(|apk| &apk[8..apk.len()])
        .for_each(|apk| {
            // let trn = &apk[8..apk.len()];
            let status = Command::new("adb")
                            .args(["pull", apk])
                            .status()
                            .expect("adb pull failed to run");
            if !status.success() {
                success = false;
                println!("There was an error pulling {}", &apk);
            }
        });
    if success {
        println!("Pulled!");
    }
}
