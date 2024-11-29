use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use directories_next::UserDirs;
use std::collections::HashMap;

const REMOTE_RAW_DIR: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/raw";

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    // if you want to use key auth, then use following:
    // AuthMethod::with_key_file("key_file_name", Some("passphrase"));
    // or
    // AuthMethod::with_key_file("key_file_name", None);
    // or
    // AuthMethod::with_key(key: &str, passphrase: Option<&str>)

    //let auth_method = AuthMethod::with_password("mimeul");
    //let auth_method = AuthMethod::with_key_file("C:/Users/mmeuli2/.ssh/id_rsa", None);
    //let auth_method = AuthMethod::with_key_file("C:/Users/micha/.ssh/id_rsa", None);

    let key_path = match ssh_key_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get SSH key path: {}", e);
            return Ok(());
        }
    };

    let auth_method = AuthMethod::with_key_file(key_path, None);

    let client = Client::connect(
        ("130.60.24.133", 22),
        "mimeul",
        auth_method,
        ServerCheckMethod::NoCheck,
    )
    .await?;

    println!("Connected to the server");
    let result = client.execute("echo Hello SSH").await?;
    print!("stdout: {}", result.stdout);
    assert_eq!(result.stdout, "Hello SSH\n");
    assert_eq!(result.exit_status, 0);

    let result = client.execute("echo Hello Again \\:\\)").await?;
    print!("stdout2: {}", result.stdout);
    assert_eq!(result.stdout, "Hello Again :)\n");
    assert_eq!(result.exit_status, 0);


    let command = format!("test -d {} && echo 'exists'", REMOTE_RAW_DIR);
    let result = client.execute(&command).await?;
    if result.stdout.trim() == "exists" {
        println!("Directory {} exists.", REMOTE_RAW_DIR);
    } else {
        panic!("Directory {} does not exist or is inaccessible.", REMOTE_RAW_DIR);
    }
    let command = format!("ls {}", REMOTE_RAW_DIR);
    let result = client.execute(&command).await?;
    print!("stdout: {}", result.stdout);
    let stdout = result.stdout;
    let file_names: Vec<&str> = stdout.lines().collect();
    println!("List of file names:");
    for file_name in &file_names {
        println!("{}", file_name);
    }

    let tasks = create_tasks(file_names);
    for task in tasks {
        println!("{}", task.sample);
        println!("{}", task.read1);
        println!("{}", task.read2);
        println!("{}", task.is_checked);
    }

    Ok(())
}




struct Task {
    sample: String,
    read1: String,
    read2: String,
    is_checked: bool,
}

fn create_tasks(file_names: Vec<&str>) -> Vec<Task> {
    let mut tasks = Vec::new();

    // Group files by sample name
    let mut grouped_files: HashMap<String, (String, String)> = HashMap::new();
    for file_name in file_names {
        if let Some((sample, suffix)) = file_name.split_once('_') {
            match suffix {
                "1.fastq.gz" => {
                    grouped_files
                        .entry(sample.to_string())
                        .or_insert_with(|| (String::new(), String::new()))
                        .0 = file_name.to_string();
                }
                "2.fastq.gz" => {
                    grouped_files
                        .entry(sample.to_string())
                        .or_insert_with(|| (String::new(), String::new()))
                        .1 = file_name.to_string();
                }
                _ => {}
            }
        }
    }

    // Construct tasks
    for (sample, (read1, read2)) in grouped_files {
        tasks.push(Task {
            sample,
            read1,
            read2,
            is_checked: false,
        });
    }

    tasks
}

fn ssh_key_path() -> Result<String, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs.home_dir().join(".ssh").join("id_rsa");
        if path.exists() {
            match path.to_str() {
                Some(path_str) => Ok(path_str.to_string()),
                None => Err("Failed to convert SSH key path to string".to_string()),
            }
        } else {
            Err(format!("SSH key file does not exist at: {:?}", path))
        }
    } else {
        Err("Failed to determine the user's home directory".to_string())
    }
}

