// Базовый типаж для всех компонентов
trait Component {
    fn operation(&self) -> String;
}

struct Leaf;

impl Component for Leaf {
    fn operation(&self) -> String {
        String::from("Leaf")
    }
}

struct Composite {
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn new() -> Self {
        Composite {
            children: Vec::new(),
        }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }
}

impl Component for Composite {
    fn operation(&self) -> String {
        let result_of_operation = self
            .children
            .iter()
            .map(|i| i.operation())
            .collect::<Vec<String>>()
            .join(", ");

        format!("Composite( {} )", result_of_operation)
    }
}

fn main() {
    let leaf1 = Box::new(Leaf);
    let leaf2 = Box::new(Leaf);

    let mut composite = Composite::new();
    composite.add(leaf1);
    composite.add(leaf2);

    println!("{}", composite.operation());
}
