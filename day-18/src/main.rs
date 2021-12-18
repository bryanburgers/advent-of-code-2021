use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref},
};

fn main() {
    let input = vec![
        s![[[[2, 5], 4], [[1, 0], [8, 3]]], [[2, [2, 4]], [1, [3, 3]]]],
        s![[[2, 2], [[4, 3], 3]], [[[8, 6], 3], [3, 7]]],
        s![[[9, [4, 1]], [9, 0]], [6, [6, 0]]],
        s![[[3, 9], [[4, 4], [2, 5]]], [[9, [8, 4]], 8]],
        s![[[[0, 0], 9], [[9, 3], [8, 2]]], [2, [1, 3]]],
        s![[[8, 4], 6], [[5, 1], [3, 6]]],
        s![[[6, [7, 6]], [[2, 6], 5]], [[6, 4], 2]],
        s![[1, [9, 7]], [[[5, 9], [9, 5]], [[7, 0], 1]]],
        s![[[[5, 8], [9, 4]], [[9, 3], [7, 8]]], 8],
        s![[[0, 9], [[6, 0], 7]], [[[7, 7], 6], [[9, 7], [0, 4]]]],
        s![[[[4, 3], [9, 5]], [7, [7, 3]]], [[[2, 8], 9], 4]],
        s![[7, 5], [8, 1]],
        s![[4, 6], [[[0, 6], 6], [7, 4]]],
        s![[[1, 8], [[1, 4], [1, 6]]], [3, 4]],
        s![[[6, 5], [4, [7, 3]]], [[[0, 1], [8, 4]], [4, 8]]],
        s![[5, 1], [9, [9, [3, 3]]]],
        s![[[[7, 0], [2, 5]], 1], [9, [[2, 7], [4, 4]]]],
        s![[[[5, 8], 8], 0], [8, [1, [2, 5]]]],
        s![8, [[5, 4], 7]],
        s![[[9, 8], [6, 7]], [[2, [2, 6]], [9, 6]]],
        s![[[[2, 3], 7], 6], [[8, 6], 3]],
        s![[[8, [7, 2]], 3], [[[3, 9], 4], [6, 8]]],
        s![9, [[[6, 7], [6, 0]], [[3, 9], 8]]],
        s![[[7, 7], [4, 7]], [[[9, 8], 9], [9, [2, 4]]]],
        s![[[[5, 0], 1], [4, [4, 8]]], [9, [6, 7]]],
        s![[[[9, 2], 5], [1, [5, 8]]], [[9, [0, 1]], [3, 8]]],
        s![[[5, [2, 5]], 8], [2, [0, [9, 3]]]],
        s![[7, [[8, 4], [8, 4]]], 4],
        s![[[[3, 3], 4], [[0, 0], [5, 5]]], [4, 5]],
        s![[[[9, 3], [9, 3]], 2], [5, 3]],
        s![[[9, 5], [1, 4]], [[7, 1], [3, [6, 5]]]],
        s![8, [[[1, 1], [0, 1]], [9, [3, 6]]]],
        s![[[[4, 4], 7], [0, 3]], [1, 5]],
        s![[[3, [0, 8]], 8], [5, [7, 5]]],
        s![[[[9, 6], 2], 7], [[5, [3, 7]], 0]],
        s![4, 9],
        s![[[5, [1, 3]], [[9, 5], 6]], [[[7, 9], 5], 3]],
        s![[[[3, 9], [7, 2]], [5, [8, 8]]], [1, 9]],
        s![[[[7, 8], 8], [[9, 0], [5, 1]]], [6, [[1, 0], [3, 3]]]],
        s![[[[5, 8], 1], [[8, 6], [2, 9]]], [[5, 1], 6]],
        s![[1, 7], [[5, [3, 2]], 4]],
        s![[[[3, 1], 2], [0, 8]], [3, [4, 6]]],
        s![[9, 6], [0, [[5, 2], [1, 1]]]],
        s![[[[1, 8], 8], [[9, 0], 3]], [[6, [2, 8]], [[6, 4], [6, 0]]]],
        s![[7, [[3, 2], [9, 0]]], [[[3, 2], [2, 8]], [[5, 5], [9, 2]]]],
        s![[[[2, 5], [3, 1]], [7, [9, 6]]], [[[7, 0], 7], [2, [9, 1]]]],
        s![[[[1, 6], 9], [1, [6, 5]]], [[8, [4, 1]], 6]],
        s![[[7, [4, 6]], [[2, 7], [6, 6]]], [8, 0]],
        s![[9, 7], [[[0, 7], 5], [[1, 4], [1, 3]]]],
        s![[[1, [8, 2]], [[0, 6], [9, 0]]], 8],
        s![[[4, 0], [7, [3, 3]]], [9, 6]],
        s![0, [[[6, 9], 7], [[0, 6], 1]]],
        s![5, [[4, 3], [[8, 3], [5, 7]]]],
        s![[9, 0], [0, [[7, 8], [1, 8]]]],
        s![[[[4, 3], [5, 6]], 2], [[2, 3], 1]],
        s![4, [[9, 9], [[1, 8], [9, 2]]]],
        s![[[[6, 9], 5], 1], [[[7, 4], [8, 1]], 3]],
        s![[8, [5, [2, 6]]], [[[2, 7], 6], [6, 0]]],
        s![[[[6, 8], 8], 6], [[[5, 7], 2], [[6, 5], [3, 0]]]],
        s![[[1, [2, 5]], 3], [5, [4, [6, 6]]]],
        s![[[[4, 9], 8], 1], [9, 0]],
        s![[1, [0, [5, 7]]], [[1, [5, 9]], [[3, 2], [1, 7]]]],
        s![
            [[[2, 9], [2, 7]], [[4, 2], 5]],
            [[[9, 1], [7, 2]], [2, [7, 5]]]
        ],
        s![[[[5, 7], [8, 9]], [5, [7, 9]]], [[7, [6, 6]], [7, [8, 0]]]],
        s![[[[6, 6], [4, 6]], [4, [7, 8]]], [1, [[5, 5], [1, 9]]]],
        s![[[[4, 3], 8], 2], [[9, [4, 0]], [8, [7, 0]]]],
        s![[2, [7, 5]], [[[0, 1], 1], [8, [3, 5]]]],
        s![[[4, [4, 2]], [[0, 4], 9]], [1, 4]],
        s![[[5, 5], [5, 6]], [[0, [4, 2]], [[7, 8], [5, 6]]]],
        s![2, [[0, [9, 1]], [[1, 7], [0, 0]]]],
        s![[[5, [4, 8]], 1], 9],
        s![8, [[2, 1], [3, 0]]],
        s![[[[6, 5], [1, 1]], 7], [[[7, 5], 3], [0, 1]]],
        s![[[[0, 3], 7], 7], [[[4, 8], [6, 1]], [[6, 1], 9]]],
        s![[[[4, 8], 9], [1, 0]], [6, [4, [4, 8]]]],
        s![[[[8, 0], [5, 1]], 6], 1],
        s![
            [[[6, 6], [7, 7]], [[4, 3], [2, 6]]],
            [[3, 5], [[7, 0], [7, 3]]]
        ],
        s![[1, [5, 8]], [[[3, 7], [9, 6]], [[4, 8], [3, 4]]]],
        s![[[1, 5], [8, 2]], [[[3, 1], 5], [4, 1]]],
        s![[[[6, 3], 5], 8], [[9, [3, 6]], [[3, 5], [6, 9]]]],
        s![[[7, [5, 4]], [0, [6, 0]]], [[[7, 7], [1, 1]], [[5, 1], 7]]],
        s![[[1, 5], [[8, 6], 0]], 5],
        s![[[[0, 8], [6, 0]], [[3, 0], 9]], [[[7, 1], 2], [4, 2]]],
        s![[[6, [8, 7]], [2, [2, 0]]], [9, [7, [6, 6]]]],
        s![3, [[7, [4, 5]], [[8, 5], 4]]],
        s![[[[8, 0], [8, 3]], [[5, 4], [1, 6]]], [[0, [8, 5]], 3]],
        s![[[7, 2], 1], [9, [[3, 8], 4]]],
        s![[4, [7, [9, 9]]], [3, 8]],
        s![[[[7, 1], 9], [[6, 9], [9, 6]]], [2, 0]],
        s![[[[6, 2], 9], [3, [3, 9]]], [[8, [3, 4]], [3, 7]]],
        s![[4, 9], [8, [5, [9, 8]]]],
        s![3, [[9, [9, 7]], 4]],
        s![[[[5, 9], 6], [1, [3, 1]]], [4, [1, [3, 8]]]],
        s![[[[7, 6], 2], 3], [[0, [1, 8]], [[4, 9], [4, 3]]]],
        s![[3, [[8, 1], [3, 8]]], [[[2, 0], [0, 8]], [[7, 0], 9]]],
        s![[[[9, 7], [9, 3]], [[5, 8], 6]], [[[6, 2], 0], [2, 4]]],
        s![[[8, [9, 7]], [[5, 1], [1, 4]]], 3],
        s![[7, [[5, 6], [2, 7]]], [[[7, 3], 0], [1, [0, 6]]]],
        s![[2, [[5, 5], 2]], [[3, [7, 2]], [[7, 1], 8]]],
        s![[[[2, 4], [6, 8]], [0, [7, 5]]], [[3, [2, 5]], [7, 7]]],
    ];
    let result = solve_a(input.clone());
    println!("{}", result);
    let result = solve_b(input.clone());
    println!("{}", result);
}

