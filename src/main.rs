use std::{fs::File, io::Read};

fn main() {

    let contents = read_file();
    let commands_list = get_commands(&contents);

    let mut command_element: Vec<Command> = Vec::new();
    for item in commands_list {

        if item.contains("@"){
            command_element.push(Command::ACommand(item[1..].to_string()));
        }else if item.contains("(") {
            command_element.push(Command::LCommand(item[1..item.len()-1].to_string()));
        }else{
            command_element.push(Command::CCommand(item[..].to_string()));
        }
    }

    for item in command_element{
        print!("{:#?}", item);
    }


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
