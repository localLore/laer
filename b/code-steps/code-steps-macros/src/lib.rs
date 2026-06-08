use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{Block, LitStr, Token, parse_macro_input};

struct StepInput {
    comment: LitStr,
    _comma: Token![,],
    block: Block,
}

impl syn::parse::Parse for StepInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(StepInput {
            comment: input.parse()?,
            _comma: input.parse()?,
            block: input.parse()?,
        })
    }
}

/// 把 `[wait]` 替换为 `::code_steps::display::press_any_key()`
struct WaitReplacer;

impl VisitMut for WaitReplacer {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let syn::Expr::Array(arr) = expr {
            if arr.elems.len() == 1 {
                if let syn::Expr::Path(path) = &arr.elems[0] {
                    if path.path.is_ident("wait") {
                        *expr = syn::parse_quote! { ::code_steps::display::press_any_key() };
                        return;
                    }
                }
            }
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

/// 去掉公共前导空白
fn dedent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let min = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let mut out = String::with_capacity(s.len());
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        if line.len() > min {
            out.push_str(&line[min..]);
        }
    }
    out.push('\n');
    out
}

#[proc_macro]
pub fn step(input: TokenStream) -> TokenStream {
    // 保留原始字符串用于展示（去掉外层花括号）
    let raw = input.to_string();
    let block_start = raw.find('{').unwrap();
    let block_end = raw.rfind('}').unwrap();
    let code_str = raw[block_start + 1..block_end].to_string();
    let code_str = code_str.trim_start_matches('\n').to_string();
    let code_str = dedent(&code_str);

    // 解析
    let mut parsed = parse_macro_input!(input as StepInput);
    let comment_str = parsed.comment.value();

    // 转换 [wait] → press_any_key()
    WaitReplacer.visit_block_mut(&mut parsed.block);

    let block = &parsed.block;

    let expanded = quote! {
        {
            ::code_steps::display::print_step_header(#comment_str);
            ::code_steps::display::print_code(#code_str);
            let __result = #block;
            ::code_steps::display::dim_code(#code_str);
            ::code_steps::display::print_step_done();
            __result
        }
    };

    expanded.into()
}
