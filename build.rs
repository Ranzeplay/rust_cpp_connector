fn main() {
    cxx_build::bridge("src/bridge.rs")
        .file("src/os_process.cpp")
        .flag_if_supported("-std=c++23")
        .compile("cxxbridge-result");
}