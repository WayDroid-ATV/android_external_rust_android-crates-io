// const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
// #[grammar = "grammar.pest"] can't be used here, because
// CARGO_MANIFEST_DIR is not set in soong environment.
#[grammar_inline = r##"
//! Pest grammar for handlebars templating
WHITESPACE = _{ " "|"\t"|"\n"|"\r" }
keywords = { "as" | "else" }

escape = @{ ("\\" ~ "{{" ~ "{{"?) | ("\\" ~ "\\"+ ~ &"{{") }
raw_text = ${ ( escape | (!"{{" ~ ANY) )+ }
raw_block_text = ${ ( escape | (!"{{{{" ~ ANY) )* }

literal = { string_literal |
            array_literal |
            object_literal |
            number_literal |
            null_literal |
            boolean_literal }

null_literal = @{ "null" ~ !symbol_char }
boolean_literal = @{ ("true"|"false") ~ !symbol_char }
number_literal = @{ "-"? ~ ASCII_DIGIT+ ~ "."? ~ ASCII_DIGIT* ~ ("E" ~ "-"? ~ ASCII_DIGIT+)? ~ !symbol_char }
json_char_double_quote = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}
json_char_single_quote = {
    !("'" | "\\") ~ ANY
    | "\\" ~ ("'" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}
string_inner_double_quote = @{ json_char_double_quote* }
string_inner_single_quote = @{ json_char_single_quote* }
string_literal = ${ ("\"" ~ string_inner_double_quote ~ "\"") | ("'" ~ string_inner_single_quote ~ "'") }
array_literal = { "[" ~ literal? ~ ("," ~ literal)* ~ "]" }
object_literal = { "{" ~ (string_literal ~ ":" ~ literal)?
                   ~ ("," ~ string_literal ~ ":" ~ literal)* ~ "}" }

symbol_char = _{ASCII_ALPHANUMERIC|"-"|"_"|"$"|'\u{80}'..'\u{7ff}'|'\u{800}'..'\u{ffff}'|'\u{10000}'..'\u{10ffff}'}
partial_symbol_char = _{ASCII_ALPHANUMERIC|"-"|"_"|'\u{80}'..'\u{7ff}'|'\u{800}'..'\u{ffff}'|'\u{10000}'..'\u{10ffff}'|"/"|"."}
path_char = _{ "/" }

identifier = @{ symbol_char+ }
partial_identifier = @{ partial_symbol_char+ | ("[" ~ ANY+ ~ "]") | ("'" ~ (!"'" ~ ("\\'" | ANY))+ ~ "'") }
reference = ${ path_inline }

name = _{ subexpression | reference }

helper_parameter = { !(keywords ~ !symbol_char) ~ (literal | reference | subexpression) }
hash = { identifier ~ "=" ~ helper_parameter }
block_param = { "as" ~ "|" ~ identifier ~ identifier? ~ "|"}
exp_line = _{ identifier ~ (hash|helper_parameter)* ~ block_param?}
partial_exp_line = _{ ((partial_identifier|name) ~ (hash|helper_parameter)*) }

subexpression = { "(" ~ ((identifier ~ (hash|helper_parameter)+) | reference)  ~ ")" }

leading_tilde_to_omit_whitespace = { "~" }
trailing_tilde_to_omit_whitespace = { "~" }

expression = { !(invert_tag|invert_chain_tag) ~ "{{" ~ leading_tilde_to_omit_whitespace? ~
              ((identifier ~ (hash|helper_parameter)+) | name )
              ~ trailing_tilde_to_omit_whitespace? ~ "}}" }
html_expression_triple_bracket_legacy = _{ "{{{" ~ leading_tilde_to_omit_whitespace? ~
                                           ((identifier ~ (hash|helper_parameter)+) | name ) ~
                                           trailing_tilde_to_omit_whitespace? ~ "}}}" }
html_expression_triple_bracket = _{ "{{" ~ leading_tilde_to_omit_whitespace? ~ "{" ~
                                              ((identifier ~ (hash|helper_parameter)+) | name ) ~
                                              "}" ~ trailing_tilde_to_omit_whitespace? ~ "}}" }

amp_expression = _{ "{{" ~ leading_tilde_to_omit_whitespace? ~ "&" ~ name ~
                       trailing_tilde_to_omit_whitespace? ~ "}}" }
html_expression = { (html_expression_triple_bracket_legacy | html_expression_triple_bracket)
                   | amp_expression }

decorator_expression = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "*" ~ exp_line ~
trailing_tilde_to_omit_whitespace? ~ "}}" }
partial_expression = { "{{" ~ leading_tilde_to_omit_whitespace? ~ ">" ~ partial_exp_line
                     ~ trailing_tilde_to_omit_whitespace? ~ "}}" }

