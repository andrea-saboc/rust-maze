//Andrea Sabo Cibolja E2 91/2022

use std::fs::File;
use std::io::{self, BufRead};
use std::io::ErrorKind;

#[derive(Debug, Clone)]
struct Lavirint{
    br_vrsta: i8,
    br_kolona: i8,
    lavirint: Vec<Vec<Polje>>,
}

impl Lavirint{
    fn new() -> Self{
        Lavirint { br_vrsta: 0, br_kolona: 0, lavirint: Vec::new() }
    }

    fn ucitaj_iz_fajla(&mut self) {
        self.br_kolona = 9; //zadato
        self.br_vrsta = 0; //ucitavamo iz fajla

        let file_result = File::open("./src/lavirint.txt");
        
        let opened_file = match file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {   panic!("File not found!");
                },
                other_error => {
                    panic!("Problem opening the file : {:?}", other_error);
                }
            }

        };
        let mut trenutna_kolona = 0;
        let lines = io::BufReader::new(opened_file).lines();
        for line in lines  {
            if trenutna_kolona == 0 {
                self.lavirint.push(Vec::new());
                self.br_vrsta = self.br_vrsta+1;
            }
            

            let s = line.unwrap().replace(" ", "");
            let s1: String = s.chars().skip(0).take(4).collect(); //na koju stranu može da se kreće
            let s2: String = s.chars().skip(4).take(4).collect(); //da li postoje vrata
            let s3: String = s.chars().skip(8).take(2).collect(); //kljuc
            let s4: String = s.chars().skip(10).take(2).collect(); //izlaz


            let mut polje = Polje::new();

            polje.pozicija.kolona = trenutna_kolona;
            polje.pozicija.vrsta = self.br_vrsta-1;
    
            if s1.chars().nth(0).unwrap()=='1' {
                if s2.chars().nth(0).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona-1));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona-1));
                }
            } 
            if s1.chars().nth(1).unwrap()=='1' {
                if s2.chars().nth(1).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona+1));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona+1));
                }
            }
            if s1.chars().nth(2).unwrap()=='1' {
                if s2.chars().nth(2).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta-1, polje.pozicija.kolona));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta-1, polje.pozicija.kolona));
                }
            }
            if s1.chars().nth(3).unwrap()=='1' {
                if s2.chars().nth(3).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta+1, polje.pozicija.kolona));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta+1, polje.pozicija.kolona));
                }
            }

            if s3.eq("11") {
                polje.kljuc = true;
            }
            if s4.eq("11") {
                polje.izlaz = true;
            }

            self.lavirint.last_mut().unwrap().push(polje);
            trenutna_kolona = (trenutna_kolona+1)%9;
        }

        print!("Broj vrsta na kraju ucitavanja je {}, a broj kolona {}!", self.br_vrsta, self.br_kolona);
    }

    fn ispisi_lavirint(&mut self){
        print!("\n*************\n");
        for row in &self.lavirint{
            for col in row  {
                print!("{:?}\n", col);
            }
        }
        print!("*************\n");
    }

}

#[derive(Debug, Clone, Copy)]
struct Pozicija{
    vrsta: i8,
    kolona: i8,
}

#[derive(Debug, Clone)]
struct Trenutna{
    trenutna_pozicija: Pozicija,
    kljucevi: Vec<Kljuc>,
    neiskorisceni_kljucevi: Vec<Pozicija>,

}

impl Pozicija{
    fn new(brV: i8, brK: i8)->Self{
        Pozicija { vrsta: (brV), kolona: (brK) }
    }
}

impl Trenutna{
    fn new() -> Self{
        Trenutna {
            trenutna_pozicija: Pozicija::new(0, 0),
            kljucevi: Vec::new(),
            neiskorisceni_kljucevi: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Kljuc{
    sa_pozicije: Pozicija,
    iskorisceno_na: Pozicija,
    mesto: i8,
}

#[derive(Debug, Clone)]
struct Polje{
    pozicija: Pozicija,
    slobodan_prolaz: Vec<Pozicija>,
    vrata_prolaz: Vec<Pozicija>,
    kljuc: bool,
    izlaz: bool,
}

impl Polje {
    fn new()->Self{
        Polje { 
            pozicija: Pozicija::new(-1,-1), 
            slobodan_prolaz: Vec::new(), 
            vrata_prolaz: Vec::new(), 
            kljuc: false, 
            izlaz: false
        }
    }
    
}



fn main() {
    let mut lavirint = Lavirint::new();
    lavirint.ucitaj_iz_fajla();
    lavirint.ispisi_lavirint();
}