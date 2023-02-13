use std::{fs::File, io::{Read, Write}};
use assembler_rust::{config_element, command_tosttring::{self}};

fn main() {

    let contents = read_file();
    let commands_list = command_tosttring::get_commands(&contents);
    let commands_enum = command_tosttring::get_command_enum(commands_list);

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

    let mut add_hashmap = command_tosttring::insert_into_lcommand(&commands_enum, symbol_hashmap);

    let result = command_tosttring::command_tostring_list(
                                    &commands_enum,
                                    &mut add_hashmap,
                                    comp_hashmap,
                                    dest_hashmap,
                                    jump_hashmap
                                  );

    let mut outpath = String::new();
    println!("Please input output_path.");
    std::io::stdin().read_line(&mut outpath).expect("not found output_path.");
    let mut wf = File::create(outpath.trim()).expect("failed make file");

    for s in &result {
        wf.write(s.as_bytes()).expect("can not write file");
    }




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
