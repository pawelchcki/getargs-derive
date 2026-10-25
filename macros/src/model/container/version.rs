use super::{RawContainer, Version};
use crate::model::attr::{parse_string, reject_value, set_once};
use syn::{Token, meta::ParseNestedMeta};

pub(super) fn version(meta: &ParseNestedMeta, raw: &mut RawContainer) -> syn::Result<()> {
    let value = if meta.input.peek(Token![=]) {
        let fork = meta.input.fork();
        let _: Token![=] = fork.parse()?;
        if fork.parse::<syn::ExprMacro>().is_ok() {
            let expression: syn::ExprMacro = meta.value()?.parse()?;
            if !expression.mac.path.is_ident("concat") && !expression.mac.path.is_ident("env") {
                return Err(syn::Error::new_spanned(
                    expression,
                    "`version` supports string literals, concat! or env!",
                ));
            }
            return set_once(&mut raw.version, Version::Macro(expression), meta);
        }
        let literal = parse_string(meta, "version", "version = \"1.0.0\"")?;
        let value = literal.value();
        if value.trim().is_empty() {
            return Err(syn::Error::new(literal.span(), "`version` cannot be empty"));
        }
        Version::Literal(value)
    } else {
        reject_value(meta, "version")?;
        Version::Package
    };
    set_once(&mut raw.version, value, meta)
}
