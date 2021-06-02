use std::process::Command;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut cmd_str = String::from("default");
    if cfg!(target_os = "linux") {
        cmd_str = "telnet".to_string();
    }


    let output = Command::new("gnome-terminal").arg("--")
                        .arg("telnet")
                        .arg("localhost")
                        .arg("8080")    
                        .output().expect("cmd exec error!");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}",output_str);
    Ok(())
}