invert_tag_item = { "else"|"^" }
invert_tag = { !escape ~ "{{" ~ leading_tilde_to_omit_whitespace? ~ invert_tag_item
             ~ trailing_tilde_to_omit_whitespace? ~ "}}" }
invert_chain_tag = { !escape ~ "{{" ~ leading_tilde_to_omit_whitespace? ~ invert_tag_item
                     ~ exp_line ~ trailing_tilde_to_omit_whitespace? ~ "}}" }
helper_block_start = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "#" ~ exp_line ~
                     trailing_tilde_to_omit_whitespace? ~ "}}" }
helper_block_end = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "/" ~ identifier ~
                   trailing_tilde_to_omit_whitespace? ~ "}}" }
helper_block = _{ helper_block_start ~ template ~
                  (invert_chain_tag ~ template)* ~ (invert_tag ~ template)? ~ helper_block_end }

decorator_block_start = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "#" ~ "*"
                        ~ exp_line ~ trailing_tilde_to_omit_whitespace? ~ "}}" }
decorator_block_end = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "/" ~ identifier ~
                        trailing_tilde_to_omit_whitespace? ~ "}}" }
decorator_block = _{ decorator_block_start ~ template ~
                     decorator_block_end }

partial_block_start = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "#" ~ ">"
                        ~ partial_exp_line ~ trailing_tilde_to_omit_whitespace? ~ "}}" }
partial_block_end = { "{{" ~ leading_tilde_to_omit_whitespace? ~ "/" ~ partial_identifier ~
                      trailing_tilde_to_omit_whitespace? ~ "}}" }
partial_block = _{ partial_block_start ~ template ~ partial_block_end }

raw_block_start = { "{{{{" ~ leading_tilde_to_omit_whitespace? ~ exp_line ~
                    trailing_tilde_to_omit_whitespace? ~ "}}}}" }
raw_block_end = { "{{{{" ~ leading_tilde_to_omit_whitespace? ~ "/" ~ identifier ~
                  trailing_tilde_to_omit_whitespace? ~ "}}}}" }
raw_block = _{ raw_block_start ~ raw_block_text ~ raw_block_end }

hbs_comment = { "{{!" ~ "--" ~ (!"--}}" ~ ANY)* ~ "--" ~ "}}" }
hbs_comment_compact = { "{{!" ~ (!"}}" ~ ANY)* ~ "}}" }

template = { (
            raw_text |
            expression |
            html_expression |
            helper_block |
            raw_block |
            hbs_comment |
            hbs_comment_compact |
            decorator_expression |
            decorator_block |
            partial_expression |
            partial_block )* }

parameter = _{ helper_parameter ~ EOI }
handlebars = _{ template ~ EOI }

/// json path visitor
/// Disallowed chars: Whitespace ! " # % & ' ( ) * + , . / ; < = > @ [ \ ] ^ ` { | } ~
path_id = @{ symbol_char+ }

path_raw_id = { (!"]" ~ ANY)* }
path_sep = _{ "/" | "." }
path_up = { ".." }
path_key = _{ "[" ~  path_raw_id ~ "]" }
path_root = { "@root" }
path_current = _{ "this" ~ path_sep | "./" }
path_item = _{ path_id|path_key }
path_local = { "@" }
path_inline = ${ path_current? ~ (path_root ~ path_sep)? ~ path_local? ~ (path_up ~ path_sep)*  ~ path_item ~ (path_sep ~  path_item)* }
path = _{ path_inline ~ EOI }
"##]
pub struct HandlebarsParser;

#[cfg(test)]
mod test {
    use super::{HandlebarsParser, Rule};
    use pest::Parser;

    macro_rules! assert_rule {
        ($rule:expr, $in:expr) => {
            assert_eq!(
                HandlebarsParser::parse($rule, $in)
                    .unwrap()
                    .last()
                    .unwrap()
                    .as_span()
                    .end(),
                $in.len()
            );
        };
    }

    macro_rules! assert_not_rule {
        ($rule:expr, $in:expr) => {
            assert!(
                HandlebarsParser::parse($rule, $in).is_err()
                    || HandlebarsParser::parse($rule, $in)
                        .unwrap()
                        .last()
                        .unwrap()
                        .as_span()
                        .end()
                        != $in.len()
            );
        };
    }

