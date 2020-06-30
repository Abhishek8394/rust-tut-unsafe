//! Exact same name function overloading.
trait Pilot {
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human{
    fn fly(&self){
        println!("Buckle up folks!");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("Up! Up!");
    }
}

impl Human{
    fn fly(&self){
        println!("*waving arms*");
    }
}

trait Animal{
    fn baby_name() -> String;
}

struct Dog;
impl Dog{
    fn baby_name() -> String{
        String::from("spot")
    }
}

impl Animal for Dog{
    fn baby_name() -> String{
        String::from("puppy")
    }
}
pub fn overload_demo(){
    let human = Human;
    println!("Calling just human.fly()");
    human.fly();
    // Human::fly(&self) would work too
    println!("Calling pilot version: Pilot::fly()");
    Pilot::fly(&human);
    println!("Calling wizard version: Wizard::fly()");
    Wizard::fly(&human);
    println!("A baby of a dog is called {} (we expected puppy)", Dog::baby_name());
    // Below will error!
    // println!("A baby of a dog is called {} (we expected puppy)", Animal::baby_name());
    println!("A baby of a dog is called {} (we expected puppy)", <Dog as Animal>::baby_name());
}


