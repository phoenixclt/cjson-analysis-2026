use std::collections::HashMap;
use std::fmt;


#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}


impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}


pub struct JsonParser {
    input: String,
    pos: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> JsonParser {
        JsonParser {
            input: input.to_string(),
            pos: 0,
        }
    }

   
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input.chars().nth(self.pos).unwrap();
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> char {
        if self.pos < self.input.len() {
            self.input.chars().nth(self.pos).unwrap()
        } else {
            '\0'
        }
    }

    fn next(&mut self) -> char {
        if self.pos < self.input.len() {
            let c = self.input.chars().nth(self.pos).unwrap();
            self.pos += 1;
            c
        } else {
            '\0'
        }
    }
    fn parse_string(&mut self) -> Result<String, String> {
        if self.peek() != '"' {
            return Err("Expected string".to_string());
        }
        self.next(); 
        
        let mut result = String::new();
        
        while self.pos < self.input.len() {
            let c = self.next();
            if c == '"' {
                return Ok(result);
            }
            result.push(c);
        }
        
        Err("Unterminated string".to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        
        while self.pos < self.input.len() {
            let c = self.peek();
            if c.is_digit(10) || c == '-' {
                self.pos += 1;
            } else {
                break;
            }
        }
        

        if self.peek() == '.' {
            self.pos += 1;
            while self.pos < self.input.len() {
                let c = self.peek();
                if c.is_digit(10) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        
        let num_str: String = self.input.chars().skip(start).take(self.pos - start).collect();
        match num_str.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err("Invalid number".to_string()),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        if self.peek() != '[' {
            return Err("Expected array".to_string());
        }
        self.next(); 
        
        self.skip_whitespace();
        
        let mut arr = Vec::new();
        
        if self.peek() == ']' {
            self.next();
            return Ok(JsonValue::Array(arr));
        }
        
        loop {
            self.skip_whitespace();
            let value = self.parse_value()?;
            arr.push(value);
            
            self.skip_whitespace();
            
            if self.peek() == ']' {
                self.next();
                break;
            }
            
            if self.peek() != ',' {
                return Err("Expected comma or ]".to_string());
            }
            self.next();
        }
        
        Ok(JsonValue::Array(arr))
    }
    fn parse_object(&mut self) -> Result<JsonValue, String> {
        if self.peek() != '{' {
            return Err("Expected object".to_string());
        }
        self.next(); 
        
        self.skip_whitespace();
        
        let mut obj = HashMap::new();
        
        if self.peek() == '}' {
            self.next();
            return Ok(JsonValue::Object(obj));
        }
        
        loop {
            self.skip_whitespace();
            
 
            let key = self.parse_string()?;
            
            self.skip_whitespace();
            
            if self.peek() != ':' {
                return Err("Expected colon".to_string());
            }
            self.next();
            
            self.skip_whitespace();
            
   
            let value = self.parse_value()?;
            obj.insert(key, value);
            
            self.skip_whitespace();
            
            if self.peek() == '}' {
                self.next();
                break;
            }
            
            if self.peek() != ',' {
                return Err("Expected comma or }".to_string());
            }
            self.next();
        }
        
        Ok(JsonValue::Object(obj))
    }


    pub fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        
        let c = self.peek();
        
        match c {
            'n' => {
                // null
                self.next();
                if self.next() == 'u' && self.next() == 'l' && self.next() == 'l' {
                    Ok(JsonValue::Null)
                } else {
                    Err("Expected null".to_string())
                }
            }
            't' => {
                // true
                self.next();
                if self.next() == 'r' && self.next() == 'u' && self.next() == 'e' {
                    Ok(JsonValue::Bool(true))
                } else {
                    Err("Expected true".to_string())
                }
            }
            'f' => {
                // false
                self.next();
                if self.next() == 'a' && self.next() == 'l' && self.next() == 's' && self.next() == 'e' {
                    Ok(JsonValue::Bool(false))
                } else {
                    Err("Expected false".to_string())
                }
            }
            '"' => {
                // string
                let s = self.parse_string()?;
                Ok(JsonValue::String(s))
            }
            '[' => {
                // array
                self.parse_array()
            }
            '{' => {
                // object
                self.parse_object()
            }
            '-' | '0'..='9' => {
                // number
                let n = self.parse_number()?;
                Ok(JsonValue::Number(n))
            }
            _ => Err(format!("Unexpected character: {}", c)),
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        let result = self.parse_value()?;
        self.skip_whitespace();
        Ok(result)
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut parser = JsonParser::new(input);
    parser.parse()
}

fn main() {

    let json_str = r#"{"name": "张三", "age": 25, "is_student": true}"#;
    
    match parse_json(json_str) {
        Ok(json) => {
            println!("解析成功: {}", json);
        }
        Err(e) => {
            println!("解析失败: {}", e);
        }
    }
    

    let json_str2 = r#"[1, 2, 3, "hello", true]"#;
    
    match parse_json(json_str2) {
        Ok(json) => {
            println!("数组解析成功: {}", json);
        }
        Err(e) => {
            println!("数组解析失败: {}", e);
        }
    }
    

    let json_str3 = r#"{"user": {"name": "李四", "hobbies": ["读书", "游戏"]}}"#;
    
    match parse_json(json_str3) {
        Ok(json) => {
            println!("嵌套解析成功: {}", json);
        }
        Err(e) => {
            println!("嵌套解析失败: {}", e);
        }
    }
}
