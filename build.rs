fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("emarfcore_rs_example")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();
}

