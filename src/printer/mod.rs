use std::{fmt::Write, fs::File, io::Write as ioWrt, path::Path};

use crate::ast::{Signal, Spec};

pub fn print_module(path: &Path, spec: Spec, decs: Vec<Signal>) -> std::io::Result<()> {
    let file_path = path.join(spec.name.to_owned() + ".vhd");
    let template_path = path.join(spec.name.to_owned() + ".vho");

    let generics = if spec.generics.is_some() { &spec.generics.unwrap()} else {&vec![]};

    //--------------------Templates---------------------//
    let mut component_declaration = String::new();
    writeln!(component_declaration, "-- Begin Component Declaration").unwrap();
    writeln!(component_declaration, "component {}", spec.name).unwrap();

    if !generics.is_empty() {
        writeln!(component_declaration, "generic").unwrap();
        writeln!(component_declaration, "(").unwrap();

        write!(
            component_declaration,
            "{}\n",
            generics
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<_>>()
                .join(",\n")
        )
        .unwrap();

        writeln!(component_declaration, ");").unwrap();
    }
    writeln!(component_declaration, "port").unwrap();
    writeln!(component_declaration, "(").unwrap();

    write!(
        component_declaration,
        "{}\n",
        spec.io
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
            .join(",\n")
    )
    .unwrap();

    writeln!(component_declaration, ");").unwrap();
    writeln!(component_declaration, "end component;").unwrap();
    writeln!(component_declaration, "-- End Component Declaration\n").unwrap();

    let mut instantiation_template = String::new();
    writeln!(instantiation_template, "-- Begin Instantiation Template").unwrap();
    writeln!(instantiation_template, "instance_name : {}", spec.name).unwrap();
    if !generics.is_empty()  {
        writeln!(instantiation_template, "generic map").unwrap();
        writeln!(instantiation_template, "(").unwrap();
        write!(
                instantiation_template,
                "\t{}\t=> \n",
                generics
                    .iter()
                    .map(|g| g.name)
                    .collect::<Vec<_>>()
                    .join("\t=> ,\n\t")
            )
            .unwrap();
        writeln!(instantiation_template, ")").unwrap();
    }
    writeln!(instantiation_template, "port map").unwrap();
    writeln!(instantiation_template, "(").unwrap();
    write!(
        instantiation_template,
        "\t{}\t=> \n",
        spec.io
            .iter()
            .map(|g| g.name)
            .collect::<Vec<_>>()
            .join("\t=> ,\n\t")
    )
    .unwrap();
    writeln!(instantiation_template, ");").unwrap();
    writeln!(instantiation_template, "-- End Instantiation Template\n").unwrap();

    let templates = component_declaration + &instantiation_template;
    let mut template_file = File::create(template_path)?;
    template_file.write_all(templates.as_bytes())?;

    //------------------------Module--------------------//

    let mut entity_declaration = String::new();
    writeln!(entity_declaration, "entity {} is", spec.name).unwrap();
    if !generics.is_empty()  {
        writeln!(entity_declaration, "generic").unwrap();
        writeln!(entity_declaration, "(").unwrap();
        write!(
            entity_declaration,
            "{}\n",
            generics
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<_>>()
                .join(",\n")
        )
        .unwrap();
        writeln!(entity_declaration, ");").unwrap();
    }
    writeln!(entity_declaration, "port").unwrap();
    writeln!(entity_declaration, "(").unwrap();
    write!(
        entity_declaration,
        "{}\n",
        spec.io
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
            .join(",\n")
    )
    .unwrap();
    writeln!(entity_declaration, ");").unwrap();
    writeln!(entity_declaration, "end {};\n", spec.name).unwrap();

    let mut arch_declaration = String::new();
    writeln!(
        arch_declaration,
        "architecture implementation of {} is",
        spec.name
    )
    .unwrap();
    writeln!(arch_declaration, "\t-- declarations").unwrap();
    for sig in decs {
        writeln!(arch_declaration, "\tsignal {};", sig).unwrap();
    }
    writeln!(arch_declaration, "begin").unwrap();
    writeln!(arch_declaration, "\t-- contents").unwrap();
    //writeln!(arch_declaration, "{}", join_lines(&AsyncSync{ id: "sync1", active_low: false, falling_edge: false, stages: 4, sig_in:& Signal{name: "rst", typ: ()}, sig_out: & Signal{name: "rst_out", typ: ()}, clk: ()}.generate_rtl())).unwrap();
    writeln!(arch_declaration, "end implementation;\n").unwrap();

    let vhdl_content = entity_declaration + &arch_declaration;
    let mut file = File::create(file_path)?;
    file.write_all(vhdl_content.as_bytes())?;

    Ok(())
}


pub fn indent(lines: &[String], level: usize) -> Vec<String> {
    let prefix = " ".repeat(level * 4);
    lines.iter()
        .map(|line| format!("{}{}", prefix, line))
        .collect()
}

/// Join lines into a single String separated by newlines (`\n`).
pub fn join_lines(lines: &[String]) -> String {
    lines.join("\n")
}


/// Simple template replacement.
/// Replace occurrences of `{{key}}` in the template with the provided value.
pub fn apply_template(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut output = template.to_string();
    for (key, value) in replacements {
        let placeholder = format!("{{{{{}}}}}", key);
        output = output.replace(&placeholder, value);
    }
    output
}