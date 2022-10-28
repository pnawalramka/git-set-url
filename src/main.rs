// Traverse the current directory (non-recursive) to find all git repos
// and update the remote repository URL, based on current repository name.

use std::env;
use std::fs;
use std::process::Command;

fn is_hidden(entry: &fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() -> std::io::Result<()>{
    let current_dir = env::current_dir()?;
    println!(
        "Looking for git repos inside: {:?}",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let hidden = is_hidden(&entry);
        let is_dir = entry.metadata()?.is_dir();
        if hidden || !is_dir{
            continue;
        }

        println!("Checking {}", entry.path().display());
        let path = entry.path().into_os_string().into_string().unwrap();

        // Check if path is a git repo
        let rev_parse_out = Command::new("git").arg("-C").arg(path).arg("rev-parse").output()?;
        if !rev_parse_out.status.success() {
            println!("not a git repo...\n");
            continue
        }

        println!("Updating {}", entry.path().display());
        assert!(env::set_current_dir(&entry.path()).is_ok());

        let git_get_url_out = Command::new("git").arg("remote").arg("get-url").arg("origin").output()?;
        if !git_get_url_out.status.success() {
            println!("error getting remote origin...\n");
            continue
        }
        let raw_output:String = String::from_utf8(git_get_url_out.stdout).unwrap().into();
        let (_,repo_name) = raw_output.strip_suffix("\n").unwrap().rsplit_once("/").unwrap();

        // change the host as appropriate
        let new_url = format!("{}{}", "git@github.com:awesomeorg/", repo_name);
        let git_set_url_out = Command::new("git").arg("remote").arg("set-url").arg("origin").arg(new_url).output()?;
        if !git_set_url_out.status.success() {
            println!("failed to set new url... :-/");
            continue
        } else {
            println!("done!\n");
        }
    }

    Ok(())
}
