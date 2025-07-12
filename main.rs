fn main() {
    let data = [10, 20, 30, 40, 50];
    let target = 30;

    let mut found = false;

    for i in 0..data.len() {
        if data[i] == target {
            println!("Angka {} ditemukan di indeks {}", target, i);
            found = true;
            break;
        }
    }

    if !found {
        println!("Angka {} tidak ditemukan", target);
    }
}
