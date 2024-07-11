fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/domain/game.rs")
        .input_extern_file("src/domain/cell.rs")
        .csharp_dll_name("dynamic_tic_tac_toe")
        .csharp_class_accessibility("public")
        .csharp_use_function_pointer(false)
        .generate_csharp_file("dotnet/NativeMethods.g.cs")
        .unwrap();
}