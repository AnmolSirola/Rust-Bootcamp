
trait Park{    //1 
    fn park(&self); 
}

trait Paint{  //3 
    fn paint(&self, color: String) {
        println!("painting object: {}", color)
    }
}

struct VehicleInfo { //2
    make: String,
    model: String,
    year: u6,
}

struct Car { // from 2
    info: VehicleInfo;
}

/*
struct Car {
    make: String,
    model: String,
    year: u6,
}
 */

impl Park for Car {  // from 1
    fn park(&self){
        println!("Aye Parking hai kya bhai pe")
    }
}

impl Paint for Car{} //from 3


/* 
stuct Truck {
    make: String,
    model: String,
    year: u6,
}
*/

struct Truck { // from 2
    info: VehicleInfo;
}
 
impl Truck {     
    fn unload(&self){
        println!("Unloading Truck.")
    }
}



impl Park for Truck { // from 1
    fn park(&self){
        println!("Aye Truck hai kya bhai pe")
    }
}