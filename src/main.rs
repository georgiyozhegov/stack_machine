use std::cmp::max;
use std::io::{
      self,
      Read,
};
use std::process::exit;
use std::{
      env,
      fs,
};



const LABEL_PREFIX: char = '#';
const START_LABEL: &str = "#start";


// conditional jumps
const FLAG_EQ: u8 = 0;
const FLAG_GR: u8 = 1;
const FLAG_LE: u8 = 2;
// 'put' flags
const FLAG_FMT_NUMBER: u8 = 0;
const FLAG_FMT_CHAR: u8 = 1;
const FLAG_FMT_BITS: u8 = 2;
// 'read' flags
const FLAG_READ_CHAR: u8 = 0;
const FLAG_READ_STR: u8 = 1;


macro_rules! error {
      ($message:expr) => {
            eprintln!("\x1b[31mERROR |\x1b[m {}", $message);
            exit(1);
      };
      ($message:expr, $source:expr) => {
            eprintln!("\x1b[31mERROR |\x1b[m {}", $message);
            let (at, line) = $source.at();
            eprintln!("      \x1b[31m|\x1b[m line {}: '{}'", at, line);
            exit(1);
      };
      ($message:expr, $hint:expr, $source:expr) => {
            eprintln!("\x1b[31mERROR |\x1b[m {}", $message);
            let (at, line) = $source.at();
            eprintln!("      \x1b[31m|\x1b[m line {}: '{}'", at, line);
            eprintln!("\x1b[32mHINT  |\x1b[m {}", $hint);
            exit(1);
      };
}


fn is_string(parameter: &String) -> bool
{
      parameter.starts_with('"') && parameter.ends_with('"')
}

fn parse_string(parameter: String) -> Vec<u8>
{
      parameter
            .chars()
            .skip(1)
            .take(parameter.len() - 2)
            .map(|c| c as u8)
            .collect::<Vec<_>>()
}


fn is_integer(parameter: &String) -> bool
{
      for c in parameter.chars() {
            if !c.is_ascii_digit() {
                  return false;
            }
      }
      true
}

fn parse_integer(parameter: String) -> u8
{
      if let Ok(value) = parameter.parse() {
            value
      }
      else {
            error!("invalid integer format");
      }
}


fn read_char() -> Option<u8>
{
      let mut buffer: [u8; 1] = [0];
      if io::stdin().read_exact(&mut buffer).is_ok() {
            Some(buffer[0])
      }
      else {
            None
      }
}

fn read_string() -> Option<String>
{
      let mut string = String::new();
      if io::stdin().read_line(&mut string).is_ok() {
            Some(string)
      }
      else {
            None
      }
}


#[derive(Debug)]
struct Stack
{
      stack: Vec<u8>,
}

impl Stack
{
      pub fn new() -> Self
      {
            Self {
                  stack: Vec::new(),
            }
      }

      pub fn top(&self) -> u8
      {
            if let Some(value) = self.stack.get(max(self.stack.len(), 1) - 1) {
                  value.clone()
            }
            else {
                  error!("attempt to get item from an empty stack");
            }
      }

      pub fn pop(&mut self) -> u8
      {
            if let Some(value) = self.stack.pop() {
                  value
            }
            else {
                  error!("attempt to pop item from an empty stack");
            }
      }

      pub fn push(&mut self, value: u8)
      {
            self.stack.push(value)
      }
}


struct Source
{
      source: Vec<Vec<String>>,
      labels: Vec<(String, usize)>,
      ip: usize,
}

impl Source
{
      pub fn new(source: Vec<Vec<String>>) -> Self
      {
            let labels = Self::labels(&source);
            Self {
                  source,
                  ip: Self::find_start(&labels),
                  labels,
            }
      }

      fn labels(source: &Vec<Vec<String>>) -> Vec<(String, usize)>
      {
            let mut labels = Vec::new();
            for (i, line) in source.iter().enumerate() {
                  if line.len() == 1 && line[0].starts_with(LABEL_PREFIX) {
                        labels.push((line[0].clone(), i));
                  }
            }
            labels
      }

      fn find_start(labels: &Vec<(String, usize)>) -> usize
      {
            for (label, i) in labels {
                  if label == START_LABEL {
                        return *i;
                  }
            }
            error!("entry point '#start' not found");
      }

      pub fn mut_ip(&mut self) -> &mut usize
      {
            &mut self.ip
      }

      pub fn at(&self) -> (usize, String)
      {
            (self.ip + 1, self.source[self.ip].clone().join(" "))
      }
}

impl Iterator for Source
{
      type Item = Vec<String>;

      fn next(&mut self) -> Option<Self::Item>
      {
            self.ip += 1;
            if self.ip == self.source.len() {
                  return None;
            }
            Some(self.source[self.ip].clone())
      }
}


struct Machine
{
      stack: Stack,
      source: Source,
      halt: bool,
}

impl Machine
{
      pub fn new(stack: Stack, source: Source) -> Self
      {
            Self {
                  stack,
                  source,
                  halt: false,
            }
      }

      pub fn execute(&mut self)
      {
            while let Some(line) = self.source.next() {
                  if self.halt {
                        break;
                  }
                  self.execute_(line);
            }
      }

      // executes line
      fn execute_(&mut self, line: Vec<String>)
      {
            if line.len() == 1 && line[0].starts_with(LABEL_PREFIX) {
                  // skip labels
            }
            else if line.len() == 1 {
                  self.instruction(line[0].clone());
            }
            else if line.len() == 2 {
                  self.instruction_with_parameter(line[0].clone(), line[1].clone())
            }
            else {
                  error!("invalid number of parameters", self.source);
            }
      }

