use std::collections::{HashMap, HashSet};
use syn::*;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <StructName>", args[0]);
        std::process::exit(1);
    }

    let root_struct = &args[1];
    let structs = parse_all_structs("src");

    let mut visited = HashSet::new();
    let mut dot_nodes = Vec::new();
    let mut dot_edges = Vec::new();

    build_graph(
        root_struct,
        &structs,
        &mut visited,
        &mut dot_nodes,
        &mut dot_edges,
    );

    // Output Graphviz DOT
    println!("digraph G {{");
    println!("  node [shape=record, fontname=Helvetica];");
    for node in dot_nodes {
        println!("  {node}");
    }
    for edge in dot_edges {
        println!("  {edge}");
    }
    println!("}}");
}

fn parse_all_structs(path: &str) -> HashMap<String, Vec<(String, String)>> {
    let mut structs = HashMap::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|r| r.ok()) {
        if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
            let code = std::fs::read_to_string(entry.path()).unwrap();
            let file = syn::parse_file(&code).unwrap();
            for item in file.items {
                if let Item::Struct(s) = item {
                    let name = s.ident.to_string();
                    let mut fields_list = Vec::new();
                    if let Fields::Named(fields) = s.fields {
                        for field in fields.named {
                            let fname = field.ident.unwrap().to_string();
                            let ftype = type_to_string(&field.ty);
                            fields_list.push((fname, ftype));
                        }
                    }
                    structs.insert(name, fields_list);
                }
            }
        }
    }
    structs
}

fn build_graph(
    name: &str,
    structs: &HashMap<String, Vec<(String, String)>>,
    visited: &mut HashSet<String>,
    dot_nodes: &mut Vec<String>,
    dot_edges: &mut Vec<String>,
) {
    if visited.contains(name) {
        return;
    }
    visited.insert(name.to_string());

    if let Some(fields) = structs.get(name) {
        // Create UML-style record node
        let mut label = format!("{{{name}|");
        for (fname, ftype) in fields {
            label.push_str(&format!("{fname}: {ftype}\\l"));
        }
        label.push('}');

        dot_nodes.push(format!("{name} [label=\"{label}\"];"));

        // Follow dependencies
        for (_, ftype) in fields {
            for inner in extract_inner_types(ftype) {
                if structs.contains_key(&inner) {
                    dot_edges.push(format!("{} -> {};", name, inner));
                    build_graph(&inner, structs, visited, dot_nodes, dot_edges);
                }
            }
        }
    }
}

fn type_to_string(ty: &Type) -> String {
    quote::quote!(#ty)
        .to_string()
        .replace("<", "\\<")
        .replace(">", "\\>")
}
/// Recursively extract all custom type names from a type string.
/// This ignores standard library types and primitive types.
fn extract_inner_types(ty: &str) -> Vec<String> {
    let ty = ty.trim();

    // Handle Rust arrays like [T; N]
    if ty.starts_with('[') && ty.ends_with(']') {
        if let Some(inner) = ty.strip_prefix('[').and_then(|s| s.split(';').next()) {
            return extract_inner_types(inner);
        }
    }

    // Handle generics like Option<T>, Vec<T>, Array<T>, Result<T, E>, etc.
    if let Some(start) = ty.find('<') {
        if let Some(end) = ty.rfind('>') {
            let inner = &ty[start + 1..end];
            return inner
                .split(',')
                .flat_map(|s| extract_inner_types(s))
                .collect();
        }
    }

    // Handle tuples (T1, T2, ...)
    if ty.starts_with('(') && ty.ends_with(')') {
        return ty[1..ty.len() - 1]
            .split(',')
            .flat_map(|s| extract_inner_types(s))
            .collect();
    }

    // Base case: just return the type name itself
    vec![ty.to_string()]
}
