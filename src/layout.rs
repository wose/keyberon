use crate::action::Action::{self, *};
use crate::action::{d, k, l, m};
use crate::key_code::KeyCode::{self, *};

const CUT: Action = m(&[LShift, Delete]);
const COPY: Action = m(&[LCtrl, Insert]);
const PASTE: Action = m(&[LShift, Insert]);

pub type Layers = &'static [&'static [&'static [Action]]];

#[rustfmt::skip]
pub static LAYERS: Layers = &[
    &[
        &[k(Grave),   k(Kb1),k(Kb2),k(Kb3), k(Kb4),  k(Kb5),k(KpMinus),k(KpSlash),k(KpAsterisk),k(Kb6),   k(Kb7),  k(Kb8), k(Kb9),  k(Kb0),   k(Minus)   ],
        &[k(Tab),     k(Q),  k(W),  k(E),   k(R),    k(T),     k(Kp7), k(Kp8),    k(Kp9),       k(Y),     k(U),    k(I),   k(O),    k(P),     k(LBracket)],
        &[k(RBracket),k(A),  k(S),  k(D),   k(F),    k(G),     k(Kp4), k(Kp5),    k(Kp6),       k(H),     k(J),    k(K),   k(L),    k(SColon),k(Quote)   ],
        &[k(Equal),   k(Z),  k(X),  k(C),   k(V),    k(B),     k(Kp1), k(Kp2),    k(Kp3),       k(N),     k(M),    k(Comma),k(Dot), k(Slash), k(Bslash)  ],
        &[k(LCtrl),   l(1), k(LGui),k(LAlt),k(Space),k(LShift),k(Kp0), k(KpDot),  k(KpEnter),   k(RShift),k(Enter),k(RAlt),k(BSpace),k(Escape),k(RCtrl)  ],
    ], &[
        &[k(F1),      k(F2),    k(F3),k(F4),k(F5),k(F6),Trans,Trans,Trans,k(F7),k(F8),  k(F9),  k(F10), k(F11),  k(F12)   ],
        &[k(Escape),  Trans,    Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,  Trans,  Trans,  Trans,   k(PgUp)  ],
        &[d(0),       d(1),     Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,k(Left),k(Down),k(Up),  k(Right),k(PgDown)],
        &[k(CapsLock),k(Delete),CUT,  COPY, PASTE,Trans,Trans,Trans,Trans,Trans,Trans,  Trans,  k(Home),k(Up),   k(End)   ],
        &[Trans,      Trans,    Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,  Trans,  k(Left),k(Down), k(Right) ],
    ]
];

pub struct Layout {
    layers: Layers,
    default_layer: usize,
}

impl Layout {
    pub const fn new(layers: Layers) -> Self {
        Self {
            layers,
            default_layer: 0,
        }
    }
    pub fn key_codes<'a>(
        &'a mut self,
        kp: impl Iterator<Item = (usize, usize)> + Clone + 'a,
    ) -> impl Iterator<Item = KeyCode> + 'a {
        let layer = self.layer(kp.clone()).unwrap_or(self.default_layer);
        kp.flat_map(move |(i, j)| match self.layers[layer][i][j] {
            Trans => self.layers[self.default_layer][i][j].key_codes(),
            DefaultLayer(default) => {
                self.default_layer = default;
                DefaultLayer(default).key_codes()
            }
            kc => kc.key_codes(),
        })
    }
    fn layer(&self, kp: impl Iterator<Item = (usize, usize)>) -> Option<usize> {
        let mut iter = kp.filter_map(|(i, j)| self.layers[self.default_layer][i][j].layout());
        let first = iter.next()?;
        Some(first + iter.sum::<usize>())
    }
}
