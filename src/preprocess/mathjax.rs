use errors::*;
use regex::{CaptureMatches, Captures, Regex};

pub fn prepare_math(s: &str) -> String {
    let res = replace_inline_math(s);
    let res = replace_block_math(&res);
    res
}

fn replace_inline_math(s: &str) -> String {
    // Find `\( ... \)`
    let regex = Regex::new(r"(?P<equation>\\\([^\r\n]+\\\))").expect("Regex is valid");
    regex.replace_all(s, r#"<div style="display:inline-block;" class="inline-math">$equation</div>"#).into_owned()
}

fn replace_block_math(s: &str) -> String {
    // Find `\[ ... \]` or `$$ ... $$`
    let regex = Regex::new(r"(?x)
        (?P<equation>
            (?:
                \\\[                # Match opening `\[`
                    [^\r\n]+        # Match any character that is not a new line
                \\\]                # Match closing `\]`
            ) | (?:                 # OR
                \$\$                # Match opening `$$`
                    [^\r\n]+        # Match any character that is not a new line
                \$\$                # Match closing `$$`
            )
        )").expect("Regex is valid");
    regex.replace_all(s, r#"<div class="block-math">$equation</div>"#).into_owned()
}


#[cfg(test)]
mod tests {
    use super::prepare_math;
    #[test]
    fn inline_math() {
        let s = r"Some inline \(a*b=c\) equation";
        assert_eq!(prepare_math(s), String::from(r#"Some inline <div style="display:inline-block;" class="inline-math">\(a*b=c\)</div> equation"#));
    }

    #[test]
    fn block_math() {
        let s = r"Some inline \[a*b=c\] equation";
        assert_eq!(prepare_math(s), String::from(r#"Some inline <div class="block-math">\[a*b=c\]</div> equation"#));
    }

    #[test]
    fn block_math_dollar() {
        let s = r"Some inline $$a*b=c$$ equation";
        assert_eq!(prepare_math(s), String::from(r#"Some inline <div class="block-math">$$a*b=c$$</div> equation"#));
    }
}