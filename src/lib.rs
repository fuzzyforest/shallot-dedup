extern crate proc_macro;
use proc_macro::{Group, Punct, Spacing, TokenStream, TokenTree};

fn is_comma(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == ',')
}

fn tt_eq(a: &TokenTree, b: &TokenTree) -> bool {
    match (a, b) {
        (TokenTree::Group(g1), TokenTree::Group(g2)) => {
            let s1: Vec<_> = g1.stream().into_iter().collect();
            let s2: Vec<_> = g2.stream().into_iter().collect();
            g1.delimiter() == g2.delimiter() && tt_list_eq(&s1, &s2)
        }
        (TokenTree::Ident(i1), TokenTree::Ident(i2)) => i1.to_string() == i2.to_string(),
        (TokenTree::Punct(p1), TokenTree::Punct(p2)) => p1.as_char() == p2.as_char(),
        (TokenTree::Literal(l1), TokenTree::Literal(l2)) => l1.to_string() == l2.to_string(),
        _ => false,
    }
}

fn tt_list_eq(a: &[TokenTree], b: &[TokenTree]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| tt_eq(a, b))
}

fn dedup_stream(stream: TokenStream) -> TokenStream {
    let stream: Vec<TokenTree> = stream.into_iter().collect();
    let mut items: Vec<Vec<TokenTree>> = Vec::new();
    for tokens in stream.split(is_comma) {
        items.push(tokens.to_vec())
    }

    items.reverse();
    let mut deduplicated_items: Vec<Vec<TokenTree>> = Vec::new();
    for item in items {
        if !deduplicated_items.iter().any(|ddi| tt_list_eq(ddi, &item)) {
            deduplicated_items.push(item)
        }
    }
    deduplicated_items.reverse();

    let mut tokens: Vec<TokenTree> = Vec::new();
    for item in deduplicated_items {
        tokens.extend(item);
        tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }
    tokens.into_iter().collect()
}

#[proc_macro]
pub fn dedup_call(item: TokenStream) -> TokenStream {
    let mut stream = item.into_iter().peekable();
    let mut result: Vec<TokenTree> = Vec::new();
    result.extend((&mut stream).take_while(|tt| !is_comma(tt)));
    match stream.next() {
        Some(TokenTree::Group(group)) => {
            let group = Group::new(group.delimiter(), dedup_stream(group.stream()));
            result.push(TokenTree::Group(group));
        }
        Some(tt) => {
            panic!("Expected group to deduplicate, not {}", tt)
        }
        None => {
            panic!(
                "Expected group to deduplicate after {}",
                result.into_iter().collect::<TokenStream>()
            )
        }
    }
    if stream.next().is_some() {
        panic!("Extra tokens after deduplication group")
    }

    // NOTE This is not the most general way of doing this
    result.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));

    result.into_iter().collect()
}