    macro_rules! assert_rule_match {
        ($rule:expr, $in:expr) => {
            assert!(HandlebarsParser::parse($rule, $in).is_ok());
        };
    }

    #[test]
    fn test_raw_text() {
        let s = [
            "<h1> helloworld </h1>    ",
            r"hello\{{world}}",
            r"hello\{{#if world}}nice\{{/if}}",
            r"hello \{{{{raw}}}}hello\{{{{/raw}}}}",
        ];
        for i in &s {
            assert_rule!(Rule::raw_text, i);
        }

        let s_not_escape = [r"\\{{hello}}"];
        for i in &s_not_escape {
            assert_not_rule!(Rule::raw_text, i);
        }
    }

    #[test]
    fn test_raw_block_text() {
        let s = "<h1> {{hello}} </h1>";
        assert_rule!(Rule::raw_block_text, s);
    }

    #[test]
    fn test_reference() {
        let s = vec![
            "a",
            "abc",
            "../a",
            "a.b",
            "@abc",
            "a.[abc]",
            "aBc.[abc]",
            "abc.[0].[nice]",
            "some-name",
            "this.[0].ok",
            "this.[$id]",
            "[$id]",
            "$id",
            "this.[null]",
        ];
        for i in &s {
            assert_rule!(Rule::reference, i);
        }
    }

    #[test]
    fn test_name() {
        let s = ["if", "(abc)"];
        for i in &s {
            assert_rule!(Rule::name, i);
        }
    }

    #[test]
    fn test_param() {
        let s = ["hello", "\"json literal\"", "nullable", "truestory"];
        for i in &s {
            assert_rule!(Rule::helper_parameter, i);
        }
    }

    #[test]
    fn test_hash() {
        let s = [
            "hello=world",
            "hello=\"world\"",
            "hello=(world)",
            "hello=(world 0)",
        ];
        for i in &s {
            assert_rule!(Rule::hash, i);
        }
    }

    #[test]
    fn test_json_literal() {
        let s = [
            "\"json string\"",
            "\"quot: \\\"\"",
            "[]",
            "[\"hello\"]",
            "[1,2,3,4,true]",
            "{\"hello\": \"world\"}",
            "{}",
            "{\"a\":1, \"b\":2 }",
            "\"nullable\"",
        ];
        for i in &s {
            assert_rule!(Rule::literal, i);
        }
    }

