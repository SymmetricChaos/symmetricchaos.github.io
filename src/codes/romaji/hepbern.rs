use lazy_static::lazy_static;

use crate::tokenizer::Node;

#[test]
fn test() {
    let paths = HEPBERN_SHIKI.output_paths();
    
    for (k,v) in &paths {
        print!("{k} <= {v:?}\n")
    }
}

lazy_static! {
    pub static ref HEPBERN_SHIKI: Node = {
        let transitions = vec![
			Node::leaf('\u{3000}', " "),
            Node::leaf('、', ","),
            Node::leaf('。', "."),
            Node::leaf('「', "‘"),
            Node::leaf('」', "’"),
            Node::leaf('『', "“"),
            Node::leaf('』', "”"),
            Node::leaf('〜', "~"),
            Node::leaf('ぁ', "a"),
            Node::leaf('あ', "a"),
            Node::leaf('ぃ', "i"),
            Node::leaf('い', "i"),
            Node::leaf('ぅ', "u"),
            Node::leaf('う', "u"),
            Node::leaf('ぇ', "e"),
            Node::leaf('え', "e"),
            Node::leaf('ぉ', "o"),
            Node::leaf('お', "o"),
            Node::leaf('か', "ka"),
            Node::leaf('が', "ga"),

            Node::branch(
                'き', Some("ki"),
                vec![
                    Node::leaf('ぃ', "kyi"),
                    Node::leaf('ぇ', "kye"),
                    Node::leaf('ゃ', "kya"),
                    Node::leaf('ゅ', "kyu"),
                    Node::leaf('ょ', "kyo"),
                ]
            ),

            Node::branch(
                'ぎ', Some("gi"),
                vec![
                    Node::leaf('ぃ', "gyi"),
                    Node::leaf('ぇ', "gye"),
                    Node::leaf('ゃ', "gya"),
                    Node::leaf('ゅ', "gyu"),
                    Node::leaf('ょ', "gyo"),
                ]
            ),

            Node::branch(
                'く', Some("ku"),
                vec![
                    Node::leaf('ぃ', "kyi"),
                    Node::leaf('ぇ', "kye"),
                    Node::leaf('ゃ', "kya"),
                    Node::leaf('ゅ', "kyu"),
                    Node::leaf('ょ', "kyo"),
                ]
            ),

            Node::leaf('ぐ', "gu"),
            Node::leaf('け', "ke"),
            Node::leaf('げ', "ge"),
            Node::leaf('こ', "ko"),
            Node::leaf('ご', "go"),
            Node::leaf('さ', "sa"),
            Node::leaf('ざ', "za"),

            Node::branch(
                'し', Some("shi"),
                vec![
                    Node::leaf('ぃ', "shyi"),
                    Node::leaf('ぇ', "she"),
                    Node::leaf('ゃ', "sha"),
                    Node::leaf('ゅ', "shu"),
                    Node::leaf('ょ', "sho"),
                ]
            ),

            (
                'じ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "jyi"),
                        Node::leaf('ぇ', "je"),
                        Node::leaf('ゃ', "ja"),
                        Node::leaf('ゅ', "ju"),
                        Node::leaf('ょ', "jo"),
                    ]),
                    output: Some("ji"),
                },
            ),
            Node::leaf('す', "su"),
            Node::leaf('ず', "zu"),
            Node::leaf('せ', "se"),
            Node::leaf('ぜ', "ze"),
            Node::leaf('そ', "so"),
            Node::leaf('ぞ', "zo"),
            Node::leaf('た', "ta"),
            Node::leaf('だ', "da"),
            (
                'ち',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "chyi"),
                        Node::leaf('ぇ', "che"),
                        Node::leaf('ゃ', "cha"),
                        Node::leaf('ゅ', "chu"),
                        Node::leaf('ょ', "cho"),
                    ]),
                    output: Some("chi"),
                },
            ),
            (
                'ぢ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "jyi"),
                        Node::leaf('ぇ', "je"),
                        Node::leaf('ゃ', "ja"),
                        Node::leaf('ゅ', "ju"),
                        Node::leaf('ょ', "jo"),
                    ]),
                    output: Some("ji"),
                },
            ),
            (
                'っ',
                Node {
                    transitions: Some(vec![
						Node::leaf('\u{3000}', " "),
                        Node::leaf('、', ","),
                        Node::leaf('。', "."),
                        Node::leaf('「', "‘"),
                        Node::leaf('」', "’"),
                        Node::leaf('『', "“"),
                        Node::leaf('』', "”"),
                        Node::leaf('〜', "~"),
                        Node::leaf('ぁ', "a"),
                        Node::leaf('あ', "a"),
                        Node::leaf('ぃ', "i"),
                        Node::leaf('い', "i"),
                        Node::leaf('ぅ', "u"),
                        Node::leaf('う', "u"),
                        Node::leaf('ぇ', "e"),
                        Node::leaf('え', "e"),
                        Node::leaf('ぉ', "o"),
                        Node::leaf('お', "o"),
                        Node::leaf('か', "kka"),
                        Node::leaf('が', "gga"),
                        (
                            'き',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "kkyi"),
                                    Node::leaf('ぇ', "kkye"),
                                    Node::leaf('ゃ', "kkya"),
                                    Node::leaf('ゅ', "kkyu"),
                                    Node::leaf('ょ', "kkyo"),
                                ]),
                                output: Some("kki"),
                            },
                        ),
                        (
                            'ぎ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ggyi"),
                                    Node::leaf('ぇ', "ggye"),
                                    Node::leaf('ゃ', "ggya"),
                                    Node::leaf('ゅ', "ggyu"),
                                    Node::leaf('ょ', "ggyo"),
                                ]),
                                output: Some("ggi"),
                            },
                        ),
                        (
                            'く',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "kkyi"),
                                    Node::leaf('ぇ', "kkye"),
                                    Node::leaf('ゃ', "kkya"),
                                    Node::leaf('ゅ', "kkyu"),
                                    Node::leaf('ょ', "kkyo"),
                                ]),
                                output: Some("kku"),
                            },
                        ),
                        Node::leaf('ぐ', "ggu"),
                        Node::leaf('け', "kke"),
                        Node::leaf('げ', "gge"),
                        Node::leaf('こ', "kko"),
                        Node::leaf('ご', "ggo"),
                        Node::leaf('さ', "ssa"),
                        Node::leaf('ざ', "zza"),
                        (
                            'し',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "sshyi"),
                                    Node::leaf('ぇ', "sshe"),
                                    Node::leaf('ゃ', "ssha"),
                                    Node::leaf('ゅ', "sshu"),
                                    Node::leaf('ょ', "ssho"),
                                ]),
                                output: Some("sshi"),
                            },
                        ),
                        (
                            'じ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "jjyi"),
                                    Node::leaf('ぇ', "jje"),
                                    Node::leaf('ゃ', "jja"),
                                    Node::leaf('ゅ', "jju"),
                                    Node::leaf('ょ', "jjo"),
                                ]),
                                output: Some("jji"),
                            },
                        ),
                        Node::leaf('す', "ssu"),
                        Node::leaf('ず', "zzu"),
                        Node::leaf('せ', "sse"),
                        Node::leaf('ぜ', "zze"),
                        Node::leaf('そ', "sso"),
                        Node::leaf('ぞ', "zzo"),
                        Node::leaf('た', "tta"),
                        Node::leaf('だ', "dda"),
                        (
                            'ち',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "tchyi"),
                                    Node::leaf('ぇ', "tche"),
                                    Node::leaf('ゃ', "tcha"),
                                    Node::leaf('ゅ', "tchu"),
                                    Node::leaf('ょ', "tcho"),
                                ]),
                                output: Some("tchi"),
                            },
                        ),
                        (
                            'ぢ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "jjyi"),
                                    Node::leaf('ぇ', "jje"),
                                    Node::leaf('ゃ', "jja"),
                                    Node::leaf('ゅ', "jju"),
                                    Node::leaf('ょ', "jjo"),
                                ]),
                                output: Some("jji"),
                            },
                        ),
                        Node::leaf('つ', "ttsu"),
                        Node::leaf('づ', "zzu"),
                        Node::leaf('て', "tte"),
                        Node::leaf('で', "dde"),
                        Node::leaf('と', "tto"),
                        Node::leaf('ど', "ddo"),
                        Node::leaf('な', "na"),
                        (
                            'に',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "nyi"),
                                    Node::leaf('ぇ', "nye"),
                                    Node::leaf('ゃ', "nya"),
                                    Node::leaf('ゅ', "nyu"),
                                    Node::leaf('ょ', "nyo"),
                                ]),
                                output: Some("ni"),
                            },
                        ),
                        Node::leaf('ぬ', "nu"),
                        Node::leaf('ね', "ne"),
                        Node::leaf('の', "no"),
                        Node::leaf('は', "hha"),
                        Node::leaf('ば', "bba"),
                        Node::leaf('ぱ', "ppa"),
                        (
                            'ひ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "hhyi"),
                                    Node::leaf('ぇ', "hhye"),
                                    Node::leaf('ゃ', "hhya"),
                                    Node::leaf('ゅ', "hhyu"),
                                    Node::leaf('ょ', "hhyo"),
                                ]),
                                output: Some("hhi"),
                            },
                        ),
                        (
                            'び',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "bbyi"),
                                    Node::leaf('ぇ', "bbye"),
                                    Node::leaf('ゃ', "bbya"),
                                    Node::leaf('ゅ', "bbyu"),
                                    Node::leaf('ょ', "bbyo"),
                                ]),
                                output: Some("bbi"),
                            },
                        ),
                        (
                            'ぴ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ppyi"),
                                    Node::leaf('ぇ', "ppye"),
                                    Node::leaf('ゃ', "ppya"),
                                    Node::leaf('ゅ', "ppyu"),
                                    Node::leaf('ょ', "ppyo"),
                                ]),
                                output: Some("ppi"),
                            },
                        ),
                        (
                            'ふ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ffyi"),
                                    Node::leaf('ぇ', "ffye"),
                                    Node::leaf('ゃ', "ffya"),
                                    Node::leaf('ゅ', "ffyu"),
                                    Node::leaf('ょ', "ffyo"),
                                ]),
                                output: Some("ffu"),
                            },
                        ),
                        Node::leaf('ぶ', "bbu"),
                        Node::leaf('ぷ', "ppu"),
                        Node::leaf('へ', "hhe"),
                        Node::leaf('べ', "bbe"),
                        Node::leaf('ぺ', "ppe"),
                        Node::leaf('ほ', "hho"),
                        Node::leaf('ぼ', "bbo"),
                        Node::leaf('ぽ', "ppo"),
                        Node::leaf('ま', "mma"),
                        (
                            'み',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "mmyi"),
                                    Node::leaf('ぇ', "mmye"),
                                    Node::leaf('ゃ', "mmya"),
                                    Node::leaf('ゅ', "mmyu"),
                                    Node::leaf('ょ', "mmyo"),
                                ]),
                                output: Some("mmi"),
                            },
                        ),
                        Node::leaf('む', "mmu"),
                        Node::leaf('め', "mme"),
                        Node::leaf('も', "mmo"),
                        Node::leaf('ゃ', "ya"),
                        Node::leaf('や', "ya"),
                        Node::leaf('ゅ', "yu"),
                        Node::leaf('ゆ', "yu"),
                        Node::leaf('ょ', "yo"),
                        Node::leaf('よ', "yo"),
                        Node::leaf('ら', "rra"),
                        (
                            'り',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "rryi"),
                                    Node::leaf('ぇ', "rrye"),
                                    Node::leaf('ゃ', "rrya"),
                                    Node::leaf('ゅ', "rryu"),
                                    Node::leaf('ょ', "rryo"),
                                ]),
                                output: Some("rri"),
                            },
                        ),
                        Node::leaf('る', "rru"),
                        Node::leaf('れ', "rre"),
                        Node::leaf('ろ', "rro"),
                        Node::leaf('わ', "wwa"),
                        Node::leaf('ゐ', "wwi"),
                        Node::leaf('ゑ', "wwe"),
                        Node::leaf('を', "wwo"),
                        Node::leaf('ん', "n"),
                        (
                            'ゔ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "vvyi"),
                                    Node::leaf('ぇ', "vvye"),
                                    Node::leaf('ゃ', "vvya"),
                                    Node::leaf('ゅ', "vvyu"),
                                    Node::leaf('ょ', "vvyo"),
                                ]),
                                output: Some("vvu"),
                            },
                        ),
                        Node::leaf('ゔ', "vva"),
                        Node::leaf('ゔ', "vvi"),
                        Node::leaf('ゔ', "vve"),
                        Node::leaf('ゔ', "vvo"),
                        Node::leaf('・', "/"),
                        Node::leaf('ー', "-"),
                        Node::leaf('！', "!"),
                        Node::leaf('（', "("),
                        Node::leaf('）', ")"),
                        Node::leaf('：', ":"),
                        Node::leaf('？', "?"),
                        Node::leaf('［', "["),
                        Node::leaf('］', "]"),
                        Node::leaf('｛', "{"),
                        Node::leaf('｝', "}"),
                    ]),
                    output: None,
                },
            ),
            Node::leaf('つ', "tsu"),
            Node::leaf('づ', "zu"),
            Node::leaf('て', "te"),
            Node::leaf('で', "de"),
            Node::leaf('と', "to"),
            Node::leaf('ど', "do"),
            Node::leaf('な', "na"),
            (
                'に',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "nyi"),
                        Node::leaf('ぇ', "nye"),
                        Node::leaf('ゃ', "nya"),
                        Node::leaf('ゅ', "nyu"),
                        Node::leaf('ょ', "nyo"),
                    ]),
                    output: Some("ni"),
                },
            ),
            Node::leaf('ぬ', "nu"),
            Node::leaf('ね', "ne"),
            Node::leaf('の', "no"),
            Node::leaf('は', "ha"),
            Node::leaf('ば', "ba"),
            Node::leaf('ぱ', "pa"),
            (
                'ひ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "hyi"),
                        Node::leaf('ぇ', "hye"),
                        Node::leaf('ゃ', "hya"),
                        Node::leaf('ゅ', "hyu"),
                        Node::leaf('ょ', "hyo"),
                    ]),
                    output: Some("hi"),
                },
            ),
            (
                'び',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "byi"),
                        Node::leaf('ぇ', "bye"),
                        Node::leaf('ゃ', "bya"),
                        Node::leaf('ゅ', "byu"),
                        Node::leaf('ょ', "byo"),
                    ]),
                    output: Some("bi"),
                },
            ),
            (
                'ぴ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "pyi"),
                        Node::leaf('ぇ', "pye"),
                        Node::leaf('ゃ', "pya"),
                        Node::leaf('ゅ', "pyu"),
                        Node::leaf('ょ', "pyo"),
                    ]),
                    output: Some("pi"),
                },
            ),
            (
                'ふ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "fyi"),
                        Node::leaf('ぇ', "fye"),
                        Node::leaf('ゃ', "fya"),
                        Node::leaf('ゅ', "fyu"),
                        Node::leaf('ょ', "fyo"),
                    ]),
                    output: Some("fu"),
                },
            ),
            Node::leaf('ぶ', "bu"),
            Node::leaf('ぷ', "pu"),
            Node::leaf('へ', "he"),
            Node::leaf('べ', "be"),
            Node::leaf('ぺ', "pe"),
            Node::leaf('ほ', "ho"),
            Node::leaf('ぼ', "bo"),
            Node::leaf('ぽ', "po"),
            Node::leaf('ま', "ma"),
            (
                'み',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "myi"),
                        Node::leaf('ぇ', "mye"),
                        Node::leaf('ゃ', "mya"),
                        Node::leaf('ゅ', "myu"),
                        Node::leaf('ょ', "myo"),
                    ]),
                    output: Some("mi"),
                },
            ),
            Node::leaf('む', "mu"),
            Node::leaf('め', "me"),
            Node::leaf('も', "mo"),
            Node::leaf('ゃ', "ya"),
            Node::leaf('や', "ya"),
            Node::leaf('ゅ', "yu"),
            Node::leaf('ゆ', "yu"),
            Node::leaf('ょ', "yo"),
            Node::leaf('よ', "yo"),
            Node::leaf('ら', "ra"),
            (
                'り',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "ryi"),
                        Node::leaf('ぇ', "rye"),
                        Node::leaf('ゃ', "rya"),
                        Node::leaf('ゅ', "ryu"),
                        Node::leaf('ょ', "ryo"),
                    ]),
                    output: Some("ri"),
                },
            ),
            Node::leaf('る', "ru"),
            Node::leaf('れ', "re"),
            Node::leaf('ろ', "ro"),
            Node::leaf('わ', "wa"),
            Node::leaf('ゐ', "wi"),
            Node::leaf('ゑ', "we"),
            Node::leaf('を', "wo"),
            (
                'ん',
                Node {
                    transitions: Some(vec![
                        Node::leaf('あ', "n\'a"),
                        Node::leaf('い', "n\'i"),
                        Node::leaf('う', "n\'u"),
                        Node::leaf('え', "n\'e"),
                        Node::leaf('お', "n\'o"),
                        Node::leaf('や', "n\'ya"),
                        Node::leaf('ゆ', "n\'yu"),
                        Node::leaf('よ', "n\'yo"),
                    ]),
                    output: Some("n"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "vyi"),
                        Node::leaf('ぇ', "vye"),
                        Node::leaf('ゃ', "vya"),
                        Node::leaf('ゅ', "vyu"),
                        Node::leaf('ょ', "vyo"),
                    ]),
                    output: Some("vu"),
                },
            ),
            Node::leaf('ゔ', "va"),
            Node::leaf('ゔ', "vi"),
            Node::leaf('ゔ', "ve"),
            Node::leaf('ゔ', "vo"),
            Node::leaf('・', "/"),
            Node::leaf('ー', "-"),
            Node::leaf('！', "!"),
            Node::leaf('（', "("),
            Node::leaf('）', ")"),
            Node::leaf('：', ":"),
            Node::leaf('？', "?"),
            Node::leaf('［', "["),
            Node::leaf('］', "]"),
            Node::leaf('｛', "{"),
            Node::leaf('｝', "}"),
        ];

        Node::tree(transitions)
    };
}