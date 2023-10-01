use std::io;

fn main() {
    
    clearscreen::clear().unwrap();

    loop {
        pileouface();
    }
}

fn pileouface() {
    
    let mut rep = String::new();

    println!("pile ou face ?");

    io::stdin()
        .read_line(&mut rep)
        .expect("Impossible de lire la valeur");

    clearscreen::clear().unwrap();
    
    rep = rep.trim().to_lowercase();

    if rep == "pile" {
        println!("C'était face cheh !");
    } else if rep == "face" {
        println!("C'était pile cheh!");
    } else {
        println!("Réponse invalide : {}", rep);
    }

    
}
