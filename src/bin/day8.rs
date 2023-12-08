use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::alpha1,
    combinator::{delimited, separated, separated_pair},
    BStr, PResult, Parser,
};

type Node = [u8; 3];
type Map = HashMap<Node, (Node, Node)>;

fn parse_node(input: &mut &BStr) -> PResult<Node> {
    alpha1.try_map(Node::try_from).parse_next(input)
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

    let mut node = map.get(b"AAA").ok_or(eyre!("no starting node"))?;
    let mut steps = 0;
    for i in inst.iter().cycle() {
        let (left, right) = node;
        steps += 1;

        match i {
            b'R' => {
                if right == b"ZZZ" {
                    break;
                }
                node = map
                    .get(right)
                    .ok_or(eyre!("no right node: {}", String::from_utf8_lossy(right)))?;
            }
            b'L' => {
                if left == b"ZZZ" {
                    break;
                }
                node = map
                    .get(left)
                    .ok_or(eyre!("no left node: {}", String::from_utf8_lossy(left)))?;
            }
            _ => panic!("unknown instruction: {}", String::from_utf8_lossy(&[*i])),
        }
    }

    Ok(steps)
}
