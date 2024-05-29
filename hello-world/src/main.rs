#[derive(Clone, Copy, Debug)]

struct Point{
    x: f64,
    y: f64,
}

fn distance(p1:Point, p2:Point) -> f64{
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx*dx + dy*dy).sqrt()
}

fn translate(p:&mut Point, dx:f64, dy:f64){
    (*p).x += dx;
    (*p).y += dy;
}

fn main() {
    //Déclarer une variable
    let i = 19i16;
    let b:bool = true;
    let c:char = 'c';
    let f:f64 = 3.14;
    let p :&i16 = &i;


    let s = "Hello, world!";
    //Déclarer un buffered string -> Variable placé en mémoire dynamique
    let bs:String = String::from("World!");

    let a:[i32; 3] = [1, 2, 3];
    let mut vec:Vec<i32> = Vec::new(); // :: pour les chevons de généricité
    vec.push(1);


    let t = (1, 3.14f32, 'c');

    let p1 = Point{x: 0.0, y: 0.0};
    let mut p2 = p1;

    // Quand on a un point d'exclamation, ceci est une macro et non une fonction
    // Ajouter {:?} pour afficher le contenu d'une variable tuple ou tableau
    println!("Hello, world!\nLes coordonnées de notre point sont : x={} et y={}\nLes coordonnées de notre deuxième point sont : x={} et y={}",p1.x,p1.y,p2.x,p2.y);
    translate(&mut p2, 3.0, 4.0);
    println!("La distance est : {}",distance(p1,p2));
    println!("La distance est : {:#?}",p1);
    println!("-----------");
    
}