      fn put(&mut self)
      {
            let format = self.stack.pop();
            let value = self.stack.top();
            if format == FLAG_FMT_CHAR {
                  print!("{}", value as char);
            }
            else if format == FLAG_FMT_NUMBER {
                  print!("{}", value);
            }
            else if format == FLAG_FMT_BITS {
                  print!("{:08b}", value);
            }
            else {
                  error!("format flag is not valid", "you can set format mode to 'number' (0), 'char' (1) or 'bits' (2) via \x1b[33m'load <mode>'\x1b[m", self.source);
            };
      }

      fn read(&mut self)
      {
            let mode = self.stack.pop();
            if mode == FLAG_READ_CHAR {
                  if let Some(value) = read_char() {
                        self.stack.push(value);
                  }
                  else {
                        error!("failed to read char from stdin", self.source);
                  }
            }
            else if mode == FLAG_READ_STR {
                  if let Some(value) = read_string() {
                        // Reversed strings are easier to manipulate
                        for value in value.chars().rev() {
                              self.stack.push(value as u8);
                        }
                  }
                  else {
                        error!("failed to read string from stdin", self.source);
                  }
            }
            else {
                  error!("read flag is not valid", "you can set read mode to 'char' (0) or 'string' (1) via \x1b[33m'load <mode>'\x1b[m", self.source);
            }
      }

      fn compare(&mut self)
      {
            let a = self.stack.pop();
            let b = self.stack.top();
            self.stack.push(if a > b {
                  FLAG_GR
            }
            else if a < b {
                  FLAG_LE
            }
            else {
                  FLAG_EQ
            });
      }

      fn jump(&mut self, parameter: String)
      {
            for (label, i) in self.source.labels.clone() {
                  if parameter == label {
                        *self.source.mut_ip() = i;
                  }
            }
      }

      fn instruction(&mut self, instruction: String)
      {
            match instruction.as_str() {
                  "add" => {
                        let result = self.stack.pop() + self.stack.pop();
                        self.stack.push(result);
                  },
                  "sub" => {
                        let mut result = self.stack.pop();
                        result = self.stack.pop() - result;
                        self.stack.push(result);
                  },
                  "mul" => {
                        let result = self.stack.pop() * self.stack.pop();
                        self.stack.push(result);
                  },
                  "div" => {
                        let mut result = self.stack.pop();
                        result = self.stack.pop() / result;
                        self.stack.push(result);
                  },
                  "put" => self.put(),
                  "read" => self.read(),
                  "cmp" => self.compare(),
                  "and" => {
                        let result = self.stack.pop() & self.stack.pop();
                        self.stack.push(result);
                  },
                  "or" => {
                        let result = self.stack.pop() | self.stack.pop();
                        self.stack.push(result);
                  },
                  "xor" => {
                        let result = self.stack.pop() ^ self.stack.pop();
                        self.stack.push(result);
                  },
                  "not" => {
                        let result = !self.stack.pop();
                        self.stack.push(result);
                  },
                  "copy" => {
                        let value = self.stack.top();
                        self.stack.push(value);
                  },
                  "drop" => {
                        let _ = self.stack.pop();
                  },
                  "halt" => {
                        self.halt = true;
                  },
                  _ => {
                        error!("invalid instruction", self.source);
                  },
            }
      }

      fn instruction_with_parameter(&mut self, instruction: String, parameter: String)
      {
            match instruction.as_str() {
                  "load" => {
                        if is_string(&parameter) {
                              for value in parse_string(parameter) {
                                    self.stack.push(value);
                              }
                        }
                        else if is_integer(&parameter) {
                              self.stack.push(parse_integer(parameter));
                        }
                  },
                  "jmp" => self.jump(parameter),
                  "jeq" => {
                        if self.stack.top() == FLAG_EQ {
                              self.jump(parameter)
                        }
                  },
                  "jne" => {
                        let flag = self.stack.top();
                        if flag == FLAG_GR || flag == FLAG_LE {
                              self.jump(parameter)
                        }
                  },
                  "jgr" => {
                        if self.stack.top() == FLAG_GR {
                              self.jump(parameter);
                        }
                  },
                  "jle" => {
                        if self.stack.top() == FLAG_LE {
                              self.jump(parameter);
                        }
                  },
                  _ => {
                        error!("invalid instruction", self.source);
                  },
            }
      }
}


fn tokenize(line: String) -> Vec<String>
{
      let mut tokens = Vec::new();
      let mut buffer = String::new();
      let mut string = false;
      for c in line.chars() {
            if c == '"' {
                  string = !string;
            }
            if c.is_whitespace() && !string {
                  if !buffer.is_empty() {
                        tokens.push(buffer.clone());
                        buffer.clear();
                  }
            }
            else {
                  buffer.push(c);
            }
      }
      if !buffer.is_empty() {
            tokens.push(buffer);
      }
      tokens
}


fn path() -> String
{
      let mut args = env::args();
      args.next();
      let path = args.next();
      if path.is_none() {
            error!("program requires file path");
      }
      path.unwrap()
}


fn open(path: String) -> String
{
      if let Ok(file) = fs::read_to_string(path) {
            file
      }
      else {
            error!("failed to open file");
      }
}


fn main()
{
      let source = open(path());
      let source = source
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().take_while(|c| *c != ';').collect::<String>()) // skip comments
            .map(|l| tokenize(l.to_string()))
            .collect::<Vec<Vec<_>>>();

      let stack = Stack::new();
      let source = Source::new(source);
      let mut machine = Machine::new(stack, source);
      machine.execute();
}