fn solve_a(input: Vec<SnailfishNumber>) -> usize {
    let result = input.into_iter().reduce(|a, b| a + b).unwrap();
    result.magnitude()
}

fn solve_b(mut input: Vec<SnailfishNumber>) -> usize {
    let mut largest_magnitude = 0;
    while let Some(popped) = input.pop() {
        for item in &input {
            let m1 = (popped.clone() + item.clone()).magnitude();
            let m2 = (item.clone() + popped.clone()).magnitude();
            let m = std::cmp::max(m1, m2);
            largest_magnitude = std::cmp::max(largest_magnitude, m);
        }
    }
    largest_magnitude
}

#[derive(Eq, PartialEq, Clone)]
enum SnailfishNumber {
    Raw(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    pub fn pair(a1: impl Into<SnailfishNumber>, a2: impl Into<SnailfishNumber>) -> Self {
        let a1 = a1.into();
        let a2 = a2.into();
        Self::Pair(Box::new(a1), Box::new(a2))
    }

    fn reduce(self) -> Self {
        let mut r = self;
        loop {
            if let Some(exploded) = r.explode() {
                r = exploded;
                continue;
            } else if let Some(splited) = r.split() {
                r = splited;
                continue;
            }
            break;
        }
        r
    }

    pub fn explode(&self) -> Option<Self> {
        fn explode_depth(s: &SnailfishNumber, depth: u8) -> Option<(u8, u8, SnailfishNumber)> {
            match s {
                SnailfishNumber::Raw(..) => None,
                SnailfishNumber::Pair(a1, a2) if depth == 4 => {
                    let v1 = match a1.deref() {
                        SnailfishNumber::Raw(v1) => *v1,
                        SnailfishNumber::Pair(..) => panic!("Unexpected pair"),
                    };
                    let v2 = match a2.deref() {
                        SnailfishNumber::Raw(v2) => *v2,
                        SnailfishNumber::Pair(..) => panic!("Unexpected pair"),
                    };
                    Some((v1, v2, SnailfishNumber::Raw(0)))
                }
                SnailfishNumber::Pair(a1, a2) => {
                    if let Some((l, r, s)) = explode_depth(a1, depth + 1) {
                        let new = SnailfishNumber::pair(s, a2.add_to_leftmost(r));
                        Some((l, 0, new))
                    } else if let Some((l, r, s)) = explode_depth(a2, depth + 1) {
                        let new = SnailfishNumber::pair(a1.add_to_rightmost(l), s);
                        Some((0, r, new))
                    } else {
                        None
                    }
                }
            }
        }
        if let Some((_l, _r, s)) = explode_depth(self, 0) {
            Some(s)
        } else {
            None
        }
    }

    fn add_to_leftmost(&self, n: u8) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(Box::new(l.add_to_leftmost(n)), r.clone())
            }
            SnailfishNumber::Raw(r) => SnailfishNumber::Raw(r + n),
        }
    }

    fn add_to_rightmost(&self, n: u8) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(l.clone(), Box::new(r.add_to_rightmost(n)))
            }
            SnailfishNumber::Raw(r) => SnailfishNumber::Raw(r + n),
        }
    }

    pub fn split(&self) -> Option<Self> {
        match self {
            Self::Raw(n) if *n >= 10 => Some(Self::pair(*n / 2, (*n + 1) / 2)),
            Self::Raw(_) => None,
            Self::Pair(a1, a2) => {
                if let Some(splited) = a1.split() {
                    Some(Self::Pair(Box::new(splited), a2.clone()))
                } else if let Some(splited) = a2.split() {
                    Some(Self::Pair(a1.clone(), Box::new(splited)))
                } else {
                    None
                }
            }
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Self::Raw(v) => *v as usize,
            Self::Pair(a1, a2) => 3 * a1.magnitude() + 2 * a2.magnitude(),
        }
    }
}

