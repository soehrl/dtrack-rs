fn main() {
    println!("cargo:rerun-if-changed=src/dtrack.cpp");
    println!("cargo:rerun-if-changed=DTrackSDK");
    println!("cargo:rerun-if-changed=build.rs");
    cc::Build::new()
        .cpp(true)
        .include("DTrackSDK/include")
        .file("src/dtrack.cpp")
        .file("DTrackSDK/src/DTrackData.cpp")
        .file("DTrackSDK/src/DTrackNet.cpp")
        .file("DTrackSDK/src/DTrackParse.cpp")
        .file("DTrackSDK/src/DTrackParser.cpp")
        .file("DTrackSDK/src/DTrackSDK.cpp")
        .compile("dtrack_c_wrapper");
}
