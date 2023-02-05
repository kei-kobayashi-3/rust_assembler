use std::{fs::File, io::Read, collections::HashMap};

fn main() {

    let contents = read_file();
    let commands_list = get_commands(&contents);
    let commands_enum = get_command_enum(commands_list);






    // symbol table
    let dest_mnemonic = vec!["","M","D","MD","A","AM","AD","AMD"];
    let dest_hashmap = get_dest_hashmap(dest_mnemonic);

    let comp_mnemonic = vec!["0","1","-1","D","A","!D","!A","-D","-A","D+1",
                                        "A+1","D-1","A-1","D+A","D-A","A-D","D&A","D|A",
                                        "M","!M","-M","M+1","M-1","D+M","D-M","M-D","D&M","D|M"];
    let comp_hashmap = get_comp_hashmap(comp_mnemonic);

    let jump_mnemonic = vec!["","JGT","JEQ","JGE","JLT","JNE","JLE","JMP"];
    let jump_hashmap = get_jump_hashmap(jump_mnemonic);

    let symbol_str = vec!["SP","LCL","ARG","THIS","THAT","SCREEN","KBD"];
    let mut symbolr_str:Vec<String> = Vec::new();
    for i in 0..16 {
        let mut s = String::from("R");
        s = s + i.to_string().as_str();
        symbolr_str.push(s);
    }
    let mut symbol_hashmap = get_symbol_hashmap(symbol_str, &symbolr_str);

    println!("{:#?}", commands_enum);



}

fn get_dest_hashmap(dest_mnemonic: Vec<&str>) -> HashMap<&str, u16>{
    let dest_hashmap: HashMap<_,_> = dest_mnemonic.into_iter().zip(0..=7).collect();
    dest_hashmap
}
fn get_comp_hashmap(comp_mnemonic: Vec<&str>) -> HashMap<&str, u16>{
    let comp_num: Vec<u16> = vec![42, 63, 58, 12, 48, 13, 49, 15,
    51, 31, 55, 14, 50, 2, 19, 7, 0, 21,
    112, 113, 115, 119, 114, 66, 83, 71, 64, 85];
    let comp_hashmap: HashMap<_,_> = comp_mnemonic.into_iter().zip(comp_num.into_iter()).collect();
    comp_hashmap
}
fn get_jump_hashmap(jump_mnemonic: Vec<&str>) -> HashMap<&str, u16>{
    let jump_hashmap: HashMap<_,_> = jump_mnemonic.into_iter().zip(0..=7).collect();
    jump_hashmap
}
fn get_symbol_hashmap<'a>(symbol_str: Vec<&'a str>, symbolr_str: &'a Vec<String>) -> HashMap<&'a str, u16>{
    let symbol_num: Vec<u16> = vec![0, 1, 2, 3, 4, 16384, 24576];
    let mut symbol_hashmap: HashMap<_,_> = symbol_str.into_iter().zip(symbol_num).collect();

    for i  in 0..16 {
        symbol_hashmap.insert(&symbolr_str[i], i as u16);
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
