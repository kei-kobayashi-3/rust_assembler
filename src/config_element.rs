use std::collections::HashMap;


pub fn get_dest_hashmap() -> HashMap<&'static str, u16>{
  let dest_mnemonic = vec!["","M","D","MD","A","AM","AD","AMD"];
  let dest_hashmap: HashMap<_,_> = dest_mnemonic.into_iter().zip(0..=7).collect();
  dest_hashmap
}
pub fn get_comp_hashmap<'a>() -> HashMap<&'a str, u16>{
  let comp_mnemonic = vec!["0","1","-1","D","A","!D","!A","-D","-A","D+1",
                                      "A+1","D-1","A-1","D+A","D-A","A-D","D&A","D|A",
                                      "M","!M","-M","M+1","M-1","D+M","D-M","M-D","D&M","D|M"];
  let comp_num: Vec<u16> = vec![42, 63, 58, 12, 48, 13, 49, 15,
  51, 31, 55, 14, 50, 2, 19, 7, 0, 21,
  112, 113, 115, 119, 114, 66, 83, 71, 64, 85];
  let comp_hashmap: HashMap<_,_> = comp_mnemonic.into_iter().zip(comp_num.into_iter()).collect();
  comp_hashmap
}
pub fn get_jump_hashmap<'a>() -> HashMap<&'a str, u16>{
  let jump_mnemonic = vec!["","JGT","JEQ","JGE","JLT","JNE","JLE","JMP"];
  let jump_hashmap: HashMap<_,_> = jump_mnemonic.into_iter().zip(0..=7).collect();
  jump_hashmap
}
pub fn get_symbol_hashmap<'a>(symbolr_str: &'a Vec<String>) -> HashMap<&'a str, u16>{
  let symbol_str: Vec<&'a str> = vec!["SP","LCL","ARG","THIS","THAT","SCREEN","KBD"];

  let symbol_num: Vec<u16> = vec![0, 1, 2, 3, 4, 16384, 24576];
  let mut symbol_hashmap: HashMap<_,_> = symbol_str.into_iter().zip(symbol_num).collect();

  for i  in 0..16 {
      symbol_hashmap.insert(symbolr_str[i].as_str(), i as u16);
  }
  symbol_hashmap
}
