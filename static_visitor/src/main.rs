trait Element {
    fn accept<V: Visitor>(&self, visitor: &V) -> String;
}

struct ConcreteElementA;

impl Element for ConcreteElementA {
    fn accept<V: Visitor>(&self, visitor: &V) -> String {
        visitor.visit_concrete_element_a(self)
    }
}

struct ConcreteElementB;

impl Element for ConcreteElementB {
    fn accept<V: Visitor>(&self, visitor: &V) -> String {
        visitor.visit_concrete_element_b(self)
    }
}

trait Visitor {
    fn visit_concrete_element_a(&self, element: &ConcreteElementA) -> String;
    fn visit_concrete_element_b(&self, element: &ConcreteElementB) -> String;
}

struct ConcreteVisitor1;

impl Visitor for ConcreteVisitor1 {
    fn visit_concrete_element_a(&self, _element: &ConcreteElementA) -> String {
        String::from("ConcreteVisitor1 visits ConcreteElementA")
    }

    fn visit_concrete_element_b(&self, _element: &ConcreteElementB) -> String {
        String::from("ConcreteVisitor1 visits ConcreteElementB")
    }
}

struct ConcreteVisitor2;

impl Visitor for ConcreteVisitor2 {
    fn visit_concrete_element_a(&self, _element: &ConcreteElementA) -> String {
        String::from("ConcreteVisitor2 visits ConcreteElementA")
    }

    fn visit_concrete_element_b(&self, _element: &ConcreteElementB) -> String {
        String::from("ConcreteVisitor2 visits ConcreteElementB")
    }
}

fn main() {
    let element_a = ConcreteElementA;
    let element_b = ConcreteElementB;

    let visitor1 = ConcreteVisitor1;
    let visitor2 = ConcreteVisitor2;

    println!("{}", element_a.accept(&visitor1));
    println!("{}", element_b.accept(&visitor1));

    println!("{}", element_a.accept(&visitor2));
    println!("{}", element_b.accept(&visitor2));
}
