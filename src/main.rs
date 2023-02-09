use std::{fs::File, io::Read, collections::HashMap};
use assembler_rust::config_element;

fn main() {

    let contents = read_file();
    let commands_list = get_commands(&contents);
    let commands_enum = get_command_enum(commands_list);

    // symbol table
    let dest_hashmap = config_element::get_dest_hashmap();

    let comp_hashmap = config_element::get_comp_hashmap();

    let jump_hashmap = config_element::get_jump_hashmap();

    let mut symbolr_str:Vec<String> = Vec::new();
    for i in 0..16 {
        let mut s:String = String::from("R");
        s += i.to_string().as_str();
        symbolr_str.push(s);
    }
    let symbol_hashmap = config_element::get_symbol_hashmap(&symbolr_str);
    let add_hashmap = insert_into_lcommand(&commands_enum, symbol_hashmap);

    let mut result = Vec::new();
    for command in &commands_enum{
        let mut i = 15;
        match command {
            Command::ACommand(s) => {
                let str = match s.parse::<u16>() {
                    Ok(n) => String::from(format!("{:>016b}", n)),
                    Err(_)=> {
                            match &add_hashmap.get(&s.as_str()){
                               Some(num) => String::from(format!("{:>016b}", num)),
                               _ => {
                                    i += 1;
                                    String::from(format!("{:>016b}",i))
                                    },
                            }
                        }
                    };
                result.push(str);
            },
            Command::CCommand(s) => {
                let str = match &s.find("="){
                    Some(n) =>  { let ss = match &s.find(";") {
                                 Some(n) =>
                                        String::from(format!("111{:>07b}{:>03b}{:>03b}",
                                        &comp_hashmap.get(&s[(n+1)..s.find(";").unwrap()]).unwrap(),
                                        &dest_hashmap.get(&s[..*n]).unwrap(),
                                        &jump_hashmap.get(&s[(s.find(";").unwrap()+1)..]).unwrap())),
                                 None =>
                                        String::from(format!("111{:>07b}{:>03b}000",
                                        &comp_hashmap.get(&s[(n+1)..]).unwrap(),
                                        &dest_hashmap.get(&s[..*n]).unwrap())),
                                    };
                                ss
                                },
                    None => {
                        String::from(format!("111{:>07b}000{:>03b}",
                                        &comp_hashmap.get(&s[..s.find(";").unwrap()]).unwrap(),
                                        &jump_hashmap.get(&s[(s.find(";").unwrap()+1)..]).unwrap()))
                            },
                        };
                result.push(str);

                },
            Command::LCommand(_) => (),
            }
        }

        for s in result{
            println!("{}", s);
        }
}


fn insert_into_lcommand<'a>(commands_enum: &'a Vec<Command>, mut symbol_hashmap: HashMap<&'a str, u16>) -> HashMap<&'a str, u16>{
    let mut i = 0;
    for command in commands_enum {
        if let Command::LCommand(s) = command {
            symbol_hashmap.insert(&s, i);
        }else {
            i += 1;
        }
    }
    symbol_hashmap
}

fn get_command_enum(command_list: Vec<&str>) -> Vec<Command>{
    let mut command_element: Vec<Command> = Vec::new();
    for item in command_list {

        if item.contains("@"){
            command_element.push(Command::ACommand(item[1..].to_string()));
        }else if item.contains("(") {
            command_element.push(Command::LCommand(item[1..item.len()-1].to_string()));
        }else{
            command_element.push(Command::CCommand(item[..].to_string()));
        }
    }
    command_element
}


#[derive(Debug)]
enum Command {
    ACommand(String),
    CCommand(String),
    LCommand(String),
}

fn read_file() -> String {
    let mut filepath = String::new();
    println!("Please input filepath.");
    std::io::stdin().read_line(&mut filepath).expect("not enough filepath.");
    let filepath = filepath.as_str().trim();

    let mut f = File::open(filepath).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Can not read contents.");
    contents
}

fn get_commands(contents: &String) -> Vec<&str> {
    let mut commands: Vec<&str> = Vec::new();
    'outer: for line in contents.lines() {

        if line.starts_with("/"){ continue; }

        for (i, c ) in line.chars().enumerate() {
            if c == '/' {
                commands.push(line[..i].trim());
                continue 'outer;
            }
        }
        let s = line[..].trim();
        if !s.is_empty(){
            commands.push(&line[..].trim());
        }
    }
    commands
}
