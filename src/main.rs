use async_ssh2_tokio::client::{Client, AuthMethod, ServerCheckMethod};
use std::fs;
use directories_next::UserDirs;

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

    
    let auth_method = match read_ssh_key() {
        Ok(key_data) => {
            println!("Auth method created successfully.");
            AuthMethod::PrivateKey { key_data, key_pass: None }
        }
        Err(err) => {
            eprintln!("Failed to read SSH key: {}", err);
            return Err(async_ssh2_tokio::Error::KeyInvalid(russh_keys::Error));
        }
    };

    let client = Client::connect(
        ("130.60.24.133", 22),
        "mimeul",
        auth_method,
        ServerCheckMethod::NoCheck,
    ).await?;

    println!("Connected to the server");
    let result = client.execute("ls").await?;
    print!("stdout: {}", result.stdout);
    assert_eq!(result.stdout, "Hello SSH\n");
    assert_eq!(result.exit_status, 0);

    let result = client.execute("echo Hello Again \\:\\)").await?;
    print!("stdout2: {}", result.stdout);
    assert_eq!(result.stdout, "Hello Again :)\n");
    assert_eq!(result.exit_status, 0);

    Ok(())
}

fn read_ssh_key() -> Result<String, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let ssh_dir = user_dirs.home_dir().join(".ssh");
        let key_file = ssh_dir.join("id_rsa");

        if key_file.exists() {
            fs::read_to_string(&key_file)
                .map_err(|e| format!("Error reading SSH key file: {}", e))
        } else {
            Err(format!("SSH key file does not exist: {:?}", key_file))
        }
    } else {
        Err("Failed to get user directories".to_string())
    }
}