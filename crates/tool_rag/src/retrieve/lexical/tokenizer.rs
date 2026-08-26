/*
 * Bento - a tool-relay hub for AI agents and tool hosts.
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

/// A-Z, a-z, 0-9
fn is_latin_alpha_or_num(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// CJK
fn is_cjk(c: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&c)
}

/// 词法通道的分析器：无词典规则分词。
pub(crate) struct LexicalTokenizer;

impl LexicalTokenizer {
    /// 把 `text` 切成词元。建倒排和解析查询共用。
    pub fn tokenize(text: &str) -> Vec<String> {
        if text.is_empty() {
            return Vec::new();
        }

        let mut result: Vec<String> = Vec::new();
        let mut latin_buffer: Vec<char> = Vec::new();
        let mut cjk_buffer: Vec<char> = Vec::new();

        for c in text.chars() {
            if is_latin_alpha_or_num(c) {
                Self::flush_cjk(&mut cjk_buffer, &mut result);
                latin_buffer.push(c);
            } else if is_cjk(c) {
                Self::flush_latin(&mut latin_buffer, &mut result);
                cjk_buffer.push(c);
            } else {
                Self::flush_latin(&mut latin_buffer, &mut result);
                Self::flush_cjk(&mut cjk_buffer, &mut result);
            }
        }

        Self::flush_latin(&mut latin_buffer, &mut result);
        Self::flush_cjk(&mut cjk_buffer, &mut result);

        result
    }

    fn flush_latin(buffer: &mut Vec<char>, result: &mut Vec<String>) {
        if buffer.is_empty() {
            return;
        }

        let chars = buffer.as_slice();
        let mut start = 0;

        for i in 1..chars.len() {
            if Self::match_latin_roles(chars, i) {
                let slice = &chars[start..i];
                result.push(slice.iter().collect::<String>().to_ascii_lowercase());
                start = i;
            }
        }

        // 将剩余的拉丁字符直接拼接
        let remain = &chars[start..];
        if !remain.is_empty() {
            result.push(remain.iter().collect::<String>().to_ascii_lowercase());
        }

        // 清空缓冲区
        buffer.clear();
    }

    fn flush_cjk(buffer: &mut Vec<char>, result: &mut Vec<String>) {
        if buffer.is_empty() {
            return;
        }

        if buffer.len() == 1 {
            result.push(buffer[0].to_string());
        } else {
            // 带重叠的bigram
            for window in buffer.windows(2) {
                result.push(format!("{}{}", window[0], window[1]));
            }
        }

        // 清空缓冲区
        buffer.clear();
    }

    fn match_latin_roles(buffer: &[char], end: usize) -> bool {
        if buffer.is_empty() {
            return false;
        }

        let prev = buffer[end - 1];
        let curr = buffer[end];

        // 1. 小写 -> 大写(createCube, CreateCube -> create | cube)
        if prev.is_ascii_lowercase() && curr.is_ascii_uppercase() {
            return true;
        }

        // 2. 连续大写后接小写(HTTPServer -> http | server)
        if prev.is_ascii_uppercase() && curr.is_ascii_uppercase() {
            if let Some(c) = buffer.get(end + 1) {
                return c.is_ascii_lowercase();
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::LexicalTokenizer;

    #[test]
    fn test_snake_case() {
        let text = r#"Create_Cube"#;
        let result = LexicalTokenizer::tokenize(text);
        assert_eq!(result, [r#"create"#, r#"cube"#])
    }

    #[test]
    fn test_camel_case() {
        let text = r#"createCube"#;
        let result = LexicalTokenizer::tokenize(text);
        assert_eq!(result, [r#"create"#, r#"cube"#])
    }

    #[test]
    fn test_uppercase() {
        let text = r#"HTTPServer"#;
        let result = LexicalTokenizer::tokenize(text);
        assert_eq!(result, [r#"http"#, r#"server"#])
    }

    #[test]
    fn test_cjk() {
        let text = r#"创建立方体"#;
        let result = LexicalTokenizer::tokenize(text);
        assert_eq!(result, [r#"创建"#, r#"建立"#, r#"立方"#, r#"方体"#])
    }

    #[test]
    fn test_mixed() {
        {
            let text = r#"猫。"#;
            let result = LexicalTokenizer::tokenize(text);
            assert_eq!(result, [r#"猫"#]);
        }

        {
            let text = r#"创建Cube网格"#;
            let result = LexicalTokenizer::tokenize(text);
            assert_eq!(result, [r#"创建"#, r#"cube"#, r#"网格"#])
        }
    }

    #[test]
    fn test_delimiter() {
        {
            let text = r#"blender.createCube"#;
            let result = LexicalTokenizer::tokenize(text);
            assert_eq!(result, [r#"blender"#, r#"create"#, r#"cube"#])
        }

        {
            let text = r#"FBX-Export_Assert"#;
            let result = LexicalTokenizer::tokenize(text);
            assert_eq!(result, [r#"fbx"#, r#"export"#, r#"assert"#])
        }

        {
            let text = r#"animation animation"#;
            let result = LexicalTokenizer::tokenize(text);
            assert_eq!(result, [r#"animation"#, r#"animation"#])
        }
    }
}
