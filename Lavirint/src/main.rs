//Andrea Sabo Cibolja E2 91/2022

use std::fs::File;
use std::io::{self, BufRead};
use std::io::ErrorKind;
use std::vec;
use std::collections::VecDeque;
use std::collections::HashSet;

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
    
            if s1.chars().nth(0).unwrap()=='1' { //zapad
                if s2.chars().nth(0).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona-1));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona-1));
                }
            } 
            if s1.chars().nth(1).unwrap()=='1' { //istok
                if s2.chars().nth(1).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona+1));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta, polje.pozicija.kolona+1));
                }
            }
            if s1.chars().nth(2).unwrap()=='1' { //sever
                if s2.chars().nth(2).unwrap()=='1' {
                    polje.vrata_prolaz.push(Pozicija::new(polje.pozicija.vrsta-1, polje.pozicija.kolona));
                } else {
                    polje.slobodan_prolaz.push(Pozicija::new(polje.pozicija.vrsta-1, polje.pozicija.kolona));
                }
            }
            if s1.chars().nth(3).unwrap()=='1' { //jug
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

    fn dobavi_polje_na_indeksu(&self, vrsta: i8, kolona: i8) -> &Polje{
        return self.lavirint.get(vrsta as usize).unwrap().get(kolona as usize).unwrap();
    }

    fn postoje_vrata(&self, sa_indeksa: Pozicija, na_indeks: Pozicija) ->bool{
        let sa_polja = self.dobavi_polje_na_indeksu(sa_indeksa.vrsta, sa_indeksa.kolona);
        for vrata in sa_polja.vrata_prolaz.clone() {
            if vrata == na_indeks {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            neiskorisceni_kljucevi: Vec::new(),
        }
    }

    fn poseduje_kljuc(&self)-> bool{
        for kljuc in &self.kljucevi{
            if kljuc.sa_pozicije.vrsta==self.trenutna_pozicija.vrsta && kljuc.sa_pozicije.kolona==self.trenutna_pozicija.kolona{
                return true;   
            }
        }
        return false;
    }

    fn dobavi_legalne_pozicije(&self, lavirint: &Lavirint)->Vec<Pozicija>{
        let polje = lavirint.dobavi_polje_na_indeksu(self.trenutna_pozicija.vrsta, self.trenutna_pozicija.kolona);
        let mut legalne_pozicije: Vec<Pozicija> = Vec::new();
        let mut moguce_pozicije= polje.slobodan_prolaz.clone();
        let mut moguce_sa_vratima = polje.vrata_prolaz.clone();
        legalne_pozicije.append(&mut moguce_pozicije);
        if self.neiskorisceni_kljucevi.len()>=1 {
            legalne_pozicije.append(&mut moguce_sa_vratima);
        }
        return legalne_pozicije;
    }

    fn dobavi_hash_stanje(&self, vrsta: i8, kolona: i8) ->String{
        let mut hash_stanja = String::from("");
        hash_stanja=
        "[".to_owned()
        +&vrsta.to_string()
        +","
        +&kolona.to_string()
        +"]";
        for kljuc in &self.kljucevi{
            hash_stanja=
            "[".to_owned()
            +&hash_stanja
            +&kljuc.sa_pozicije.vrsta.to_string()
            +","
            +&kljuc.sa_pozicije.kolona.to_string()
            +"]";
        }
        return hash_stanja;

    }

    
    fn resavanje(&mut self,mut obradjena_stanja:&mut VecDeque<Trenutna>,mut set_obradjenih_stanja:HashSet<String>, lavirint: &Lavirint, mut put:&mut VecDeque<Trenutna>, mut putevi:&mut VecDeque<VecDeque<Trenutna>> ) -> bool{

        let tren = self.clone();
        put.push_back(tren.clone());
        obradjena_stanja.push_back(tren);
        
        if lavirint.dobavi_polje_na_indeksu(self.trenutna_pozicija.vrsta, self.trenutna_pozicija.kolona).izlaz {
           return true; //pronadjen je izlaz 
        }
        
        

        let new_positions = self.dobavi_legalne_pozicije(lavirint); //dobavlja sve moguce legalne pozicije iz trenutne pozicije
            for next in new_positions  {
                if  !set_obradjenih_stanja.contains(&self.dobavi_hash_stanje(next.vrsta, next.kolona)){

                    let mut novo_stanje = self.clone();
                    let nextPolje= lavirint.dobavi_polje_na_indeksu(next.vrsta, next.kolona);

                    if lavirint.postoje_vrata(self.trenutna_pozicija, next){
                        let kljuc = novo_stanje.neiskorisceni_kljucevi.pop();
                        match kljuc {
                            None => {continue;},
                            Some(vrednost) =>{
                            }
                        }
                        for mut klj in &mut novo_stanje.kljucevi  {
                            if klj.sa_pozicije==kljuc.unwrap() {
                                klj.iskorisceno_na = next;
                            }
                        }

                    }
                    novo_stanje.trenutna_pozicija=next;

                    if lavirint.dobavi_polje_na_indeksu(novo_stanje.trenutna_pozicija.vrsta, novo_stanje.trenutna_pozicija.kolona).kljuc && !novo_stanje.poseduje_kljuc(){ //kupimo kljuc ako vec nismo
                        novo_stanje.kljucevi.push(Kljuc::new(novo_stanje.trenutna_pozicija.vrsta, novo_stanje.trenutna_pozicija.kolona));
                        novo_stanje.neiskorisceni_kljucevi.push(Pozicija::from(novo_stanje.trenutna_pozicija));
                    }
                    set_obradjenih_stanja.insert(novo_stanje.dobavi_hash_stanje(novo_stanje.trenutna_pozicija.vrsta, self.trenutna_pozicija.kolona)); 

                    let reseno = novo_stanje.resavanje(&mut obradjena_stanja, set_obradjenih_stanja.clone(), lavirint,&mut put, &mut putevi);
                    
                    if reseno {
                        putevi.push_back(put.clone());   
                    } 

                    put.pop_back();
                
                }
            }
            
            return false;
            
    }
    
    

    
    fn prolazi_kroz_lavirint(&mut self, lavirint: Lavirint){

            
            let mut set_stanja:HashSet<String>=HashSet::new();
            set_stanja.insert(self.dobavi_hash_stanje(self.trenutna_pozicija.vrsta, self.trenutna_pozicija.kolona)); //hashovi stanja za obradu

            let mut obradjena_stanja:VecDeque<Trenutna> = VecDeque::new();
            let mut set_obradjenih_stanja:HashSet<String>=HashSet::new(); //cuvamo sve hash kodove stanja u kojima smo bili, kako ne bismo dva puta bili u istom stanju
            let mut put:VecDeque<Trenutna> = VecDeque::new();
            let mut putevi:VecDeque<VecDeque<Trenutna>> = VecDeque::new();

            self.resavanje(&mut obradjena_stanja, set_obradjenih_stanja, &lavirint, &mut put, &mut putevi);
            if putevi.len()>0 {
                print!("Pronadjen je izlaz!");
                print!("\n\nPronadjeno puteva: ({}).",putevi.len());

            let mut najkraci = putevi.pop_front().unwrap();
            for put in putevi  {
                if put.len() <najkraci.len(){
                    najkraci = put;
                }
            }
           // print!("{:?}", najkraci);
           print!("\n\nPut kojim prolazi({}):",najkraci.len());
            
            for stanje in najkraci {
                print!("\n{:?}", stanje.trenutna_pozicija);
            }
            } else {
                print!("Nije pronadjen izlaz!");
            }

            
            

    }
}



#[derive(Debug, Clone, Copy)]
struct Kljuc{
    sa_pozicije: Pozicija,
    iskorisceno_na: Pozicija,
}

impl Kljuc {
    fn new(vrsta: i8, kolona: i8) ->Self{
        Kljuc { 
            sa_pozicije: Pozicija { vrsta: vrsta, kolona: kolona }, 
            iskorisceno_na: Pozicija { vrsta: -1, kolona: -1 } }
    }
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
    let mut trenutno = Trenutna::new();
    trenutno.prolazi_kroz_lavirint(lavirint);
}