use std::ascii::AsciiExt;

fn foo() -> Vec<char> {
    let mut data = vec!['a', 'b', 'c'];
    {
            let slice = &mut data;

    capitalize(slice);
    }
    

    data.push('d');
    data.push('e');
    data.push('f');
    data
}

fn capitalize(data: &mut [char]) {
    for c in data {
        c.make_ascii_uppercase();
    }
}


fn main() {
    let v = foo();
    println!("{:?}", v);
}
