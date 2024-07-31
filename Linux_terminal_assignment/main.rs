use std::process::Command;

fn executing_os_commands_linux(command_full:&str){

    let output: Output = Command::new(program:actual_command)
        .arg(arg1);
        .output();
        .unwrap();

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
    
}

fn main() {
    // reading_from_console();
    // reading_from_file();

    let full_command = accept_linux_command_from_user();
    executing_os_commands_linux("mkdir hello");
    executing_os_commands_linux("echo hello");
    executing_os_commands_linux("ls -la");
}