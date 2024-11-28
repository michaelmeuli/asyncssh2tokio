use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
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
    let auth_method = AuthMethod::with_key_file(ssh_key_path(), None);

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

    Ok(())
}

fn ssh_key_path() -> String {
    UserDirs::new()
        .unwrap()
        .home_dir()
        .join(".ssh")
        .join("id_rsa")
        .to_str()
        .unwrap()
        .to_string()
}
