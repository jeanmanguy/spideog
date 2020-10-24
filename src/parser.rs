use nom::IResult;

use crate::kraken::Indent;

pub fn spaces_and_rest(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    nom::multi::fold_many0(
        nom::bytes::complete::tag("  "),
        Vec::new(),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

pub fn parse_ident_organism_name(input: &[u8]) -> IResult<&[u8], (Indent, &[u8])> {
    let (name, spaces) = spaces_and_rest(input).unwrap();

    Ok((&[], (spaces.len(), name)))
}
