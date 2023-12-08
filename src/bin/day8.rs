use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::{alpha1, alphanumeric1},
    combinator::{delimited, separated, separated_pair},
    BStr, PResult, Parser,
};

type Node = [u8; 3];
type Map = HashMap<Node, (Node, Node)>;

fn parse_node(input: &mut &BStr) -> PResult<Node> {
    alphanumeric1.try_map(Node::try_from).parse_next(input)
}

fn parse_map(input: &mut &BStr) -> PResult<Map> {
    separated(
        1..,
        separated_pair(
            parse_node,
            " = ",
            delimited("(", separated_pair(parse_node, ", ", parse_node), ")"),
        ),
        "\n",
    )
    .parse_next(input)
}

fn parse_input<'s>(input: &mut &'s BStr) -> PResult<(&'s [u8], Map)> {
    separated_pair(alpha1, "\n\n", parse_map).parse_next(input)
}

#[aoc_macro::main("day8")]
fn main() -> color_eyre::Result<u32> {
    let (inst, map) = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut nodes: Vec<(&Node, &(Node, Node))> = map.iter().filter(|(n, _)| n[2] == b'A').collect();
    let steps = nodes
        .iter_mut()
        .map(|m| {
            for (steps, i) in inst.iter().cycle().enumerate() {
                if m.0[2] == b'Z' {
                    return steps;
                }
                match i {
                    b'R' => {
                        *m = (
                            &m.1 .1,
                            map.get(&m.1 .1).unwrap_or_else(|| {
                                panic!("no right node: {}", String::from_utf8_lossy(&m.1 .1))
                            }),
                        );
                    }
                    b'L' => {
                        *m = (
                            &m.1 .0,
                            map.get(&m.1 .0).unwrap_or_else(|| {
                                panic!("no left node: {}", String::from_utf8_lossy(&m.1 .0))
                            }),
                        );
                    }
                    _ => panic!("unknown instruction: {}", String::from_utf8_lossy(&[*i])),
                }
            }
            unreachable!("no Z node found")
        })
        .reduce(num::integer::lcm)
        .unwrap();

    Ok(steps)
}
