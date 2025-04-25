// Базовый типаж для всех компонентов
trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        String::from("ConcreteComponent")
    }
}

struct Decorator<T: Component> {
    component: T,
}

impl<T: Component> Decorator<T> {
    fn new(component: T) -> Self {
        Decorator { component }
    }
}

impl<T: Component> Component for Decorator<T> {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

struct ConcreteDecoratorA<T: Component> {
    decorator: Decorator<T>,
}

impl<T: Component> ConcreteDecoratorA<T> {
    fn new(component: T) -> Self {
        ConcreteDecoratorA {
            decorator: Decorator::new(component),
        }
    }
}

impl<T: Component> Component for ConcreteDecoratorA<T> {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorA({})", self.decorator.operation())
    }
}

struct ConcreteDecoratorB<T: Component> {
    decorator: Decorator<T>,
}

impl<T: Component> ConcreteDecoratorB<T> {
    fn new(component: T) -> Self {
        ConcreteDecoratorB {
            decorator: Decorator::new(component),
        }
    }
}

impl<T: Component> Component for ConcreteDecoratorB<T> {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorB({})", self.decorator.operation())
    }
}

fn main() {
    let simple = ConcreteComponent;
    println!("Simple component: {}", simple.operation());

    let decorated_a = ConcreteDecoratorA::new(simple);
    println!("Decorated A: {}", decorated_a.operation());

    let decorated_b = ConcreteDecoratorB::new(decorated_a);
    println!("Decorated B: {}", decorated_b.operation());
}
