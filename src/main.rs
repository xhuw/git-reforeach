use std::path::Path;

fn do_git(dir: &Path, args: &[String]) -> () {
    println!("++ {:?}", dir);
    let mut child = std::process::Command::new("git")
        .current_dir(if dir.to_str().unwrap().len() > 0 {
            dir
        } else {
            Path::new(".")
        })
        .args(args)
        .spawn()
        .expect("git failed");
    let _ = child.wait();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let match_str = args.get(1).expect("First argument must be the match str");
    let forward_args = args.get(2..).unwrap();

    let m = regex::Regex::new(match_str).expect("Invalid regex passed");

    for entry in glob::glob("**/.git").unwrap() {
        match entry {
            Ok(p) => {
                let p = p.parent().unwrap();
                if m.is_match(p.to_str().unwrap()) {
                    do_git(p, forward_args)
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // todo return code
}
