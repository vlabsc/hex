// version 0.6

#![allow(warnings, unused)]

use std::io::{stdin,stdout,Write};

mod passwd;
mod ports;
mod protocols;
mod utils;
mod dns;
mod cheat_sheets;
mod hunts;

fn main() 
{
    println!("Hello, world!");

    start();
}


fn start()
{
    print_ascii_logo();
    print_welcome_banner();

    main_mode_start_prompt_loop();
}

fn print_ascii_logo()
{
println!("    ___ __________________  ___             ___ ___                __    ___________                      __                .__ ___.         ____  ___");
println!("    /   |   \\_   _____/\\   \\/  /            /   |   \\_____    ____ |  | __\\_   _____/______  ______      _/  |_  ____   ____ |  |\\_ |__   ____\\   \\/  /");
println!("   /    ~    \\    __)_  \\     /    ______  /    ~    \\__  \\ _/ ___\\|  |/ / |    __)_\\_  __ \\/  ___/      \\   __\\/  _ \\ /  _ \\|  | | __ \\ /  _ \\\\     / ");
println!("   \\    Y    /        \\ /     \\   /_____/  \\    Y    // __ \\\\  \\___|    <  |        \\|  | \\/\\___ \\        |  | (  <_> |  <_> )  |_| \\_\\ (  <_> )     \\ ");
println!("    \\___|_  /_______  //___/\\  \\            \\___|_  /(____  /\\___  >__|_ \\/_______  /|__|  /____  >       |__|  \\____/ \\____/|____/___  /\\____/___/\\  \\");
println!("          \\/        \\/       \\_/                  \\/      \\/     \\/     \\/        \\/            \\/                                    \\/            \\_/");
}


fn print_welcome_banner()
{
    println!("Starting HEX - HackErs toolboX");
    println!("version 0.6");
    println!("the hackers one stop for all - references, small scans, calculators; etc.")
    
}


fn main_mode_start_prompt_loop()
{   
    loop 
    {
        main_mode_input_prompt();
    }
}



fn main_mode_input_prompt()
{
    let mut input_command = String::new();

    println!("");
    print!("hex: > ");

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
    main_mode_process_input_command(input_command);
}


fn main_mode_process_input_command(input_command: String)
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    
    valid_commands.push("..".to_string());

    valid_commands.push("modes".to_string());
    valid_commands.push("passwd".to_string());
    valid_commands.push("ports".to_string());
    valid_commands.push("protocols".to_string());
    valid_commands.push("tools".to_string());
    valid_commands.push("commands".to_string());
    valid_commands.push("shellcodes".to_string());
    valid_commands.push("exploits".to_string());
    valid_commands.push("utils".to_string());
    valid_commands.push("http".to_string());
    valid_commands.push("run commands".to_string());
    valid_commands.push("cheat sheets".to_string());
    valid_commands.push("dns".to_string());
    valid_commands.push("hunts".to_string());
    
    
    
    
    

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
        if input_command == "quit".to_string() || input_command == "exit".to_string() || 
        input_command == "q".to_string() || input_command == "e".to_string() || 
        input_command == "..".to_string()
        {
            main_mode_quit();
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            main_mode_help();
        }
        if input_command == "modes".to_string()
        {
            main_mode_modes();
        }        

        if input_command == "passwd".to_string()
        {
            main_mode_to_passwd_mode();
        }
        if input_command == "ports".to_string()
        {
            main_mode_to_ports_mode();
        }
        if input_command == "protocols".to_string()
        {
            main_mode_to_protocols_mode();
        }
        if input_command == "tools".to_string()
        {
            main_mode_to_tools_mode();
        }
        
        if input_command == "utils".to_string()
        {
            main_mode_to_utils_mode();
        }

        if input_command == "dns".to_string()
        {
            main_mode_to_dns_mode();
        }

        if input_command == "cheat sheets".to_string()
        {
            main_mode_to_cheat_sheets_mode();
        }

        if input_command == "hunts".to_string()
        {
            main_mode_to_hunts_mode();
        }
    }
    if bool_is_valid_command == false
    {
        println!("invalid command");
    }
}

