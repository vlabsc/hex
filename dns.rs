// version 0.6

#![allow(warnings, unused)]

use std::any::type_name;
use std::io::{stdin,stdout,Write};
use std::time::{Duration, Instant};
use dns_lookup::lookup_host;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn dns_mode_start_prompt_loop() -> i32
{   

    let mut ret = 0;

    loop 
    {
        ret = dns_mode_input_prompt();

        if ret == 99
        {
            return 1;
        }
    }


    return 1;
}



fn dns_mode_input_prompt() -> i32
{
    let mut input_command = String::new();

    println!("");
    print!("hex: dns > ");

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
    let ret  = dns_mode_process_input_command(input_command);

    return ret;
}


fn dns_mode_process_input_command(input_command: String) -> i32
{
    let mut valid_commands: Vec<String> = vec![];

    valid_commands.push("quit".to_string());
    valid_commands.push("exit".to_string());
    valid_commands.push("q".to_string());
    valid_commands.push("e".to_string());
    valid_commands.push("help".to_string());
    valid_commands.push("?".to_string());
    valid_commands.push("cls".to_string());
    

    valid_commands.push("dns".to_string());
    valid_commands.push("fowl".to_string());
    valid_commands.push("revl".to_string());
    valid_commands.push("dnsb100".to_string());
    valid_commands.push("dnsb500".to_string());
    valid_commands.push("dnsb1000".to_string());
    valid_commands.push("dnsb10000".to_string());



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
            dns_mode_quit();
        }
        if input_command == "dns".to_string()
        {
            println!("already in dns mode.");
        }
        if input_command == "?".to_string() || input_command == "help".to_string()
        {
            dns_mode_help();
        }
        if input_command == "cls".to_string()
        {
            dns_mode_clrscr();
        }

        if input_command == "fowl".to_string()
        {
            dns_mode_fowl();
        }
        if input_command == "revl".to_string()
        {
            dns_mode_revl();
        }
        if input_command == "dnsb100".to_string()
        {
            dns_mode_dnsb100();
        }
        if input_command == "dnsb500".to_string()
        {
            dns_mode_dnsb500();
        }
        if input_command == "dnsb1000".to_string()
        {
            dns_mode_dnsb1000();
        }
        if input_command == "dnsb10000".to_string()
        {
            dns_mode_dnsb10000();
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


fn dns_mode_quit()
{
    println!("quitting ....");
    std::process::exit(0);
}


fn dns_mode_help()
{
    println!("current mode: dns");
    println!("all doman name system functionos. Like do forward lookup, reverse lookups, subdomain bruteforcing; etc.");
    println!("");

    println!("common");
    println!("------");
    
    println!("help - displays this screen or list of commands available.");
    println!("helpers - displays list of helpers available under current mode.");
    println!("quit/exit/q/e - quits the program.");
    println!("");

    println!("helpers");
    println!("---------");
    println!("fowl       - do a forward lookup - find the IP address for a domain name.");
    println!("revl       - do a reverse lookup - find the domain name for an IP address.");
    println!("dnsb100    - do a dns brute force for a given domains. top 100 sub-domains used.");
    println!("dnsb500    - do a dns brute force for a given domains. top 500 sub-domains used.");
    println!("dnsb1000   - do a dns brute force for a given domains. top 1000 sub-domains used.");
    println!("dnsb10000  - do a dns brute force for a given domains. top 10000 sub-domains used.");

    
    
    1;
}

fn dns_mode_fowl() -> i32
{
    let mut hostname = String::new();

    print!("Enter the domain: ");
    let _=stdout().flush();
    stdin().read_line(&mut hostname).expect("error: unable to read user input");
    if let Some('\n')=hostname.chars().next_back() {
        hostname.pop();
    }
    if let Some('\r')=hostname.chars().next_back() {
        hostname.pop();
    }


    //let mut ips: Vec<std::net::IpAddr> = lookup_host(&hostname.to_string());

    if lookup_host(&hostname.to_string()).is_err() == false
    {
        let ips: Vec<std::net::IpAddr> = lookup_host(&hostname.to_string()).unwrap();
        //assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
        for ipaddr in ips
        {
            if ipaddr.is_ipv6() == true
            {
                println!("IPv6 address: {}", ipaddr);
            }
            else if ipaddr.is_ipv4() == true
            {
                println!("IPv4 address: {}", ipaddr);
            }
        }
    }

    else if lookup_host(&hostname.to_string()).is_err() == true
    {
        println!("error in lookup.");
    }
    return 1;
}

fn dns_mode_revl() -> i32
{


    return 1;
}



fn dns_mode_dnsb100() -> i32
{
    //TODO - LICENSE TO - https://github.com/rbsec/dnscan/blob/master/LICENSE

    let mut maindomain = String::new();
    let mut total_domains = 0;

    print!("Enter the domain: ");
    let _=stdout().flush();
    stdin().read_line(&mut maindomain).expect("error: unable to read user input");
    if let Some('\n')=maindomain.chars().next_back() {
        maindomain.pop();
    }
    if let Some('\r')=maindomain.chars().next_back() {
        maindomain.pop();
    }

    println!("enumerating 100 subdomains for {}...", maindomain);
    if let Ok(lines) = read_lines_from_a_file("dns100.txt")
    {
        for line in lines
        {
            if let Ok(subdomains) = line
            {
                //let mut wholedomain = subdomains.clone();
                //let mut wholedomain = format!("{}.{}", "Goo", "gle.com");
                let mut wholedomain = format!("{}.{}", subdomains, maindomain);
                //println!("typ: {}", wholedomain);
            
                //if lookup_host(&maindomain.to_string()).is_err() == false
                if lookup_host(&wholedomain.to_string()).is_err() == false
                {
                    let ips: Vec<std::net::IpAddr> = lookup_host(&maindomain.to_string()).unwrap();
                    //assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
                    if ips.is_empty() == true
                    {

                    }
                    else if ips.is_empty() == false
                    {
                        total_domains = total_domains + 1;
                        println!("{}) {}.{} -> valid", total_domains, subdomains, maindomain);
                    }
                }
                else if lookup_host(&wholedomain.to_string()).is_err() == true
                {
                    //println!("error in lookup.");
                    //println!("{}.{} -> not valid", subdomains, maindomain);
                }

                //let mut wholedomain = subdomains + "." + &maindomain;
                //println!("{}", wholedomain);
            }
        }

    }


    println!("100 subdomains queried for {}. total found subdomains: {}", maindomain, total_domains);
    return 1;
}

fn dns_mode_dnsb500() -> i32
{
    //TODO - LICENSE TO - https://github.com/rbsec/dnscan/blob/master/LICENSE
    let mut maindomain = String::new();
    let mut total_domains = 0;

    print!("Enter the domain: ");
    let _=stdout().flush();
    stdin().read_line(&mut maindomain).expect("error: unable to read user input");
    if let Some('\n')=maindomain.chars().next_back() {
        maindomain.pop();
    }
    if let Some('\r')=maindomain.chars().next_back() {
        maindomain.pop();
    }

    println!("enumerating 500 subdomains for {}...", maindomain);
    if let Ok(lines) = read_lines_from_a_file("dns500.txt")
    {
        for line in lines
        {
            if let Ok(subdomains) = line
            {
                //let mut wholedomain = subdomains.clone();
                //let mut wholedomain = format!("{}.{}", "Goo", "gle.com");
                let mut wholedomain = format!("{}.{}", subdomains, maindomain);
                //println!("typ: {}", wholedomain);
            
                //if lookup_host(&maindomain.to_string()).is_err() == false
                if lookup_host(&wholedomain.to_string()).is_err() == false
                {
                    let ips: Vec<std::net::IpAddr> = lookup_host(&maindomain.to_string()).unwrap();
                    //assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
                    if ips.is_empty() == true
                    {

                    }
                    else if ips.is_empty() == false
                    {
                        total_domains = total_domains + 1;
                        println!("{}) {}.{} -> valid", total_domains, subdomains, maindomain);
                    }
                }
                else if lookup_host(&wholedomain.to_string()).is_err() == true
                {
                    //println!("error in lookup.");
                    //println!("{}.{} -> not valid", subdomains, maindomain);
                }

                //let mut wholedomain = subdomains + "." + &maindomain;
                //println!("{}", wholedomain);
            }
        }

    }


    println!("500 subdomains queried for {}. total found subdomains: {}", maindomain, total_domains);
    return 1;
}

fn dns_mode_dnsb1000() -> i32
{
    //TODO - LICENSE TO - https://github.com/rbsec/dnscan/blob/master/LICENSE
    let mut maindomain = String::new();
    let mut total_domains = 0;

    print!("Enter the domain: ");
    let _=stdout().flush();
    stdin().read_line(&mut maindomain).expect("error: unable to read user input");
    if let Some('\n')=maindomain.chars().next_back() {
        maindomain.pop();
    }
    if let Some('\r')=maindomain.chars().next_back() {
        maindomain.pop();
    }

    println!("enumerating 1000 subdomains for {}...", maindomain);
    if let Ok(lines) = read_lines_from_a_file("dns1000.txt")
    {
        for line in lines
        {
            if let Ok(subdomains) = line
            {
                //let mut wholedomain = subdomains.clone();
                //let mut wholedomain = format!("{}.{}", "Goo", "gle.com");
                let mut wholedomain = format!("{}.{}", subdomains, maindomain);
                //println!("typ: {}", wholedomain);
            
                //if lookup_host(&maindomain.to_string()).is_err() == false
                if lookup_host(&wholedomain.to_string()).is_err() == false
                {
                    let ips: Vec<std::net::IpAddr> = lookup_host(&maindomain.to_string()).unwrap();
                    //assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
                    if ips.is_empty() == true
                    {

                    }
                    else if ips.is_empty() == false
                    {
                        total_domains = total_domains + 1;
                        println!("{:<width$}) {}.{} -> valid", total_domains, subdomains, maindomain, width = 3);
                    }
                }
                else if lookup_host(&wholedomain.to_string()).is_err() == true
                {
                    //println!("error in lookup.");
                    //println!("{}.{} -> not valid", subdomains, maindomain);
                }

                //let mut wholedomain = subdomains + "." + &maindomain;
                //println!("{}", wholedomain);
            }
        }

    }


    println!("1000 subdomains queried for {}. total found subdomains: {}", maindomain, total_domains);
    return 1;
}



fn dns_mode_dnsb10000() -> i32
{
    //TODO - LICENSE TO - https://github.com/rbsec/dnscan/blob/master/LICENSE
    let mut maindomain = String::new();
    let mut total_domains = 0;

    print!("Enter the domain: ");
    let _=stdout().flush();
    stdin().read_line(&mut maindomain).expect("error: unable to read user input");
    if let Some('\n')=maindomain.chars().next_back() {
        maindomain.pop();
    }
    if let Some('\r')=maindomain.chars().next_back() {
        maindomain.pop();
    }

    println!("enumerating 10000 subdomains for {}...", maindomain);
    if let Ok(lines) = read_lines_from_a_file("dns10000.txt")
    {
        for line in lines
        {
            if let Ok(subdomains) = line
            {
                //let mut wholedomain = subdomains.clone();
                //let mut wholedomain = format!("{}.{}", "Goo", "gle.com");
                let mut wholedomain = format!("{}.{}", subdomains, maindomain);
                //println!("typ: {}", wholedomain);
            
                //if lookup_host(&maindomain.to_string()).is_err() == false
                if lookup_host(&wholedomain.to_string()).is_err() == false
                {
                    let ips: Vec<std::net::IpAddr> = lookup_host(&maindomain.to_string()).unwrap();
                    //assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
                    if ips.is_empty() == true
                    {

                    }
                    else if ips.is_empty() == false
                    {
                        total_domains = total_domains + 1;
                        println!("{:<width$}) {}.{} -> valid", total_domains, subdomains, maindomain, width = 3);
                    }
                }
                else if lookup_host(&wholedomain.to_string()).is_err() == true
                {
                    //println!("error in lookup.");
                    //println!("{}.{} -> not valid", subdomains, maindomain);
                }

                //let mut wholedomain = subdomains + "." + &maindomain;
                //println!("{}", wholedomain);
            }
        }

    }


    println!("10000 subdomains queried for {}. total found subdomains: {}", maindomain, total_domains);
    return 1;
}


fn read_lines_from_a_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}


fn dns_mode_clrscr()
{
    print!("\x1B[2J");
    print!("\x1B[2J");

    //print!("{}[2J", 27 as char);
    print!("\x1B[2J\x1B[1;1H");

}




fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}