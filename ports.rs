// version 0.6

#![allow(warnings, unused)]

use std::io::{stdin,stdout,Write};



pub fn ports_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = ports_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn ports_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: ports > ");

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
    let ret  = ports_mode_process_input_command(input_command);

    return ret;
}


fn ports_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    
    valid_commands.push("ports".to_string());
    valid_commands.push("lp".to_string());

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
            ports_mode_quit();
        }
        if input_command == "ports".to_string()
        {
            println!("already in ports mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            ports_mode_help();
        }

        if input_command == "..".to_string()
        {
            return 99;
        }

        if input_command == "lp".to_string()
        {
            ports_list_port_numbers();
        }


        //only numbers
        if input_command.chars().all(char::is_numeric) == true
        {
            let mut portnumber = input_command.parse::<usize>().unwrap();
            let mut portnumber_network_service_string: String = "unknown".to_string();
            portnumber_network_service_string = ports_list_service_name_for_port_number(portnumber);
            print!(" {} - {} ", portnumber, portnumber_network_service_string);
        }
    }
    if bool_is_valid_command == false
    {
        println!("invalid command");
    }

    return 1;
}


fn ports_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn ports_mode_help()
{
    println!("current mode: ports");
    println!("all services related to network ports. Like know all about network port numbers, port to application, servivce to port; etc.");
    println!("");

    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("---------");
    println!("lp             - list port numbers and associated services.");
    println!("ls             - list known services names and associated ports.");
    println!("pa             - list ports and associated applications.");
    println!("<port number>  - list the associated service and application for the service.");

    
    1;
}

fn ports_list_port_numbers()
{


    //println!("  port number      -         network service  ");
    //println!("----------------------------------------------");
    //println!(" port - network service");
    //println!("------------------------");


    let mut portnumber = 0;
    let mut portnumber_network_service_string: String = "unknown".to_string();

    
    loop {
        portnumber_network_service_string = ports_list_service_name_for_port_number(portnumber);
        
        //println!("   {:>width$}           -         {} ", portnumber, portnumber_network_service_string, width = 5);
        if portnumber_network_service_string.eq("unknown") == false
        {
            println!(" {}  - {} ", portnumber, portnumber_network_service_string);
        }

        if portnumber > 112
        {
            break;
        }
        else 
        {
            portnumber = portnumber + 1;
        }
    }

    println!("unknown ports are skipped.");
    
    1;
}


