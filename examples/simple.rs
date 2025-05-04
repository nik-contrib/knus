use std::io::Read;

use miette::IntoDiagnostic;

#[derive(ferrishot_knus::Decode, Debug)]
#[allow(dead_code)]
struct Plugin {
    #[ferrishot_knus(argument)]
    name: String,
    #[ferrishot_knus(property)]
    url: String,
    #[ferrishot_knus(child, unwrap(argument))]
    version: String,
}

#[derive(ferrishot_knus::Decode, Debug)]
#[allow(dead_code)]
struct Config {
    #[ferrishot_knus(child, unwrap(argument))]
    version: String,
    #[ferrishot_knus(children(name = "plugin"))]
    plugins: Vec<Plugin>,
}

fn main() -> miette::Result<()> {
    let mut buf = String::new();
    println!("Please type KDL document, press Return, Ctrl+D to finish");
    std::io::stdin()
        .read_to_string(&mut buf)
        .into_diagnostic()?;
    let cfg: Config = ferrishot_knus::parse("<stdin>", &buf)?;
    println!("{:#?}", cfg);
    Ok(())
}
