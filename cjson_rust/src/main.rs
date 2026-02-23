/// 表示 JSON 中可能的值类型
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// 从字符串解析 JSON 值（简化版）
    pub fn parse(input: &str) -> Result<JsonValue, String> {
        let mut chars = input.chars().peekable();
        Self::parse_value(&mut chars)
    }

    // 解析一个值（根据第一个字符决定类型）
    fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        // 跳过空白字符
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        match chars.peek() {
            Some('n') => Self::parse_null(chars),
            Some('t') => Self::parse_true(chars),
            Some('f') => Self::parse_false(chars),
            Some('"') => Self::parse_string(chars),
            Some('0'..='9') | Some('-') => Self::parse_number(chars),
            Some('[') => Self::parse_array(chars),
            Some('{') => Self::parse_object(chars),
            Some(c) => Err(format!("意外的字符: '{}'", c)),
            None => Err("输入为空".to_string()),
        }
    }

    // 解析 null
    fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        let expected = "null";
        for exp_c in expected.chars() {
            match chars.next() {
                Some(c) if c == exp_c => continue,
                _ => return Err(format!("期望 '{}', 但解析失败", expected)),
            }
        }
        Ok(JsonValue::Null)
    }

    // 解析 true
    fn parse_true(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        let expected = "true";
        for exp_c in expected.chars() {
            match chars.next() {
                Some(c) if c == exp_c => continue,
                _ => return Err(format!("期望 '{}', 但解析失败", expected)),
            }
        }
        Ok(JsonValue::Bool(true))
    }

    // 解析 false
    fn parse_false(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        let expected = "false";
        for exp_c in expected.chars() {
            match chars.next() {
                Some(c) if c == exp_c => continue,
                _ => return Err(format!("期望 '{}', 但解析失败", expected)),
            }
        }
        Ok(JsonValue::Bool(false))
    }

    // 解析字符串（不支持转义，只解析普通字符串）
    fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        // 跳过开头的引号
        if chars.next() != Some('"') {
            return Err("期望 '\"' 开始字符串".to_string());
        }

        let mut result = String::new();
        while let Some(&c) = chars.peek() {
            match c {
                '"' => {
                    chars.next(); // 跳过结束引号
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    // 简化：遇到反斜杠直接返回错误（不支持转义）
                    return Err("转义字符暂不支持".to_string());
                }
                _ => {
                    result.push(c);
                    chars.next();
                }
            }
        }
        Err("未闭合的字符串".to_string())
    }

    // 解析数字（简化：只支持整数和浮点数，不支持科学计数法）
    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        let mut num_str = String::new();
        // 允许负号开头
        if let Some(&'-') = chars.peek() {
            num_str.push('-');
            chars.next();
        }
        // 至少一位数字
        match chars.peek() {
            Some(c) if c.is_ascii_digit() => {
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            _ => return Err("期望数字".to_string()),
        }
        // 可选的小数部分
        if let Some(&'.') = chars.peek() {
            num_str.push('.');
            chars.next();
            // 小数点后至少一位数字
            match chars.peek() {
                Some(c) if c.is_ascii_digit() => {
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err("小数点后期望数字".to_string()),
            }
        }
        // 转换为 f64
        num_str.parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| format!("无效的数字格式: {}", num_str))
    }

    // 解析数组 [value1, value2, ...]
    fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        // 跳过 '['
        if chars.next() != Some('[') {
            return Err("期望 '[' 开始数组".to_string());
        }
        let mut array = Vec::new();
        // 跳过空白
        Self::skip_whitespace(chars);

        // 如果是空数组 "[]"
        if let Some(']') = chars.peek() {
            chars.next();
            return Ok(JsonValue::Array(array));
        }

        loop {
            // 解析值
            let value = Self::parse_value(chars)?;
            array.push(value);
            // 跳过空白
            Self::skip_whitespace(chars);

            match chars.peek() {
                Some(',') => {
                    chars.next();
                    Self::skip_whitespace(chars);
                    continue;
                }
                Some(']') => {
                    chars.next();
                    break;
                }
                _ => return Err("期望 ',' 或 ']'".to_string()),
            }
        }
        Ok(JsonValue::Array(array))
    }

    // 解析对象 {"key": value, ...}
    fn parse_object(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, String> {
        // 跳过 '{'
        if chars.next() != Some('{') {
            return Err("期望 '{{' 开始对象".to_string());
        }
        let mut object = Vec::new();
        Self::skip_whitespace(chars);

        if let Some('}') = chars.peek() {
            chars.next();
            return Ok(JsonValue::Object(object));
        }

        loop {
            // 解析键（必须是字符串）
            let key = match Self::parse_string(chars)? {
                JsonValue::String(s) => s,
                _ => return Err("对象的键必须是字符串".to_string()),
            };
            Self::skip_whitespace(chars);

            // 解析冒号
            match chars.next() {
                Some(':') => {}
                _ => return Err("期望 ':'".to_string()),
            }
            Self::skip_whitespace(chars);

            // 解析值
            let value = Self::parse_value(chars)?;
            object.push((key, value));
            Self::skip_whitespace(chars);

            match chars.peek() {
                Some(',') => {
                    chars.next();
                    Self::skip_whitespace(chars);
                    continue;
                }
                Some('}') => {
                    chars.next();
                    break;
                }
                _ => return Err("期望 ',' 或 '}'".to_string()),
            }
        }
        Ok(JsonValue::Object(object))
    }

    // 辅助函数：跳过空白字符
    fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }

    /// 将 JsonValue 转换为格式化的 JSON 字符串
    pub fn to_string_pretty(&self, indent: usize) -> String {
        self.to_string_inner(0, indent)
    }

    /// 非格式化输出（紧凑）
    pub fn to_string_compact(&self) -> String {
        self.to_string_inner(0, 0)
    }

    // 内部递归函数
    fn to_string_inner(&self, depth: usize, indent: usize) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),  // 简化：不处理转义
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else {
                    let mut result = String::from("[");
                    if indent > 0 {
                        result.push('\n');
                    }
                    for (i, val) in arr.iter().enumerate() {
                        if indent > 0 {
                            result.push_str(&" ".repeat(indent * (depth + 1)));
                        }
                        result.push_str(&val.to_string_inner(depth + 1, indent));
                        if i < arr.len() - 1 {
                            result.push(',');
                            if indent > 0 {
                                result.push('\n');
                            }
                        } else if indent > 0 {
                            result.push('\n');
                        }
                    }
                    if indent > 0 {
                        result.push_str(&" ".repeat(indent * depth));
                    }
                    result.push(']');
                    result
                }
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() {
                    "{}".to_string()
                } else {
                    let mut result = String::from("{");
                    if indent > 0 {
                        result.push('\n');
                    }
                    for (i, (key, val)) in obj.iter().enumerate() {
                        if indent > 0 {
                            result.push_str(&" ".repeat(indent * (depth + 1)));
                        }
                        result.push_str(&format!("\"{}\":", key));
                        if indent > 0 {
                            result.push(' ');
                        }
                        result.push_str(&val.to_string_inner(depth + 1, indent));
                        if i < obj.len() - 1 {
                            result.push(',');
                            if indent > 0 {
                                result.push('\n');
                            }
                        } else if indent > 0 {
                            result.push('\n');
                        }
                    }
                    if indent > 0 {
                        result.push_str(&" ".repeat(indent * depth));
                    }
                    result.push('}');
                    result
                }
            }
        }
    }
}

fn main() {
    let json_str = r#"
    {
        "name": "Tom",
        "age": 18,
        "hobbies": ["coding", "music"],
        "address": {
            "city": "Beijing",
            "zip": 100000
        }
    }"#;

    match JsonValue::parse(json_str) {
        Ok(value) => {
            println!("解析成功！");
            println!("格式化输出（缩进2空格）：");
            println!("{}", value.to_string_pretty(2));
            println!("\n紧凑输出：");
            println!("{}", value.to_string_compact());
        }
        Err(e) => println!("解析失败: {}", e),
    }
}