fn main_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}

fn main_mode_help()
{
    println!("common");
    println!("------");
    
    /*
    old one
    println!("help - displays this screen or list of commands available.");
    println!("modes - displays list of modes available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");
    println!("modes");
    println!("-----");
    println!("passwd - all services related to passwords. Like ispasswordstrong, find hash; etc.");
    println!("ports - all services related to network ports. Like know all about network port numbers, port to application, servivce to port; etc.");
    println!("apis - all services related to windows APIs. Like know about various commonly used APIs, it's arguments, return values; etc.");
    println!("protocols - all services related to protocols. Like know about various protocols, RFC, protocal headers; etc.");
    println!("tools - all services related to tools. Like knowing about various tools - tcptraceroute, nmap, metasploit; etc.");
    println!("commands - all about commands. Like nmap commands, metasploit commands, smb commands; etc.");
    println!("shellcodes - all about commands. Like NOP codes, shellcode size, generate shellcode; etc.");
    println!("exploits - all about commands. Like searching exploits, compiling exploits, running exploits; etc.");
    println!("utils - all about utility commands. like generate ASCII table, find hash; etc.");
    println!("http - all about HTTP. Like know about HTTP commands, HTTP RFC; etc.");
    println!("run commands - query local OS. Like query local IP address, clear ARP cache; etc.");
    println!("cheat sheets - all cheat sheets in once place. Like nmap cheat sheet, metasploit cheat sheet, SQL injection cheat sheet; etc.");
    println!("dns - all doman name system functionos. Like do forward lookup, reverse lookups, subdomain bruteforcing; etc.");
    println!("hunts - various tools to hunt specific things. Like find out internal systems running RDP(3389), find out printers, find out WiFi access points; etc.");
    println!("wireless - all wireless related things. Like find out find out WiFi access points; etc.");
    println!("calc - do all kind of calculations. Like hex additions, binary conversions, address calculation, offset calculation; etc.");
    println!("credits - list of websites and people. for all those who have given content, resources, ideas, suggesstions; etc.");
    println!("mitre att4ck - Access the MITRE ATT4CK table. Get to know about Initial Access, Execution, Persistence; etc.");
    old one
    */

    println!("help - displays this screen or list of commands available.");
    println!("modes - displays list of modes available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");
    println!("modes");
    println!("-----");
    println!("apis           - all services related to windows APIs. Like know about various commonly used APIs, it's arguments, return values; etc.");
    println!("commands       - all about commands. Like nmap commands, metasploit commands, smb commands; etc.");
    println!("cheat sheets   - all cheat sheets in once place. Like nmap cheat sheet, metasploit cheat sheet, SQL injection cheat sheet; etc.");
    println!("calc           - do all kind of calculations. Like hex additions, binary conversions, address calculation, offset calculation; etc.");
    println!("credits        - list of websites and people. for all those who have given content, resources, ideas, suggesstions; etc.");
    println!("dns            - all doman name system functionos. Like do forward lookup, reverse lookups, subdomain bruteforcing; etc.");
    println!("exploits       - all about commands. Like searching exploits, compiling exploits, running exploits; etc.");
    println!("hunts          - various tools to hunt specific things. Like find out internal systems running RDP(3389), find out printers, WiFi access points; etc.");
    println!("http           - all about HTTP. Like know about HTTP commands, HTTP RFC; etc.");
    println!("mitre att4ck   - Access the MITRE ATT4CK table. Get to know about Initial Access, Execution, Persistence; etc.");
    println!("passwd         - all services related to passwords. Like ispasswordstrong, find hash; etc.");
    println!("ports          - all services related to network ports. Like know all about network port numbers, port to application, servivce to port; etc.");
    println!("protocols      - all services related to protocols. Like know about various protocols, RFC, protocal headers; etc.");
    println!("run commands   - query local OS. Like query local IP address, clear ARP cache; etc.");
    println!("tools          - all services related to tools. Like knowing about various tools - tcptraceroute, nmap, metasploit; etc.");
    println!("shellcodes     - all about commands. Like NOP codes, shellcode size, generate shellcode; etc.");
    println!("utils          - all about utility commands. like generate ASCII table, find hash; etc.");
    println!("wireless       - all wireless related things. Like find out find out WiFi access points; etc.");
 
    1;
}