impl From<u8> for SnailfishNumber {
    fn from(v: u8) -> Self {
        SnailfishNumber::Raw(v)
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SnailfishNumber::pair(self, rhs).reduce()
    }
}

impl AddAssign for SnailfishNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = SnailfishNumber::pair(self.clone(), rhs);
    }
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(arg0) => write!(f, "{}", arg0),
            Self::Pair(arg0, arg1) => write!(f, "[{:?},{:?}]", arg0, arg1),
        }
    }
}

macro_rules! s {
    [$a1:literal, $a2:literal] => {
        SnailfishNumber::pair($a1, $a2)
    };
    [$a1:literal, [$($tt2:tt)*]] => {
        {
            let a2 = s![$($tt2)*];
            SnailfishNumber::pair($a1, a2)
        }
    };
    [[$($tt1:tt)*], $a2:literal] => {
        {
            let a1 = s![$($tt1)*];
            SnailfishNumber::pair(a1, $a2)
        }
    };
    [[$($tt1:tt)*], [$($tt2:tt)*]] => {
        {
            let a1 = s![$($tt1)*];
            let a2 = s![$($tt2)*];
            SnailfishNumber::pair(a1, a2)
        }
    };
}

pub(crate) use s;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let n1 = s![1, 2];
        assert_eq!(n1, SnailfishNumber::pair(1, 2));

        let n2 = s![1, [2, 3]];
        assert_eq!(n2, SnailfishNumber::pair(1, SnailfishNumber::pair(2, 3)));

        let n2 = s![[1, 2], 3];
        assert_eq!(n2, SnailfishNumber::pair(SnailfishNumber::pair(1, 2), 3));

        let n2 = s![[1, 2], [[3, 4], 5]];
        assert_eq!(
            n2,
            SnailfishNumber::pair(
                SnailfishNumber::pair(1, 2),
                SnailfishNumber::pair(SnailfishNumber::pair(3, 4), 5)
            )
        );
    }

    #[test]
    fn test_basic_add() {
        assert_eq!(s![1, 1] + s![2, 2], s![[1, 1], [2, 2]]);

        let mut s = s![1, 1];
        s += s![2, 2];
        s += s![3, 3];
        s += s![4, 4];
        assert_eq!(s, s![[[[1, 1], [2, 2]], [3, 3]], [4, 4]]);
    }

    #[test]
    fn test_explode() {
        let s = s![[[[[9, 8], 1], 2], 3], 4];
        assert_eq!(s.explode(), Some(s![[[[0, 9], 2], 3], 4]));

        let s = s![7, [6, [5, [4, [3, 2]]]]];
        assert_eq!(s.explode(), Some(s![7, [6, [5, [7, 0]]]]));

        let s = s![[6, [5, [4, [3, 2]]]], 1];
        assert_eq!(s.explode(), Some(s![[6, [5, [7, 0]]], 3]));

        let s = s![[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]];
        assert_eq!(
            s.explode(),
            Some(s![[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]])
        );

        let s = s![[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]];
        assert_eq!(s.explode(), Some(s![[3, [2, [8, 0]]], [9, [5, [7, 0]]]]));
    }

    #[test]
    fn test_split() {
        let s = s![1, 10];
        assert_eq!(s.split(), Some(s![1, [5, 5]]));

        let s = s![1, 11];
        assert_eq!(s.split(), Some(s![1, [5, 6]]));

        let s = s![[[[0, 7], 4], [15, [0, 13]]], [1, 1]];
        assert_eq!(
            s.split(),
            Some(s![[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]])
        );

        let s = s![[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]];
        assert_eq!(
            s.split(),
            Some(s![[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]])
        );
    }

    #[test]
    fn test_complicated_addition() {
        let s = s![[[[4, 3], 4], 4], [7, [[8, 4], 9]]] + s![1, 1];
        assert_eq!(s, s![[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]);
    }

    #[test]
    fn test_magnitude() {
        let s = s![[1, 2], [[3, 4], 5]];
        assert_eq!(s.magnitude(), 143);

        let s = s![[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]];
        assert_eq!(s.magnitude(), 1384);

        let s = s![[[[1, 1], [2, 2]], [3, 3]], [4, 4]];
        assert_eq!(s.magnitude(), 445);

        let s = s![[[[3, 0], [5, 3]], [4, 4]], [5, 5]];
        assert_eq!(s.magnitude(), 791);

        let s = s![[[[5, 0], [7, 4]], [5, 5]], [6, 6]];
        assert_eq!(s.magnitude(), 1137);

        let s = s![
            [[[8, 7], [7, 7]], [[8, 6], [7, 7]]],
            [[[0, 7], [6, 6]], [8, 7]]
        ];
        assert_eq!(s.magnitude(), 3488);
    }

    #[test]
    fn test_larger_example() {
        let input = vec![
            s![[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]],
            s![7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]],
            s![[2, [[0, 8], [3, 4]]], [[[6, 7], 1], [7, [1, 6]]]],
            s![
                [[[2, 4], 7], [6, [0, 5]]],
                [[[6, 8], [2, 8]], [[2, 1], [4, 5]]]
            ],
            s![7, [5, [[3, 8], [1, 4]]]],
            s![[2, [2, 2]], [8, [8, 1]]],
            s![2, 9],
            s![1, [[[9, 3], 9], [[9, 0], [0, 7]]]],
            s![[[5, [7, 4]], 7], 1],
            s![[[[4, 2], 2], 6], [8, 7]],
        ];
        let result = input.into_iter().reduce(|a, b| a + b).unwrap();
        assert_eq!(
            result,
            s![
                [[[8, 7], [7, 7]], [[8, 6], [7, 7]]],
                [[[0, 7], [6, 6]], [8, 7]]
            ]
        );
    }

    #[test]
    fn test_solve_a() {
        let input = vec![
            s![[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]],
            s![[[5, [2, 8]], 4], [5, [[9, 9], 0]]],
            s![6, [[[6, 2], [5, 6]], [[7, 6], [4, 7]]]],
            s![[[6, [0, 7]], [0, 9]], [4, [9, [9, 0]]]],
            s![[[7, [6, 4]], [3, [1, 3]]], [[[5, 5], 1], 9]],
            s![[6, [[7, 3], [3, 2]]], [[[3, 8], [5, 7]], 4]],
            s![[[[5, 4], [7, 7]], 8], [[8, 3], 8]],
            s![[9, 3], [[9, 9], [6, [4, 9]]]],
            s![[2, [[7, 7], 7]], [[5, 8], [[9, 3], [0, 2]]]],
            s![[[[5, 2], 5], [8, [3, 7]]], [[5, [7, 5]], [4, 4]]],
        ];
        let result = solve_a(input);
        assert_eq!(result, 4140);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            s![[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]],
            s![[[5, [2, 8]], 4], [5, [[9, 9], 0]]],
            s![6, [[[6, 2], [5, 6]], [[7, 6], [4, 7]]]],
            s![[[6, [0, 7]], [0, 9]], [4, [9, [9, 0]]]],
            s![[[7, [6, 4]], [3, [1, 3]]], [[[5, 5], 1], 9]],
            s![[6, [[7, 3], [3, 2]]], [[[3, 8], [5, 7]], 4]],
            s![[[[5, 4], [7, 7]], 8], [[8, 3], 8]],
            s![[9, 3], [[9, 9], [6, [4, 9]]]],
            s![[2, [[7, 7], 7]], [[5, 8], [[9, 3], [0, 2]]]],
            s![[[[5, 2], 5], [8, [3, 7]]], [[5, [7, 5]], [4, 4]]],
        ];
        let result = solve_b(input);
        assert_eq!(result, 3993);
    }
}
