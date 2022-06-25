// version 0.6

#![allow(warnings, unused)]

use std::any::type_name;
use std::io::{stdin,stdout,Write};



pub fn hunts_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = hunts_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn hunts_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: hunts > ");

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
    let ret  = hunts_mode_process_input_command(input_command);

    return ret;
}


fn hunts_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    valid_commands.push("cls".to_string());
    

    valid_commands.push("hunts".to_string());
    valid_commands.push("hunt rdp".to_string());
    valid_commands.push("hunt printers".to_string());
    valid_commands.push("hunt aps".to_string());

    valid_commands.push("..".to_string());
    
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

    //only numbers
    if input_command.chars().all(char::is_numeric) == true
    {
        bool_is_valid_command = true;
        //println!("only numbers");
    }


    if bool_is_valid_command == true
    {
        //println!("valid command. pos: {}", valid_command_position);
        //valid_commands_functions[valid_command_position]();
        if input_command == "quit".to_string() || input_command == "exit".to_string() || input_command == "q".to_string() || input_command == "e".to_string()
        {
            hunts_mode_quit();
        }
        if input_command == "hunts".to_string()
        {
            println!("already in hunts mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            hunts_mode_help();
        }
        if input_command == "cls".to_string()
        {
            hunts_mode_clrscr();
        }

        if input_command == "hunt rdp".to_string()
        {
            hunts_mode_nmap();
        }
        
        

        if input_command == "..".to_string()
        {
            return 99;
        }



        


    }
    if bool_is_valid_command == false
    {
        println!("invalid command");
    }

    return 1;
}


fn hunts_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn hunts_mode_help()
{
    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("-------");
    println!("hunt rdp       - find out internal systems that is running RDP service.");
    println!("hunt printers  - find out internal printers that are running.");
    println!("hunt aps       - find out WiFi access points that are available.");
    
    1;
}





fn hunts_mode_nmap() -> i32
{


    return 1;
}


fn hunts_mode_clrscr()
{
    print!("\x1B[2J");
    print!("\x1B[2J");

    //print!("{}[2J", 27 as char);
    print!("\x1B[2J\x1B[1;1H");

}




fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}