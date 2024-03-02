use std::io::{
      self,
      Read,
};



const REQUIRES_ARGUMENT: [&str; 4] = ["load", "jeq", "jmp", "jne"];






fn instruction_with_argument(
      instruction: String,
      argument: String,
      stack: &mut Vec<u8>,
      ip: &mut usize,
      labels: &Vec<(String, usize)>,
)
{
      match instruction.as_str() {
            "load" => {
                  if argument.starts_with('"') {
                        for c in argument.chars().rev().skip(1).take_while(|c| *c != '"') {
                              stack.push(c as u8);
                        }
                  }
                  else {
                        stack.push(argument.parse::<u8>().unwrap());
                  }
            },
            "jmp" => {
                  for (label, i) in labels {
                        if argument == *label {
                              *ip = *i;
                        }
                  }
            },
            "jeq" => {
                  if stack.pop().unwrap() != 0 {
                        for (label, i) in labels {
                              if argument == *label {
                                    *ip = *i;
                              }
                        }
                  }
            },
            "jne" => {
                  if stack.pop().unwrap() == 0 {
                        for (label, i) in labels {
                              if argument == *label {
                                    *ip = *i;
                              }
                        }
                  }
            },
            _ => {},
      }
}


fn instruction(instruction: String, stack: &mut Vec<u8>)
{
      match instruction.as_str() {
            "add" => {
                  let sum = stack.pop().unwrap() + stack.pop().unwrap();
                  stack.push(sum);
            },
            "sub" => {
                  let first = stack.pop().unwrap();
                  let sum = stack.pop().unwrap() - first;
                  stack.push(sum);
            },
            "mul" => {
                  let sum = stack.pop().unwrap() * stack.pop().unwrap();
                  stack.push(sum);
            },
            "div" => {
                  let first = stack.pop().unwrap();
                  let sum = stack.pop().unwrap() / first;
                  stack.push(sum);
            },
            "read" => {
                  let mut buffer: [u8; 1] = [0];
                  io::stdin().read(&mut buffer).unwrap();
                  if buffer[0] != 0 {
                        stack.push(buffer[0]);
                  }
            },
            "outc" => {
                  let value = stack.pop().unwrap();
                  print!("{}", value as char);
            },
            "out" => {
                  let value = stack.pop().unwrap();
                  print!("{}", value);
            },
            "cmp" => {
                  let a = stack.pop().unwrap();
                  let b = stack.pop().unwrap();
                  if a == b {
                        stack.push(1);
                  }
                  else if a != b {
                        stack.push(0);
                  }
            },
            "copy" => {
                  let value = stack.pop().unwrap();
                  stack.push(value);
                  stack.push(value);
            },
            "halt" => {
                  std::process::exit(0);
            },
            _ => {},
      }
}


fn main()
{
      let source = vec![
            vec!["#printString"],
            vec!["outc"],
            vec!["copy"],
            vec!["load", "0"],
            vec!["cmp"],
            vec!["jne", "#printString"],
            vec!["jmp", "#end"],
            vec!["#main"],
            vec!["load", "\"Hello Wooorld!\n\0\""],
            vec!["jmp", "#printString"],
            vec!["#end"],
            vec!["halt"],
      ];

      let mut stack: Vec<u8> = Vec::new();
      let mut labels: Vec<(String, usize)> = Vec::new();
      let mut ip = 0;

      let mut found = false;
      let mut entry = 0;
      for label in source.iter() {
            if label[0].starts_with('#') {
                  labels.push((label[0].to_owned(), ip));
            }
            if label[0] == "#main" {
                  found = true;
                  entry = ip;
            }
            ip += 1;
      }

      ip = entry;

      if !found {
            eprintln!("No #main entry label found.");
            std::process::exit(1);
      }

      while ip < source.len() {
            let line = &source[ip];
            // println!("[ip: {}, exe: '{}']", ip, line.join(" "));
            let mut tokens = line.iter();
            let inst = tokens.next().unwrap();
            if REQUIRES_ARGUMENT.contains(inst) {
                  instruction_with_argument(
                        inst.to_string(),
                        tokens.next().expect("'{instruction}' requires argument").to_string(),
                        &mut stack,
                        &mut ip,
                        &labels,
                  );
            }
            else {
                  instruction(inst.to_string(), &mut stack);
            }
            ip += 1;
      }
}
