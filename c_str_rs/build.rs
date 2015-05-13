fn main(){
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rustc-link-search=native=./c/");
}
