// version 0.6

#![allow(warnings, unused)]

use std::io::{stdin,stdout,Write};



pub fn protocols_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = protocols_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn protocols_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: protocols > ");

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
    let ret  = protocols_mode_process_input_command(input_command);

    return ret;
}


fn protocols_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    
    valid_commands.push("eth".to_string());
    valid_commands.push("tcp".to_string());
    valid_commands.push("ip".to_string());
    valid_commands.push("icmp".to_string());
    valid_commands.push("udp".to_string());
    valid_commands.push("ip4".to_string());
    valid_commands.push("ip6".to_string());



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
            protocols_mode_quit();
        }
        if input_command == "protocols".to_string()
        {
            println!("already in protocols mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            protocols_mode_help();
        }

        
        if input_command == "eth".to_string()
        {
            protocols_show_eth_header();
        }
        if input_command == "ip4".to_string()
        {
            protocols_show_ip4_header();
        }
        if input_command == "ip6".to_string()
        {
            protocols_show_ip6_header();
        }

        if input_command == "tcp".to_string()
        {
            protocols_show_tcp_header();
        }
        if input_command == "udp".to_string()
        {
            protocols_show_udp_header();
        }
        if input_command == "icmp".to_string()
        {
            protocols_show_icmp_header();
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


fn protocols_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn protocols_mode_help()
{
    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("---------");
    println!("eth   - show the ethernet header.");
    println!("ip4   - show the ip v4 header.");
    println!("ip6   - show the ip v6 header.");
    println!("tcp   - show the tcp header.");
    println!("udp   - show the udp header.");
    println!("icmp  - show the icmp header.");

    
    1;
}

fn protocols_show_tcp_header()
{
    // TODO - LICENSE - source:http://www.luismg.com/protocol/

    println!("");
    println!("0                   1                   2                   3");
    println!("0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|          Source Port          |        Destination Port       |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|                        Sequence Number                        |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|                     Acknowledgment Number                     |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("| Offset|  Res. |     Flags     |             Window            |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|            Checksum           |         Urgent Pointer        |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|                    Options                    |    Padding    |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    1;
}

fn protocols_show_udp_header()
{
    // TODO - LICENSE - source:http://www.luismg.com/protocol/

    println!("");
    println!("0                   1                   2                   3");
    println!("0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1");
    println!("-----------------------------------------------------------------");
    println!("|          Source Port          |        Destination Port       |");
    println!("-----------------------------------------------------------------");
    println!("|             Length            |            Checksum           |");
    println!("-----------------------------------------------------------------");

}

fn protocols_show_eth_header()
{
    // TODO - LICENSE - source:http://www.luismg.com/protocol/

	println!(" 0                   1                   2                   3                   4");
	println!(" 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                                      Destination Address                                      |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                                         Source Address                                        |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|           EtherType           |                                                               |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+                                                               +");
	println!("|                                                                                               |");
	println!("+                                            Payload                                            +");
	println!("|                                                                                               |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
}

fn protocols_show_ip4_header()
{
    // TODO - LICENSE - source:http://www.luismg.com/protocol/

	println!("0                   1                   2                   3");
	println!("0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|Version|  IHL  |Type of Service|          Total Length         |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|         Identification        |Flags|      Fragment Offset    |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|  Time to Live |    Protocol   |         Header Checksum       |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                       Source Address                          |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                    Destination Address                        |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                    Options                    |    Padding    |");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    
}


fn protocols_show_ip6_header()
{

}


fn protocols_show_icmp_header()
{

    // TODO - LICENSE - source:http://www.luismg.com/protocol/

	println!("0                   1                   2                   3");
	println!("0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|      Type     |      Code     |            Checksum           |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
	println!("|                                                               |");
	println!("+                          Message Body                         +");
	println!("|                                                               |");
	println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");

}
