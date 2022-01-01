use super::ast::*;
use super::attr;
use super::ctxt::*;

pub fn check(cx: &Ctxt, container: &Container) {
    check_boxed(cx, container);
    check_size_hints(cx, container);
}

fn check_boxed(cx: &Ctxt, container: &Container) {
    match &container.data {
        Data::Enum(variants) => {
            if container.attrs.id.is_some() {
                cx.error_spanned_by(
                    container.original,
                    "#[tl(id = 0x...)] is not allowed in an enum",
                )
            }

            if container.attrs.boxed && variants.is_empty() {
                cx.error_spanned_by(
                    container.original,
                    "#[tl(boxed)] is not allowed in an empty enum",
                )
            }

            for variant in variants {
                if container.attrs.boxed && variant.attrs.id.is_none() {
                    cx.error_spanned_by(
                        variant.original,
                        "#[tl(id = 0x...)] is required for boxed enum variant",
                    )
                }

                if !container.attrs.boxed && variant.attrs.id.is_some() {
                    cx.error_spanned_by(
                        variant.original,
                        "#[tl(id = 0x...)] is not allowed for bare enum variant",
                    )
                }
            }
        }
        Data::Struct(_, _) => {
            if container.attrs.boxed && container.attrs.id.is_none() {
                cx.error_spanned_by(
                    container.original,
                    "#[tl(id = 0x...)] is required for struct with #[tl(boxed)]",
                )
            }

            if !container.attrs.boxed && container.attrs.id.is_some() {
                cx.error_spanned_by(
                    container.original,
                    "#[tl(id = 0x...)] can't be used without #[tl(boxed)]",
                )
            }
        }
    }
}

fn check_size_hints(cx: &Ctxt, container: &Container) {
    check_size_hint(cx, container.original, &container.attrs.size_hint);

    match &container.data {
        Data::Enum(variants) => {
            for variant in variants {
                check_size_hint(cx, variant.original, &variant.attrs.size_hint);
            }
        }
        Data::Struct(_, fields) => {
            for field in fields {
                check_size_hint(cx, field.original, &field.attrs.size_hint);
            }
        }
    }
}

fn check_size_hint<T>(cx: &Ctxt, object: T, size_hint: &Option<attr::SizeHint>)
where
    T: quote::ToTokens,
{
    if let Some(attr::SizeHint::Explicit { value }) = &size_hint {
        match *value {
            hint if hint < 4 => cx.error_spanned_by(object, "size hint is too small"),
            hint if hint % 4 != 0 => {
                cx.error_spanned_by(object, "size hint must be aligned to 4 bytes")
            }
            _ => {}
        }
    }
}