fn main_mode_modes()
{

    println!("apis           - all services related to windows APIs. Like know about various commonly used APIs, it's arguments, return values; etc.");
    println!("commands       - all about commands. Like nmap commands, metasploit commands, smb commands; etc.");
    println!("cheat sheets   - all cheat sheets in once place. Like nmap cheat sheet, metasploit cheat sheet, SQL injection cheat sheet; etc.");
    println!("calc           - do all kind of calculations. Like hex additions, binary conversions, address calculation, offset calculation; etc.");
    println!("credits        - list of websites and people. for all those who have given content, resources, ideas, suggesstions; etc.");
    println!("dns            - all doman name system functionos. Like do forward lookup, reverse lookups, subdomain bruteforcing; etc.");
    println!("exploits       - all about commands. Like searching exploits, compiling exploits, running exploits; etc.");
    println!("hunts          - various tools to hunt specific things. Like find out internal systems running RDP(3389), find out printers, WiFi access points; etc.");
    println!("http           - all about HTTP. Like know about HTTP commands, HTTP RFC; etc.");
    println!("mitre att4ck   - Access the MITRE ATT4CK table. Get to know about Initial Access, Execution, Persistence; etc.");
    println!("passwd         - all services related to passwords. Like ispasswordstrong, find hash; etc.");
    println!("ports          - all services related to network ports. Like know all about network port numbers, port to application, servivce to port; etc.");
    println!("protocols      - all services related to protocols. Like know about various protocols, RFC, protocal headers; etc.");
    println!("run commands   - query local OS. Like query local IP address, clear ARP cache; etc.");
    println!("tools          - all services related to tools. Like knowing about various tools - tcptraceroute, nmap, metasploit; etc.");
    println!("shellcodes     - all about commands. Like NOP codes, shellcode size, generate shellcode; etc.");
    println!("utils          - all about utility commands. like generate ASCII table, find hash; etc.");
    println!("wireless       - all wireless related things. Like find out find out WiFi access points; etc.");
 
    /*
    println!("passwd - all services related to passwords. Like ispasswordstrong, find hash; etc.");
    println!("ports - all services related to network ports.");
    println!("apis - all services related to windows APIs.");
    println!("protocols - all services related to protocols.");
    println!("tools - all services related to tools.");
    */
    1;
}


fn main_mode_to_passwd_mode()
{
    println!("changing to passwd mode...");
    passwd::paswd_mode_start_prompt_loop();
    //global_prompt_mode.mode = "passwd".to_string();
    1;
}


fn main_mode_to_ports_mode()
{
    println!("changing to ports mode...");
    ports::ports_mode_start_prompt_loop();

    //global_prompt_mode.mode = "passwd".to_string();
    1;
}


fn main_mode_to_protocols_mode()
{
    println!("changing to protocols mode...");
    protocols::protocols_mode_start_prompt_loop();

    //global_prompt_mode.mode = "passwd".to_string();
    1;
}

fn main_mode_to_tools_mode()
{
    println!("changing to tools mode...");
    //tools::

    //global_prompt_mode.mode = "passwd".to_string();
    1;
}

fn main_mode_to_utils_mode()
{
    println!("changing to utils mode...");
    utils::utils_mode_start_prompt_loop();

    1;
}


fn main_mode_to_dns_mode()
{
    println!("changing to dns mode...");
    dns::dns_mode_start_prompt_loop();
}


fn main_mode_to_cheat_sheets_mode()
{
    println!("changing to cheat sheets mode...");
    cheat_sheets::cheat_sheets_mode_start_prompt_loop();
}


fn main_mode_to_hunts_mode()
{
    println!("changing to hunts mode...");
    hunts::hunts_mode_start_prompt_loop();
}

