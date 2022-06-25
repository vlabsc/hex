// version 0.6

#![allow(warnings, unused)]

use std::any::type_name;
use std::io::{stdin,stdout,Write};
use std::time::{Duration, Instant};
use dns_lookup::lookup_host;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn cheat_sheets_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = cheat_sheets_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn cheat_sheets_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: cheat sheets > ");

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
    let ret  = cheat_sheets_mode_process_input_command(input_command);

    return ret;
}


fn cheat_sheets_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    valid_commands.push("cls".to_string());
    

    valid_commands.push("cheat sheets".to_string());
    valid_commands.push("nmap".to_string());
    valid_commands.push("nmap switches".to_string());
    
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
            cheat_sheets_mode_quit();
        }
        if input_command == "cheat sheets".to_string()
        {
            println!("already in cheat sheets mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            cheat_sheets_mode_help();
        }
        if input_command == "cls".to_string()
        {
            cheat_sheets_mode_clrscr();
        }

        if input_command == "nmap".to_string()
        {
            cheat_sheets_mode_nmap();
        }

        if input_command == "nmap switches".to_string()
        {
            cheat_sheets_mode_nmap_switches();
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


fn cheat_sheets_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn cheat_sheets_mode_help()
{
    println!("current mode: ports");
    println!("all cheat sheets in once place. Like nmap cheat sheet, metasploit cheat sheet, SQL injection cheat sheet; etc.");
    println!("");

    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("-------");
    println!("nmap           - display the nmap cheat sheet.");
    println!("nmap switches  - display most commonly used nmap switches.");
    
    
    1;
}



fn read_lines_from_a_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}


fn cheat_sheets_mode_nmap() -> i32
{
    println!("source: https://hackertarget.com/nmap-cheatsheet-a-quick-reference-guide/");
    println!("Scan a single IP                         - nmap 192.168.1.1");
    println!("Scan a host                              - namp www.example.com");
    println!("Scan a range of IPs                      - nmap 192.168.1.1-254");
    println!("Scan a subnet                            - nmap 192.168.1.0/24");
    println!("Scan targets from a text file            - nmap -iL list-of-ips.txt");
    println!("Scan a single Port                       - nmap -p 22 192.168.1.1");
    println!("Scan a range of ports                    - nmap -p 1-100 192.168.1.1");
    println!("Scan 100 most common ports (Fast)        - nmap -F 192.168.1.1");
    println!("Scan all 65535 ports                     - nmap -p- 192.168.1.1");
    println!("Scan using TCP connect                   - nmap -sT 192.168.1.1");
    println!("Scan using TCP SYN scan (default)        - nmap -sS 192.168.1.1");
    println!("Scan UDP ports                           - nmap -sU -p 123,161,162 192.168.1.1");
    println!("Scan selected ports (ignore discovery)   - nmap -Pn -F 192.168.1.1");
    println!("Detect OS and Services                   - nmap -A 192.168.1.1");
    println!("Standard service detection               - nmap -sV 192.168.1.1");
    println!("More aggressive Service Detection        - nmap -sV --version-intensity 5 192.168.1.1");
    println!("Lighter banner grabbing detection        - nmap -sV --version-intensity 0 192.168.1.1");
    println!("Save default output to file              - nmap -oN outputfile.txt 192.168.1.1");
    println!("Save results as XML                      - nmap -oX outputfile.xml 192.168.1.1");
    println!("Save results in a format for grep        - nmap -oG outputfile.txt 192.168.1.1");
    println!("Save in all formats                      - nmap -oA outputfile 192.168.1.1");

    return 1;
}

fn cheat_sheets_mode_nmap_switches() -> i32
{
    println!("source: https://www.stationx.net/nmap-cheat-sheet/");
    println!("-iL                       - Scan targets from a file.");
    println!("-iR                       - Scan 100 random hosts.");
    println!("--exclude                 - Exclude listed hosts.");
    println!("-sS                       - TCP SYN port scan (Default).");
    println!("-sT                       - TCP connect port scan.");
    println!("-sU                       - UDP port scan.");
    println!("-sA                       - TCP ACK port scan.");
    println!("-sW                       - TCP Window port scan.");
    println!("-sM                       - TCP Maimon port scan.");
    println!("-sL                       - No Scan. List targets only.");
    println!("-sn                       - Disable port scanning. Host discovery only.");
    println!("-Pn                       - Disable host discovery. Port scan only.");
    println!("-PS                       - Port 80 by default.");
    println!("-PA                       - TCP ACK discovery on port x."); 
    println!("-PU                       - UDP discovery on port x. .");
    println!("-PR                       - ARP discovery on local network.");
    println!("-n                        - Never do DNS resolution.");
    println!("-p                        - Port scan for port x.");
    println!("-p-                       - Port scan all ports.");
    println!("-F                        - Fast port scan (100 ports).");
    println!("-p-65535                  - Leaving off initial port in range makes the scan start at port 1.");
    println!("-p0                       - Leaving off end port in range makes the scan go through to port 65535.");
    println!("-sV                       - Attempts to determine the version of the service running on port.");
    println!("-sV --version-intensity   - Intensity level 0 to 9. Higher number increases possibility of correctness.");
    println!("-sV --version-light       - Enable light mode. Lower possibility of correctness. Faster.");
    println!("-sV --version-all         - Enable intensity level 9. Higher possibility of correctness. Slower.");
    println!("-A                        - Enables OS detection, version detection, script scanning, and traceroute.");
    println!("-O                        - Remote OS detection using TCP/IP stack fingerprinting.");
    println!("-O --osscan-limit         - If at least one open and one closed TCP port are not found it will not try OS detection against host.");
    println!("-O --osscan-guess         - Makes Nmap guess more aggressively.");
    println!("-O --max-os-tries         - Set the maximum number x of OS detection tries against a target.");
    println!("-A                        - Enables OS detection, version detection, script scanning, and traceroute.");
    println!("-T0                       - Paranoid (0) Intrusion Detection System evasion.");
    println!("-T1                       - Sneaky (1) Intrusion Detection System evasion.");
    println!("-T2                       - Polite (2) slows down the scan to useless bandwidth and use less target machine resources.");
    println!("-T3                       - Normal (3) which is default speed.");
    println!("-T4                       - Aggressive (4) speeds scans; assumes you are on a reasonably fast and reliable network.");
    println!("-T5                       - Insane (5) speeds scan; assumes you are on an extraordinarily fast network.");
    println!("-sC                       - Scan with default NSE scripts. Considered useful for discovery and safe.");
    println!("--script default          - Scan with default NSE scripts. Considered useful for discovery and safe.");
    println!("--script-args             - NSE script with arguments.");
    println!("-f                        - Requested scan (including ping scans) use tiny fragmented IP packets. Harder for packet filters.");
    println!("--mtu                     - Set your own offset size.");
    println!("-D                        - Send scans from spoofed IPs.");
    println!("-S                        - Scan Facebook from Microsoft (-e eth0 -Pn may be required).");
    println!("-g                        - Use given source port number.");
    println!("--proxies                 - Relay connections through HTTP/SOCKS4 proxies.");
    println!("--data-length             - Appends random data to sent packets.");
    println!("-oN normal.file           - Normal output to the file normal.file.");
    println!("-oX xml.file              - XML output to the file xml.file.");
    println!("-oG grep.file             - Grepable output to the file grep.file.");
    println!("-oA results               - Output in the three major formats at once.");
    println!("-oG                       - Grepable output to screen. -oN -, -oX - also usable.");
    println!("--append-output           - Append a scan to a previous scan file.");
    println!("-v                        - Increase the verbosity level (use -vv or more for greater effect).");
    println!("-d                        - Increase debugging level (use -dd or more for greater effect).");
    println!("--reason                  - Display the reason a port is in a particular state, same output as -vv.");
    println!("--open                    - Only show open (or possibly open) ports.");
    println!("--packet-trace            - Show all packets sent and received.");
    println!("--iflist                  - Shows the host interfaces and routes.");
    println!("--resume                  - Resume a scan.");
    println!("-6                        - Enable IPv6 scanning.");
    println!("-h                        - nmap help screen.");






    


    return 1;
}

fn cheat_sheets_mode_clrscr()
{
    print!("\x1B[2J");
    print!("\x1B[2J");

    //print!("{}[2J", 27 as char);
    print!("\x1B[2J\x1B[1;1H");

}




fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}