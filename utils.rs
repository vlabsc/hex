// version 0.6

#![allow(warnings, unused)]

use std::io::{stdin,stdout,Write};
use std::time::{Duration, Instant};



pub fn utils_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = utils_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn utils_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: utils > ");

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
    let ret  = utils_mode_process_input_command(input_command);

    return ret;
}


fn utils_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    

    valid_commands.push("genip".to_string());



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
            utils_mode_quit();
        }
        if input_command == "utils".to_string()
        {
            println!("already in utils mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            utils_mode_help();
        }

        
        if input_command == "genip".to_string()
        {
            utils_mode_genip();
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


fn utils_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn utils_mode_help()
{
    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("---------");
    println!("genip   - generate IP addresses from 1.1.1.1 to 244.244.244.244");
    println!("asciit  - generate ASCII table");
    
    
    1;
}


fn utils_mode_genip()
{
    let starttime = Instant::now();
    let mut durationtime;
    let mut durationtimes = 0;
    let mut durationtimem = 0;
    let mut durationtimeh = 0;
    let mut total_ip_addresses: u64 = 0;

    

    let mut oct1: i32 = 1;
    let mut oct2: i32 = 1;
    let mut oct3: i32 = 1;
    let mut oct4: i32 = 1;
    
    loop {
        loop {
            durationtime = starttime.elapsed();
            durationtimes = durationtime.as_secs();
            durationtimem = durationtimes / 60;
            durationtimeh = durationtimem / 60;
            loop {
                loop {
                    print!("time elapsed: {:?}s, {}m, {}h - total ips: {} - ", durationtimes, durationtimem, durationtimeh, total_ip_addresses);
                    println!("ip: {}.{}.{}.{}", oct1, oct2, oct3, oct4);

                    if oct4 > 253 {
                        break;
                    }
                    else {
                        oct4 = oct4 + 1;
                    }
                }
                
                oct4 = 1;

                if oct3 > 253 {
                    break;
                }
                else {
                    oct3 = oct3 + 1;
                }
            }
            oct3 = 1;
            if oct2 > 253 {
                break;
            }
            else {
                oct2 = oct2 + 1;
            }
            total_ip_addresses = total_ip_addresses+(255 * 255);
        }
        oct2 = 1;
        if oct1 > 253 {
            break;
        }
        else {
            oct1 = oct1 + 1;
        }
    }


    1;
}