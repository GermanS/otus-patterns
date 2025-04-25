trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        String::from("ConcreteComponent")
    }
}

struct Decorator {
    component: Box<dyn Component>,
}

impl Decorator {
    fn new(component: Box<dyn Component>) -> Self {
        Decorator { component }
    }
}

impl Component for Decorator {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

struct ConcreteDecoratorA {
    decorator: Decorator,
}

impl ConcreteDecoratorA {
    fn new(component: Box<dyn Component>) -> Self {
        ConcreteDecoratorA {
            decorator: Decorator::new(component),
        }
    }
}

impl Component for ConcreteDecoratorA {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorA({})", self.decorator.operation())
    }
}

struct ConcreteDecoratorB {
    decorator: Decorator,
}

impl ConcreteDecoratorB {
    fn new(component: Box<dyn Component>) -> Self {
        ConcreteDecoratorB {
            decorator: Decorator::new(component),
        }
    }
}

impl Component for ConcreteDecoratorB {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorB({})", self.decorator.operation())
    }
}

fn main() {
    let simple = Box::new(ConcreteComponent);
    println!("Simple component: {}", simple.operation());

    let decorated_a = Box::new(ConcreteDecoratorA::new(simple));
    println!("Decorated A: {}", decorated_a.operation());

    let decorated_b = Box::new(ConcreteDecoratorB::new(decorated_a));
    println!("Decorated B: {}", decorated_b.operation());
}
