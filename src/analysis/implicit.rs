use crate::types;
use crate::types::*;

// from => [into]
pub fn implicit_conversions(from: &AtomKind) -> Vec<AtomKind> {
    // AtomKind::Any conversions (anything convert to these)
    let mut results = vec![
        AtomKind::Dynamic,
        AtomKind::Atom(types::Str.clone()),
        // Const(T) (TODO! this is a bit of a hack, but it works for now (C backend const pointers))
        AtomKind::Atom(types::Const.spec(&[AtomType {
            kind: from.clone(),
            details: None,
        }])),
    ];

    // reset of conversions for each specific type
    results.append(&mut match from {
        AtomKind::Atom(ref atom) if atom == &*types::Const => vec![atom.generics[0].kind.clone()],

        AtomKind::Basic(BasicType::Int) => vec![AtomKind::Basic(BasicType::Float)],

        AtomKind::Dynamic => vec![AtomKind::Any],
        _ => Vec::new(),
    });

    results
}

pub fn can_implicitly_convert(from: &AtomKind, to: &AtomKind) -> bool {
    let conversions = implicit_conversions(from);

    conversions.contains(to) || conversions.contains(&AtomKind::Any)
}
