fn main() {
    let mut cont = 0;

    let result = loop {
        cont += 1;

        if cont == 10 {
            break cont * 2;
        }
    };

    println!("O resultado é {result}")
}
