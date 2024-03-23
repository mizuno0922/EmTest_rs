fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("emarfcore_rs_example")
        .csharp_class_name("NativeMethods")
        .csharp_use_nint_types(false)
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();
}