fn ports_list_service_name_for_port_number(portnum: usize) -> String
{
    let mut port_numbers: Vec<i32> = vec![];
    let mut port_numbers_service: Vec<String> = vec![];

    let mut portnumber = 0;
    portnumber = portnum;

    port_numbers.push(0);
    port_numbers.push(1);
    port_numbers.push(2);
    port_numbers.push(3);
    port_numbers.push(4);
    port_numbers.push(5);
    port_numbers.push(6);
    port_numbers.push(7);
    port_numbers.push(8);
    port_numbers.push(10);
    port_numbers.push(11);
    port_numbers.push(12);
    port_numbers.push(13);
    port_numbers.push(14);
    port_numbers.push(15);
    port_numbers.push(16);
    port_numbers.push(17);
    port_numbers.push(18);
    port_numbers.push(19);
    port_numbers.push(20);
    port_numbers.push(21);
    port_numbers.push(22);
    port_numbers.push(23);
    port_numbers.push(24);
    port_numbers.push(25);
    port_numbers.push(26);
    port_numbers.push(27);
    port_numbers.push(28);
    port_numbers.push(29);
    port_numbers.push(30);
    port_numbers.push(31);
    port_numbers.push(32);
    port_numbers.push(33);
    port_numbers.push(34);
    port_numbers.push(35);
    port_numbers.push(36);
    port_numbers.push(37);
    port_numbers.push(38);
    port_numbers.push(39);
    port_numbers.push(40);
    port_numbers.push(41);
    port_numbers.push(42);
    port_numbers.push(43);
    port_numbers.push(44);
    port_numbers.push(45);
    port_numbers.push(46);
    port_numbers.push(47);
    port_numbers.push(48);
    port_numbers.push(49);
    port_numbers.push(50);
    port_numbers.push(51);
    port_numbers.push(52);
    port_numbers.push(53);
    port_numbers.push(54);
    port_numbers.push(55);
    port_numbers.push(56);
    port_numbers.push(57);
    port_numbers.push(58);
    port_numbers.push(59);
    port_numbers.push(60);
    port_numbers.push(61);
    port_numbers.push(62);
    port_numbers.push(63);
    port_numbers.push(64);
    port_numbers.push(65);
    port_numbers.push(66);
    port_numbers.push(67);
    port_numbers.push(68);
    port_numbers.push(69);
    port_numbers.push(70);
    port_numbers.push(71);
    port_numbers.push(72);
    port_numbers.push(73);
    port_numbers.push(74);
    port_numbers.push(75);
    port_numbers.push(76);
    port_numbers.push(77);
    port_numbers.push(78);
    port_numbers.push(79);
    port_numbers.push(80);
    port_numbers.push(81);
    port_numbers.push(82);
    port_numbers.push(83);
    port_numbers.push(84);
    port_numbers.push(85);
    port_numbers.push(86);
    port_numbers.push(87);
    port_numbers.push(88);
    port_numbers.push(89);
    port_numbers.push(90);
    port_numbers.push(91);
    port_numbers.push(92);
    port_numbers.push(93);
    port_numbers.push(94);
    port_numbers.push(95);
    port_numbers.push(96);
    port_numbers.push(97);
    port_numbers.push(98);
    port_numbers.push(99);
    port_numbers.push(100);
    port_numbers.push(101);
    port_numbers.push(102);
    port_numbers.push(103);
    port_numbers.push(104);
    port_numbers.push(105);
    port_numbers.push(106);
    port_numbers.push(107);
    port_numbers.push(108);
    port_numbers.push(109);
    port_numbers.push(110);
    port_numbers.push(111);
    port_numbers.push(112);
    port_numbers.push(113);
    port_numbers.push(114);
    

    port_numbers_service.push("unknown".to_string());	
	port_numbers_service.push("TCPMUX".to_string());
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("Remote Job Entry".to_string());
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("Echo Protocol".to_string());
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("Discard Protocol".to_string());
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Active Users (systat service)".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Daytime Protocol".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Quote of the Day (QOTD)".to_string());   
    port_numbers_service.push("Message Send Protocol".to_string());   
    port_numbers_service.push("Character Generator Protocol (CHARGEN)".to_string());   
    port_numbers_service.push("File Transfer Protocol (FTP) data transfer".to_string());   
    port_numbers_service.push("File Transfer Protocol (FTP) control (command)".to_string());   
    port_numbers_service.push("Secure Shell (SSH), secure logins, file transfers (scp, sftp) and port forwarding".to_string());   
    port_numbers_service.push("Telnet".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Simple Mail Transfer Protocol (SMTP)".to_string());   
    port_numbers_service.push("unknown".to_string());
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Time Protocol".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Host Name Server Protocol".to_string());   
    port_numbers_service.push("WHOIS protocol".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("TACACS Login Host protocol".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Domain Name System (DNS)".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Dynamic Host Configuration Protocol (DHCP) ".to_string());   
    port_numbers_service.push("Dynamic Host Configuration Protocol (DHCP) ".to_string());   
    port_numbers_service.push("Trivial File Transfer Protocol (TFTP)".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Finger protocol".to_string());   
    port_numbers_service.push("Hypertext Transfer Protocol (HTTP)".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Kerberos".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Post Office Protocol, version 2 (POP2)".to_string());   
    port_numbers_service.push("Post Office Protocol, version 3 (POP3)".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("unknown".to_string());   
    port_numbers_service.push("Ident".to_string());   

    let mut portstring:String = "0".to_string();
    portstring = port_numbers_service[portnum].to_string();

    //println!("portstring: {}", portstring);

    //return "unknown".to_string();
    return portstring;
}
