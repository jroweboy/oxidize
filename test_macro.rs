#![feature(phase)]
#[phase(syntax)]
extern crate oxidize_macros;


pub trait App {
    fn default1(&self) {
        println!("called the default impl 1");
    }
    fn default2(&self) {
        println!("called the default impl 2");
    }
    fn default3(&self) {
        println!("called the default impl 3");
    }
}

struct HelloMacro;

router!(HelloMacro, 

    fn default2(){
        println!("Overwritten default 2");
    }    
)


fn main() {
    let test = HelloMacro;
    test.default1();
    test.default2();
    test.default3();
}