    #[test]
    fn test_comment() {
        let s = ["{{!-- <hello {{ a-b c-d}} {{d-c}} ok --}}",
                 "{{!--
                    <li><a href=\"{{up-dir nest-count}}{{base-url}}index.html\">{{this.title}}</a></li>
                --}}",
                     "{{!    -- good  --}}"];
        for i in &s {
            assert_rule!(Rule::hbs_comment, i);
        }
        let s2 = ["{{! hello }}", "{{! test me }}"];
        for i in &s2 {
            assert_rule!(Rule::hbs_comment_compact, i);
        }
    }

    #[test]
    fn test_subexpression() {
        let s = ["(sub)", "(sub 0)", "(sub a=1)"];
        for i in &s {
            assert_rule!(Rule::subexpression, i);
        }
    }

    #[test]
    fn test_expression() {
        let s = vec![
            "{{exp}}",
            "{{(exp)}}",
            "{{../exp}}",
            "{{exp 1}}",
            "{{exp \"literal\"}}",
            "{{exp \"literal with space\"}}",
            "{{exp 'literal with space'}}",
            r#"{{exp "literal with escape \\\\"}}"#,
            "{{exp ref}}",
            "{{exp (sub)}}",
            "{{exp (sub 123)}}",
            "{{exp []}}",
            "{{exp {}}}",
            "{{exp key=1}}",
            "{{exp key=ref}}",
            "{{exp key='literal with space'}}",
            "{{exp key=\"literal with space\"}}",
            "{{exp key=(sub)}}",
            "{{exp key=(sub 0)}}",
            "{{exp key=(sub 0 key=1)}}",
        ];
        for i in &s {
            assert_rule!(Rule::expression, i);
        }
    }

    #[test]
    fn test_identifier_with_dash() {
        let s = ["{{exp-foo}}"];
        for i in &s {
            assert_rule!(Rule::expression, i);
        }
    }

    #[test]
    fn test_html_expression() {
        let s = [
            "{{{html}}}",
            "{{{(html)}}}",
            "{{{(html)}}}",
            "{{&html}}",
            "{{{html 1}}}",
            "{{{html p=true}}}",
            "{{{~ html}}}",
            "{{{html ~}}}",
            "{{{~ html ~}}}",
            "{{~{ html }~}}",
            "{{~{ html }}}",
            "{{{ html }~}}",
        ];
        for i in &s {
            assert_rule!(Rule::html_expression, i);
        }
    }

    #[test]
    fn test_helper_start() {
        let s = [
            "{{#if hello}}",
            "{{#if (hello)}}",
            "{{#if hello=world}}",
            "{{#if hello hello=world}}",
            "{{#if []}}",
            "{{#if {}}}",
            "{{#if}}",
            "{{~#if hello~}}",
            "{{#each people as |person|}}",
            "{{#each-obj obj as |val key|}}",
            "{{#each assets}}",
        ];
        for i in &s {
            assert_rule!(Rule::helper_block_start, i);
        }
    }

    #[test]
    fn test_helper_end() {
        let s = ["{{/if}}", "{{~/if}}", "{{~/if ~}}", "{{/if   ~}}"];
        for i in &s {
            assert_rule!(Rule::helper_block_end, i);
        }
    }

    #[test]
    fn test_helper_block() {
        let s = [
            "{{#if hello}}hello{{/if}}",
            "{{#if true}}hello{{/if}}",
            "{{#if nice ok=1}}hello{{/if}}",
            "{{#if}}hello{{else}}world{{/if}}",
            "{{#if}}hello{{^}}world{{/if}}",
            "{{#if}}{{#if}}hello{{/if}}{{/if}}",
            "{{#if}}hello{{~else}}world{{/if}}",
            "{{#if}}hello{{else~}}world{{/if}}",
            "{{#if}}hello{{~^~}}world{{/if}}",
            "{{#if}}{{/if}}",
            "{{#if}}hello{{else if}}world{{else}}test{{/if}}",
        ];
        for i in &s {
            assert_rule!(Rule::helper_block, i);
        }
    }

    #[test]
    fn test_raw_block() {
        let s = [
            "{{{{if hello}}}}good {{hello}}{{{{/if}}}}",
            "{{{{if hello}}}}{{#if nice}}{{/if}}{{{{/if}}}}",
        ];
        for i in &s {
            assert_rule!(Rule::raw_block, i);
        }
    }

    #[test]
    fn test_block_param() {
        let s = ["as |person|", "as |val key|"];
        for i in &s {
            assert_rule!(Rule::block_param, i);
        }
    }

    #[test]
    fn test_path() {
        let s = vec![
            "a",
            "a.b.c.d",
            "a.[0].[1].[2]",
            "a.[abc]",
            "a/v/c.d.s",
            "a.[0]/b/c/d",
            "a.[bb c]/b/c/d",
            "a.[0].[#hello]",
            "../a/b.[0].[1]",
            "this.[0]/[1]/this/a",
            "./this_name",
            "./goo/[/bar]",
            "a.[你好]",
            "a.[10].[#comment]",
            "a.[]", // empty key
            "./[/foo]",
            "[foo]",
            "@root/a/b",
            "nullable",
        ];
        for i in &s {
            assert_rule_match!(Rule::path, i);
        }
    }

    #[test]
    fn test_decorator_expression() {
        let s = ["{{* ssh}}", "{{~* ssh}}"];
        for i in &s {
            assert_rule!(Rule::decorator_expression, i);
        }
    }

    #[test]
    fn test_decorator_block() {
        let s = [
            "{{#* inline}}something{{/inline}}",
            "{{~#* inline}}hello{{/inline}}",
            "{{#* inline \"partialname\"}}something{{/inline}}",
        ];
        for i in &s {
            assert_rule!(Rule::decorator_block, i);
        }
    }

    #[test]
    fn test_partial_expression() {
        let s = [
            "{{> hello}}",
            "{{> (hello)}}",
            "{{~> hello a}}",
            "{{> hello a=1}}",
            "{{> (hello) a=1}}",
            "{{> hello.world}}",
            "{{> [a83?f4+.3]}}",
            "{{> 'anif?.bar'}}",
        ];
        for i in &s {
            assert_rule!(Rule::partial_expression, i);
        }
    }

    #[test]
    fn test_partial_block() {
        let s = ["{{#> hello}}nice{{/hello}}"];
        for i in &s {
            assert_rule!(Rule::partial_block, i);
        }
    }
}
