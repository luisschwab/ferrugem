use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Errou" => "Err",
        "Beleza" => "Ok",
        "Corda" => "String",
        "Dicionário" => "HashMap",
        "Padrão" => "Default",
        "Erro" => "Error",
        "Opção" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Eu" => "Self",
        "imprimeln" => "println",
        "quebra" => "break",
        "assíncrono" => "async",
        "peraí" => "await",
        "volta" => "loop",
        "mexe" => "move",
        "caixote" => "crate",
        "código_inacessível" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "convenção" => "trait",
        "perigoso" => "unsafe",
        "entre" => "in",
        "de" => "from",
        "dinâmico" => "dyn",
        "desembrulha" => "unwrap",
        "padrão" => "default",
        "como_ref" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "mentira" => "false",
        "função" => "fn",
        "massa" => "super",
        "inserir" => "insert",
        "pega" => "get",
        "permite" => "allow",
        "fudeu" | "deu_merda" | "eita" => "panic",
        "módulo" => "mod",
        "mutável" => "mut",
        "novo" => "new",
        "onde" => "where",
        "para" => "for",
        "pega_ou_insere_com" => "get_or_insert_with",
        "principal" => "main",
        "público" => "pub",
        "que" => None?,
        "retorna" => "return",
        "realização" => "impl",
        "ref" => "ref",
        "combina" => "match",
        "se" => "if",
        "ou_então" => "else",
        "eu" => "self",
        "deixa" => "let",
        "estático" => "static",
        "estrutura" => "struct",
        "espera" => "expect",
        "enquanto" => "while",
        "use" => "use",
        "transforma" => "into",
        "verdade" => "true",
        "enumeração" => "enum",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "FluxoDeTokens" => "TokenStream",
        "ÁrvoreDeTokens" => "TokenTree",
        "para_corda" => "to_string",
        "como_corda" => "as_str",
        "localização" => "span",
        "Vetor" => "Vec",
        "fluxo" => "stream",
        "empurra" => "push",
        "extende" => "extend",
        "delimitador" => "delimiter",
        "Pontuação" => "Punct",
        "Literal" => "Literal",
        "macro_procedural" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
