fn main() {
    let dtrack = dtrack_rs::DTrack::with_connection(&std::env::args().nth(1).unwrap()).unwrap();

    println!("is valid: {}", dtrack.is_valid());
    println!("is data interface valid: {}", dtrack.is_data_interface_valid());
}
