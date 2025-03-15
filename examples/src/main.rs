ferrugem::ferrugem! {
    externo caixote ferrugem;

    use std::collections::Dicionário;

    convenção ChaveValor {
        função escrever(&eu, chave: Corda, valor: Corda);
        função ler(&eu, chave: Corda) -> Resultado<Opção<&Corda>, Corda>;
    }

    estático mutável DICIONÁRIO: Opção<Dicionário<Corda, Corda>> = Nenhum;

    estrutura Concreta;

    realização ChaveValor para Concreta {
        função escrever(&eu, chave: Corda, valor: Corda) {
            deixa dicionário = perigoso {
                DICIONÁRIO.pega_ou_insere_com(Padrão::padrão)
            };
            dicionário.inserir(chave, valor);
        }
        função ler(&eu, chave: Corda) -> Resultado<Opção<&Corda>, Corda> {
            se deixa Algum(dicionário) = perigoso{ DICIONÁRIO.como_ref() } {
                Beleza(dicionário.pega(&chave))
            } ou_então {
                Errou("busca o dicionário!".transforma())
            }
        }
    }

    público(caixote) função talvez(i: u32) -> Opção<Resultado<u32, Corda>> {
        se i % 2 == 1 {
            se i == 42 {
                Algum(Errou(Corda::de("merda")))
            } ou_então {
                Algum(Beleza(33))
            }
        } ou_então {
            Nenhum
        }
    }

    assíncrono função exemplo() {
    }

    assíncrono função exemplo2() {
        exemplo().peraí;
    }

    função principal() {
        deixa mutável x = 31;

        combina x {
            42 => {
                imprimeln!("mortandela")
            }
            _ => imprimeln!("pronto")
        }

        para i entre 0..10 {
            deixa val = volta {
                quebra i;
            };

            enquanto x < val {
                x += 1;
            }

            x = se deixa Algum(resultado) = Algum(i) {
                resultado
            } ou_então {
                12
            };
        }

        //secundária();
    }

    #[permite(código_inacessível)]
    função secundária() {
        fudeu!("fudeu!");
        deu_merda!("deu merda!");
        eita!("eita!"); // in SFW contexts
    }
}
