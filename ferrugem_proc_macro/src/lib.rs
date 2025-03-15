ferrugem_compilacao::ferrugem! {
    use macro_procedural::{Grupo, Identificador, FluxoDeTokens, ÁrvoreDeTokens};

    função trocar_identificador(identificador: Identificador) -> Opção<ÁrvoreDeTokens> {
        deixa identificador_corda = identificador.para_corda();

        deixa nova_corda = combina identificador_corda.como_corda() {
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
            _ => &identificador_corda,
        };

        deixa novo_identificador = Identificador::novo(nova_corda, identificador.localização());
        Algum(ÁrvoreDeTokens::Identificador(novo_identificador))
    }

    função trocar_árvore(token: ÁrvoreDeTokens, sortie: &mutável Vetor<ÁrvoreDeTokens>) {
        combina token {
            ÁrvoreDeTokens::Grupo(grupo) => {
                deixa mutável grupo_elementos = Vetor::novo();
                trocar_fluxo(grupo.fluxo(), &mutável grupo_elementos);
                deixa mutável novo_fluxo = FluxoDeTokens::novo();
                novo_fluxo.extende(grupo_elementos);
                sortie.empurra(ÁrvoreDeTokens::Grupo(Grupo::novo(grupo.delimitador(), novo_fluxo)));
            }
            ÁrvoreDeTokens::Identificador(identificador) => {
                se deixa Algum(identificador) = trocar_identificador(identificador) {
                    sortie.empurra(identificador);
                }
            }
            ÁrvoreDeTokens::Pontuação(..) | ÁrvoreDeTokens::Literal(..) => {
                sortie.empurra(token);
            }
        }
    }

    função trocar_fluxo(árvore_de_tokens: FluxoDeTokens, sortie: &mutável Vetor<ÁrvoreDeTokens>) {
        para token in árvore_de_tokens {
            trocar_árvore(token, sortie)
        }
    }

    #[macro_procedural]
    público função ferrugem(elemento: FluxoDeTokens) -> FluxoDeTokens {
        deixa mutável retornado = Vetor::novo();
        trocar_fluxo(elemento, &mutável retornado);
        deixa mutável sortie = FluxoDeTokens::novo();
        sortie.extende(retornado);
        sortie
    }
}
