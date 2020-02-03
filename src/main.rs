mod cs;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILES")]
    input: Vec<PathBuf>,

    #[structopt(short, long)]
    format: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let mut fields = cs::Fields::with_capacity(4);
    fields.insert(cs::Variable {
        t: cs::Type::I32,
        name: "a".to_string(),
    });
    fields.insert(cs::Variable {
        t: cs::Type::String,
        name: "b".to_string(),
    });
    fields.insert(cs::Variable {
        t: cs::Type::Unknown,
        name: "c".to_string(),
    });
    let class = cs::Class {
        namespace: vec!["Example".to_owned(), "Namespace".to_owned()],
        name: "ExampleClass".to_string(),
        fields,
        methods: Default::default(),
    };
    println!("{}", class);
}
