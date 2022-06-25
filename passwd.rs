// version 0.6

#![allow(warnings, unused)]

use std::io::{stdin,stdout,Write};



pub fn paswd_mode_start_prompt_loop() -> i32
{   
    let mut ret = 0;

    loop 
    {
        ret = paswd_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }

    return 1;
}



fn paswd_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: passwd > ");

    let _=stdout().flush();
    stdin().read_line(&mut input_command).expect("error: unable to read user input");


    if let Some('\n')=input_command.chars().next_back()
    {
        input_command.pop();
    }
    if let Some('\r')=input_command.chars().next_back()
    {
        input_command.pop();
    }
    let ret  = paswd_mode_process_input_command(input_command);

    return ret;
}


fn paswd_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];
        
    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    
    valid_commands.push("passwd".to_string());
    valid_commands.push("..".to_string());

    valid_commands.push("ispasswordstrong".to_string());
    
    


    let mut bool_is_valid_command = false;
    let mut valid_command_position = 0;
    
    for (pos, commands) in valid_commands.iter().enumerate()
    {
        //println!("input_command: {}, pos: {} and val: {}", input_command, pos, commands);
        match commands
        {
            _ if input_command == commands.to_string() =>
            {
                bool_is_valid_command = true;
                valid_command_position = pos;
            }
            _ =>
            {
                if bool_is_valid_command != true
                {
                    bool_is_valid_command = false;
                }
            }
        }
    }
    if bool_is_valid_command == true
    {
        //println!("valid command. pos: {}", valid_command_position);
        //valid_commands_functions[valid_command_position]();
        if input_command == "quit".to_string() || input_command == "exit".to_string() || input_command == "q".to_string() || input_command == "e".to_string()
        {
            passwd_mode_quit();
        }
        if input_command == "passwd".to_string()
        {
            println!("already in passwd mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            passwd_mode_help();
        }
        if input_command == "..".to_string()
        {
            return 99;
        }
        if input_command == "ispasswordstrong".to_string()
        {
            passwd_mode_ispasswordstrong();
        }
        
        
    }
    if bool_is_valid_command == false
    {
        println!("invalid command");
    }

    return 1;
}




fn passwd_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}

fn passwd_mode_help()
{
    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");
    
    println!("helpers");
    println!("---------");
    println!("ispasswordstrong - find if a given password is strong or not.");

    
    1;
}


fn passwd_mode_ispasswordstrong() -> i32
{
    use std::io::{stdin,stdout,Write};

    let mut password = String::new();

    print!("Enter the password to check: ");

    let _=stdout().flush();
    stdin().read_line(&mut password).expect("error: unable to read user input");
    if let Some('\n')=password.chars().next_back() {
        password.pop();
    }
    if let Some('\r')=password.chars().next_back() {
        password.pop();
    }

    
    let mut bool_length = false;
    let mut bool_uppercase = false;
    let mut bool_lowercase = false;
    let mut bool_number: bool = false;
    let mut bool_symbol = false;


    // check for length
    let password_length = password.len();
    if password_length > 9
    {
        bool_length = true;
    }
    
    for c in password.chars()
    {
        //print!("{}, ", c);

        if c.is_ascii_uppercase()
        {
            bool_uppercase = true;
        }
        if c.is_ascii_lowercase()
        {
            bool_lowercase = true;
        }
        if c.is_ascii_digit()
        {
            bool_number = true;
        }
        if c.is_ascii_punctuation()
        {
            bool_symbol = true;
        }
    }


    println!("Entered password is: {}", password);
    println!("Length of the password: {}", password_length);
    print!("Is this a strong password?: ");

    if bool_length == true && bool_uppercase == true && bool_lowercase == true && bool_symbol == true
    {
        println!("Yes :)");
    }
    else {
        println!("No :(");
    }



    return 1;
}