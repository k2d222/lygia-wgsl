fn main() {
    let source = wesl::Wesl::new("src/shaders")
        .add_package(&lygia_wgsl::PACKAGE)
        .set_options(wesl::CompileOptions {
            validate: true,
            lower: true,
            lazy: false,
            strip: false,
            ..Default::default()
        })
        .compile(&"package::main".parse().unwrap())
        .inspect_err(|e| {
            eprintln!("{e}");
            panic!();
        })
        .unwrap()
        .to_string();

    println!("{source}")
}
