use ndarray::prelude::*;
use ndarray::Array;
use plotpy::{Curve, Plot};
use std::cell::RefCell;
use std::io::{self};
use uuid::Uuid;

fn f(xs: Array1<f64>) -> Array1<f64> {
    xs.mapv(|x| 3.0 * x * x - 4.0 * x + 5.0)
}

/// The derivative of f() with respect to x.
#[allow(dead_code)]
fn f_prime(xs: Array1<f64>) -> Array1<f64> {
    xs.mapv(|x| 6.0 * x - 4.0)
}
// The plots generated in this code are available in the plots directory
// and also included in the README.md file.
//
// During development it was useful run the README.md using grip which
// watches the file, and provides a live preview of the README.md and makes
// it easy to inspect the plots without having them pop up which can be a little
// annoying otherwise.
fn main() -> io::Result<()> {
    // -----------------  intro ---------------------------
    println!("f(x) = 3.0 * x * x - 4.0 * x + 5.0");

    println!("\nLets tryout the function f:");
    println!("f(3.0) = {}", f(array![3.0])[0]);

    println!("\nLets generate some input data:");
    let xs = Array::range(-5., 5., 0.25);
    println!("xs = {xs:?}");

    println!("\nLets try invoking f(xs):");
    let ys = f(xs.clone());
    println!("ys = {ys:?}");

    plot(&xs, &ys, "part1_intro");

    // We can decrease this value, the nudge to be closer and closer to zero.
    println!("\nLets take a look at when the derivative:");
    let h = 0.00000001;
    let x = 3.0;
    println!("h = {h}");
    println!("x = {}", x);
    println!("f(x + h) =  {}", f(array![x + h])[0]);
    println!(
        "f(x + h) - f(x) / h = {}",
        (f(array![x + h])[0] - f(array![x])[0]) / h
    );
    // These values won't be exactly equal but the smaller h becomes the closer
    // they will be.
    println!("f_prime(x) =  {}", f_prime(array![x])[0]);

    println!("\nLets take a look at when the derivative is negative:");
    let x = -3.0;
    println!("x = {}", x);
    println!(
        "f(x + h) - f(x) / h = {}",
        (f(array![x + h])[0] - f(array![x])[0]) / h
    );

    // Show when the deriative is zero:
    println!("\nLets take a look at when the derivative is zero:");
    let x = 2.0 / 3.0;
    println!("x = {} (2/3)", x);
    println!(
        "f(x + h) - f(x) / h = {}",
        (f(array![x + h])[0] - f(array![x])[0]) / h
    );

    println!("\nNow lets take a look at a more complex example:");
    let a = 2.0;
    let b = -3.0;
    let c = 10.0;
    let d = a * b + c;
    println!("a = {a:.1}");
    println!("b = {b:.1}");
    println!("c = {c:.1}");
    println!("d = {d:.1}");

    let h = 0.0001;
    let mut a = 2.0;
    let b = -3.0;
    let c = 10.0;

    // d1 is our original function that we will use as a example
    let d1 = a * b + c;
    a += h;
    // d2 is the function with a nudged/dumped a little.
    let d2 = a * b + c;

    println!("\nDeriviative with respect to a:");
    println!("d1 (original function) = {d1:.6}");
    println!("d2 (nudged a         ) = {d2:.6}");
    println!("slope (d2 - d1) / h = {}", (d2 - d1) / h);

    let a = 2.0;
    let mut b = -3.0;
    let c = 10.0;
    let d1 = a * b + c;
    b += h;
    let d2 = a * b + c;
    println!("\nDeriviative with respect to b:");
    println!("d1 (original function) = {d1:.6}");
    println!("d2 (nudged b         ) = {d2:.6}");
    println!("slope (d2 - d1) / h = {}", (d2 - d1) / h);

    let a = 2.0;
    let b = -3.0;
    let mut c = 10.0;
    let d1 = a * b + c;
    c += h;
    let d2 = a * b + c;
    println!("\nDeriviative with respect to b:");
    println!("d1 (original function) = {d1:.6}");
    println!("d2 (nudged c         ) = {d2:.6}");
    println!("slope (d2 - d1) / h = {}", (d2 - d1) / h);

    // -----------------  micrograd overview ---------------------------

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Value<'a> {
        id: Uuid,
        data: RefCell<f64>,
        label: Option<String>,
        children: Vec<&'a Value<'a>>,
        operation: Option<Operation>,
        grad: RefCell<f64>,
    }

    impl<'a> PartialEq for Value<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    // Implement Eq for Value
    impl<'a> Eq for Value<'a> {}

    use std::hash::{Hash, Hasher};
    impl<'a> Hash for Value<'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    enum Operation {
        Add,
        Sub,
        Mul,
        Tanh,
        Exp,
    }

    // Implement as_str function for Operation enum
    impl Operation {
        fn as_str(&self) -> &'static str {
            match self {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mul => "*",
                Operation::Tanh => "tanh",
                Operation::Exp => "exp",
            }
        }
    }

    // Some of the comments below have been kept as they were prompts for
    // copilot to generated the code.

    // Add a new constructor for Value which takes a single f64.
    impl<'a> Value<'a> {
        fn new(data: f64) -> Self {
            Value {
                id: Uuid::new_v4(),
                data: RefCell::new(data),
                label: None,
                children: Vec::new(),
                operation: None,
                grad: RefCell::new(0.0), // we initialize the gradient to 0.0
            }
        }
        fn new_with_label(data: f64, label: &str) -> Self {
            Value {
                id: Uuid::new_v4(),
                data: RefCell::new(data),
                label: Some(label.to_string()),
                children: Vec::new(),
                operation: None,
                grad: RefCell::new(0.0), // we initialize the gradient to 0.0
            }
        }
    }

    impl<'a> Value<'a> {
        fn backward(&self) {
            match self.operation {
                Some(Operation::Add) => {
                    // Think of this as d = c + e
                    // Then &self if d and self.children is (c, e), and sinse
                    // addition passes through the gradient we can just add
                    // set the gradients of the children to the gradient of d.
                    let lhs = self.children[0];
                    let rhs = self.children[1];
                    // If we have have a + a then both lhs and rhs will be then
                    // same value so we accumulate the gradient.
                    *lhs.grad.borrow_mut() += 1.0 * *self.grad.borrow();
                    *rhs.grad.borrow_mut() += 1.0 * *self.grad.borrow();
                }
                Some(Operation::Sub) => {
                    let lhs = self.children[0];
                    let rhs = self.children[1];
                    *lhs.grad.borrow_mut() += 1.0 * *self.grad.borrow();
                    *rhs.grad.borrow_mut() += 1.0 * *self.grad.borrow();
                }
                Some(Operation::Mul) => {
                    let lhs = self.children[0];
                    let rhs = self.children[1];
                    *lhs.grad.borrow_mut() += *rhs.data.borrow() * *self.grad.borrow();
                    *rhs.grad.borrow_mut() += *lhs.data.borrow() * *self.grad.borrow();
                }
                Some(Operation::Tanh) => {
                    let lhs = self.children[0];
                    *lhs.grad.borrow_mut() +=
                        1.0 - self.data.borrow().powf(2.0) * *self.grad.borrow();
                }
                Some(Operation::Exp) => {
                    let lhs = self.children[0];
                    // e^x * dx/dx = e^x
                    *lhs.grad.borrow_mut() += *self.data.borrow() * *self.grad.borrow();
                }
                None => {
                    //println!("No backward for you! {}", self.label.as_ref().unwrap());
                }
            }
        }
    }

    use std::collections::VecDeque;
    impl<'a> Value<'a> {
        fn topological_sort(
            root: &'a Value<'a>,
            visited: &mut HashSet<&'a Value<'a>>,
            stack: &mut VecDeque<&'a Value<'a>>,
        ) {
            visited.insert(root);

            for child in root.children.iter() {
                if !visited.contains(child) {
                    Self::topological_sort(child, visited, stack);
                }
            }

            stack.push_front(root);
        }

        fn topological_order(value: &'a Value<'a>) -> VecDeque<&'a Value<'a>> {
            let mut visited = HashSet::new();
            let mut stack = VecDeque::new();
            Self::topological_sort(&value, &mut visited, &mut stack);
            stack
        }

        fn backwards(value: &'a Value<'a>) {
            *value.grad.borrow_mut() = 1.0;
            // Now lets do the backpropagation using the topological order.
            let order = Value::topological_order(&value);
            println!("topological order:");
            for (i, node) in order.iter().enumerate() {
                println!("{}: {:?}", i, node.label);
                node.backward();
            }
        }
    }

    // Add a new_with_children constructor for Value which takes a single f64, and a
    // parameter named 'children' of type Vec and that contains Values
    // as the element types.
    impl<'a> Value<'a> {
        fn new_with_children(
            data: f64,
            label: Option<String>,
            lhs: &'a Value<'a>,
            rhs: Option<&'a Value<'a>>,
            op: Operation,
        ) -> Self {
            let children = match rhs {
                Some(rhs) => vec![lhs, rhs],
                None => vec![lhs],
            };
            Value {
                id: Uuid::new_v4(),
                data: RefCell::new(data),
                label,
                children,
                operation: Some(op),
                grad: RefCell::new(0.0),
            }
        }
    }

    // Add Add trait implementation for Value and add use statement
    use std::ops::Add;
    impl<'a> Add<&'a Value<'a>> for &'a Value<'a> {
        type Output = Value<'a>;

        fn add(self, other: &'a Value<'a>) -> Self::Output {
            Value::new_with_children(
                *self.data.borrow() + *other.data.borrow(),
                None,
                self,
                Some(other),
                Operation::Add,
            )
        }
    }

    impl<'a> Add<&'a Value<'a>> for f64 {
        type Output = Value<'a>;

        fn add(self, other: &'a Value<'a>) -> Self::Output {
            Value::new(self + *other.data.borrow())
        }
    }

    impl<'a> Add<f64> for &'a Value<'a> {
        type Output = Value<'a>;

        fn add(self, other: f64) -> Self::Output {
            Value::new(*self.data.borrow() + other)
        }
    }

    // Add Sub trait implementation for Value and add use statement
    use std::ops::Sub;
    impl<'a, 'b: 'a> Sub<&'b Value<'b>> for &'a Value<'a> {
        type Output = Value<'a>;
        fn sub(self, other: &'b Value<'b>) -> Self::Output {
            Value::new_with_children(
                *self.data.borrow() - *other.data.borrow(),
                None,
                self,
                Some(other),
                Operation::Sub,
            )
        }
    }

    impl<'a> Sub<&'a Value<'a>> for f64 {
        type Output = Value<'a>;

        fn sub(self, other: &'a Value<'a>) -> Self::Output {
            Value::new(self - *other.data.borrow())
        }
    }

    impl<'a> Sub<f64> for &'a Value<'a> {
        type Output = Value<'a>;

        fn sub(self, other: f64) -> Self::Output {
            Value::new(*self.data.borrow() - other)
        }
    }

    // Add Mul trait implementation for Value and add use statement
    use std::ops::Mul;
    impl<'a, 'b: 'a> Mul<&'b Value<'b>> for &'a Value<'a> {
        type Output = Value<'a>;
        fn mul(self, other: &'b Value<'b>) -> Self::Output {
            Value::new_with_children(
                *self.data.borrow() * *other.data.borrow(),
                None,
                self,
                Some(other),
                Operation::Mul,
            )
        }
    }

    impl<'a> Mul<&'a Value<'a>> for f64 {
        type Output = Value<'a>;

        fn mul(self, other: &'a Value<'a>) -> Self::Output {
            Value::new(self * *other.data.borrow())
        }
    }

    impl<'a> Mul<f64> for &'a Value<'a> {
        type Output = Value<'a>;

        fn mul(self, other: f64) -> Self::Output {
            Value::new(*self.data.borrow() * other)
        }
    }

    use std::ops::Div;
    impl<'a> Div<&'a Value<'a>> for &'a Value<'a> {
        type Output = Value<'a>;

        fn div(self, other: &'a Value<'a>) -> Self::Output {
            Value::new_with_children(
                *self.data.borrow() / *other.data.borrow(),
                None,
                self,
                Some(other),
                Operation::Add,
            )
        }
    }

    impl<'a> Div<&'a Value<'a>> for f64 {
        type Output = Value<'a>;

        fn div(self, other: &'a Value<'a>) -> Self::Output {
            Value::new(self / *other.data.borrow())
        }
    }

    impl<'a> Div<f64> for &'a Value<'a> {
        type Output = Value<'a>;

        fn div(self, other: f64) -> Self::Output {
            Value::new(*self.data.borrow() / other)
        }
    }

    // Implement the Display trait for Value in the format Value(data) and
    // include any necessary use statements
    use std::fmt;
    impl fmt::Display for Value<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Value(data={}, label: {}",
                *self.data.borrow(),
                self.label.as_ref().unwrap_or(&"".to_string())
            )?;
            if &self.children.len() > &0 {
                write!(f, ", lhs={}", *self.children[0].data.borrow())?;
                if &self.children.len() == &2 {
                    write!(f, ", rhs={}", *self.children[1].data.borrow())?;
                }
                write!(f, ", op=\"{:?}\"", &self.operation.as_ref().unwrap())?;
            }
            write!(f, ", grad={})", self.grad.borrow())
        }
    }

    use std::collections::HashSet;
    use std::f64;
    #[allow(dead_code)]
    impl<'a> Value<'a> {
        /*
        fn children(&self) -> Vec<&'a Value<'a>> {
            self.children
        }
        */

        fn operation(&self) -> Option<Operation> {
            self.operation.clone()
        }

        fn label(&mut self, label: &str) {
            self.label = Some(label.to_string())
        }

        fn tanh(&self) -> Value {
            let x = *self.data.borrow();
            //
            // sinh(x) = (e^x - e^-x) / 2
            //
            // cosh(x) = (e^x + e^-x) / 2
            //
            //         sinh(x)    e^x - e^-x
            // tanh =  ------- = -----------
            //         cosh(x)    e^x + e^-x
            let t = (f64::exp(x) - f64::exp(-x)) / (f64::exp(x) + f64::exp(-x));
            println!("tanh({}) = {}", x, t);
            let t = (f64::exp(2.0 * x) - 1.0) / (f64::exp(2.0 * x) + 1.0);
            println!("tanh({}) = {}", x, t);
            Value::new_with_children(t, None, self, None, Operation::Tanh)
        }

        fn exp(&self) -> Value {
            let x = *self.data.borrow();
            let e = f64::exp(x);
            println!("exp({}) = {}", x, e);
            Value::new_with_children(e, None, self, None, Operation::Exp)
        }
    }

    impl Value<'_> {
        fn dot(&self) -> String {
            let mut out = "digraph {\n".to_string();
            out += "graph [rankdir=LR]\n";
            let mut stack = vec![self];
            let mut seen = HashSet::new();

            while let Some(node) = stack.pop() {
                let node_ptr = node as *const _;
                if seen.contains(&node_ptr) {
                    continue;
                }

                let node_id = node_ptr as usize;

                let label_str = |node: &Value| -> String {
                    match &node.label {
                        Some(l) => format!("{l}:"),
                        None => "".to_string(),
                    }
                };
                out += &format!(
                    "  \"{}\" [label=\"{} value: {:.4}, grad: {:.4}\" shape=record]\n",
                    node_ptr as usize,
                    label_str(node),
                    *node.data.borrow(),
                    node.grad.borrow(),
                );

                seen.insert(node_ptr);

                if !&node.children.is_empty() {
                    let op_id = format!("{}{}", node_id, node.operation.as_ref().unwrap().as_str());
                    let lhs_id = node.children[0] as *const _ as usize;

                    out += &format!(
                        "  \"{}\" [label=\"{}\"]\n",
                        op_id,
                        node.operation.as_ref().unwrap().as_str().to_string()
                    );
                    out += &format!("  \"{}\" -> \"{}\"\n", op_id, node_id,);

                    out += &format!("  \"{}\" -> \"{}\"\n", lhs_id, op_id,);
                    if &node.children.len() == &2 {
                        let rhs_id = node.children[1] as *const _ as usize;
                        out += &format!("  \"{}\" -> \"{}\"\n", rhs_id, op_id);
                        stack.push(&node.children[1]);
                    };

                    stack.push(&*node.children[0]);
                }
            }

            out += "}\n";
            out
        }
    }

    let a = Value::new_with_label(2.0, "a");
    println!("a = {}", a);
    let b = Value::new_with_label(-3.0, "b");
    println!("{a} + {b} = {}", &a + &b);
    println!("{a} - {b} = {}", &a - &b);
    println!("{a} * {b} = {}", &a * &b);
    let c = Value::new_with_label(10.0, "c");
    let mut e = &a * &b;
    e.label("e");
    let mut d = &e + &c;
    d.label("d");
    println!("{a} * {b} + {c} = {d}");
    println!("d: {d}");
    let f = Value::new_with_label(-2.0, "f");
    let mut l = &d * &f;
    l.label("l");

    // Manually calculate the derivative of the node graph
    {
        // This scope is just for manually computing the gradients which in the
        // Python example was a function named lol.
        let h = 0.0001;

        // First we calculate the gradient for l and save it in l1.
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l1 = l.data;

        // Now, lets compute the derivative of 'a' with respect to 'l'.
        let a = Value::new_with_label(2.0 + h, "a"); // Notice the +h here.
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data.borrow();
        let _da = (*l2 - *l1.borrow()) / h;
        //println!("\nDeriviative of l with respect to a: {_da:.6}");

        // Now, lets compute the derivative of 'l' with respect to 'l'.
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = *l.data.borrow() + h; // Notice the +h here.
        let dl = (l2 - *l1.borrow()) / h;
        println!("Deriviative of l with respect to l: {dl:.6}");

        //
        //  This is the operation that produces l:
        //  let mut l = &d * &f;
        //  And we want to compute the derivative of l with respect to d:
        //  dL/dd = ?
        //  We have:
        // ( f(x+h) - f(x) ) / h
        //  And we can plug in d for x:
        //  ( f(d+h) - f(d) ) /  h
        //  Expanding that will give us:
        //  (f*d + f*h - f*d) / h
        //   ↑           ↑
        //    +-----------+
        //  And 'f*d' will cancel out leaving us with:
        //  ( f*h ) / h = f
        //  So we can set dL/dd = f
        //  d.grad = f.data;
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0 + h, "f"); // Notice the +h here.
        let mut l = &d * &f;
        l.label("l");
        let l2 = *l.data.borrow();
        let df = (l2 - *l1.borrow()) / h;
        println!("Deriviative of l with respect to f: {df:.6}");

        // No lets compute the derivative of l with respect to f:
        //  dL/df = ?
        //  ( d(f+h) - d*f) ) /  h
        //  ((d*f + h*d - d*f) / h
        // ( h*d ) / h = d
        //  So we can set dL/dd = f
        //  d.grad = f.data;
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        *d.data.borrow_mut() += h; // Notice the +h here.
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data;
        let dd = (*l2.borrow() - *l1.borrow()) / h;
        println!("Deriviative of l with respect to d: {dd:.6}");

        // Now we want to compute the derivative of L with respect to c.
        // dd /dc = ?
        // We know that 'd' was created by adding 'c' to 'e'.
        // let mut d = &e + &c;
        // And we have:
        // (f(x+h) - f(x))/h
        // So we can plug in a nudge of c:
        // ((c+h) + e) - (c + e)/h
        // ((c+h) + e) - 1(c + e)/h
        // (c + h + e - c - e)/h
        // h/h = 1.0
        // So the derivative of dd/dc = 1.0, but we are interested in the
        // effect of c on l, so we need to multiply by the derivative of l with
        // respect to d which we calculated above:
        // dd/dc * dl/dd = 1.0 * f = f
        // Notice that since the derivative of addition is just 1.0, the
        // derivative of the latter part of the equation is just f. So these
        // derivates pass through the derivate from the node ahead of them in
        //  the chain.
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        *c.data.borrow_mut() += h; // Notice the +h here.
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data;
        let dc = (*l2.borrow() - *l1.borrow()) / h;
        println!("Deriviative of l with respect to c: {dc:.6}");

        // And the same thing applies for 'e' as for 'c':
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        *e.data.borrow_mut() += h; // Notice the +h here.
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data;
        let de = (*l2.borrow() - *l1.borrow()) / h;
        println!("Deriviative of l with respect to e: {de:.6}");

        // Next we want to compute dL/da. So we want to compute the derivative
        // of a with respect to L. Looking at a which is called a local node it
        // connection/link to L is through e which was created by multiplying
        // a and b.
        //
        // let mut a = 3.0
        // let a = Value::new(2.0, "a");
        // let b = Value::new(-3.0, "b");
        // And we have:
        // (f(x+h) - f(x))/h
        // So we can plug in a nudge of a:
        // ((a+h) * b) - (a * b)/h
        // ((a+h) * b) - (a * b)/h
        // (ab + hb - ab)/h
        // (hb)/h = b
        // dl/da = (dl/de) * (de/da) = -2.0 * -3.0 = 6.0
        let a = Value::new_with_label(2.0, "a");
        *a.data.borrow_mut() += h; // Notice the +h here.
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data;
        let da = (*l2.borrow() - *l1.borrow()) / h;
        println!("Deriviative of l with respect to a: {da:.6}");
        // And the same applies for b:
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        *b.data.borrow_mut() += h; // Notice the +h here.
        let c = Value::new_with_label(10.0, "c");
        let mut e = &a * &b;
        e.label("e");
        let mut d = &e + &c;
        d.label("d");
        let f = Value::new_with_label(-2.0, "f");
        let mut l = &d * &f;
        l.label("l");
        let l2 = l.data;
        let db = (*l2.borrow() - *l1.borrow()) / h;
        println!("Deriviative of l with respect to b: {db:.6}");
        // Notice that we started a the node at the end and computed the local
        // derivative for it and then moved back in the graph calculating the
        // local derivative for each node.
        // Think of each node as the result of an operation, for example l is
        // the result of d * f. So l has two children, d and f. So we can
    }
    // Set the gradients that were manually calculated above.
    *l.grad.borrow_mut() = 1.0;
    *f.grad.borrow_mut() = *d.data.borrow();
    *d.grad.borrow_mut() = *f.data.borrow();
    *c.grad.borrow_mut() = 1.0 * *f.data.borrow();
    *e.grad.borrow_mut() = 1.0 * *f.data.borrow();
    *a.grad.borrow_mut() = *e.grad.borrow() * *b.data.borrow();
    *b.grad.borrow_mut() = *e.grad.borrow() * *a.data.borrow();

    // Write the dot output to a file named "plots/part1_intrO.dot"
    std::fs::write("plots/part1_graph.dot", l.dot()).unwrap();
    // This file needs to be converted into an svg file to be rendered
    // and one option is to use the dot command line tool:
    // dot -Tsvg plots/part1_graph.dot -o plots/part1_graph.svg
    // Another options is to open the .dot file in https://viz-js.com.

    // Run the dot command to convert the .dot file to an svg file, and add
    // any required use statements
    run_dot("part1_graph");

    // The following simulates one set of an optimization that we would be
    // performed.
    // First we nudge the values of a, b, c, and f:
    *a.data.borrow_mut() += 0.001 * *a.grad.borrow();
    *b.data.borrow_mut() += 0.001 * *b.grad.borrow();
    *c.data.borrow_mut() += 0.001 * *c.grad.borrow();
    *f.data.borrow_mut() += 0.001 * *f.grad.borrow();
    // Then perform the forward pass again
    let e = &a * &b;
    let d = &e + &c;
    let l = &d * &f;
    // And then we inspect how those nudges affected the l:
    println!("l: {l:.6}");

    // Next section is "Manual backpropagation example #2: a neuron".

    // This will show a neural network with one single neuron (but without an
    // activation function for now) and two inputs:
    //  +----+
    //  | x₁ |\
    //  +----+ \ w₁
    //          \
    //           +-----------------------+
    //           |        n              |
    //           |  (x₁*w₁ + x₂*w₂) + b  | ----------->
    //           +-----------------------+
    //          /
    //  +----+ / w₂
    //  | x₂ |/
    //  +----+
    // Inputs
    let x1 = Value::new_with_label(2.0, "x1");
    let x2 = Value::new_with_label(0.0, "x2");
    println!("{x1}, {x2}");

    // Weights
    let w1 = Value::new_with_label(-3.0, "w1");
    let w2 = Value::new_with_label(1.0, "w2");
    println!("{w1}, {w2}");

    // Bias of the neuron.
    //let b = Value::new_with_label(6.7, "b");
    // This magic number is a value use to make the numbers come out nice.
    let b = Value::new_with_label(6.8813735870195432, "b");
    println!("{b}");

    // This is the edge to the 'x1w1' node
    let mut x1w1 = &x1 * &w1;
    x1w1.label("x1*w1");
    println!("{x1w1}");
    // This is the edge to the 'x2w2' node
    let mut x2w2 = &x2 * &w2;
    x2w2.label("x2*w2");
    println!("{x2w2}");

    let mut x1w1x2w2 = &x1w1 + &x2w2; // this is the sum part of the "dot" product.
    x1w1x2w2.label("x1w1x + 2w2");
    println!("x1w1x2w2: {x1w1x2w2}");

    // The following was not part of the youtube video, but is just me trying
    // to get an intuition for what is going on. Following the operations is
    // pretty easy but I feel that I loose a sense about what is actually
    // on and why we are performing these operations.
    //
    // We can try to visualize this neuron as performing the following:
    // We have our two inputs:
    //
    //                ^
    //             -6 |
    //                |
    //             -5 |
    //                |
    //             -4 -
    //                |
    //             -3 -
    //                |
    // x₁-axis     -2 -
    //                |
    //             -1 -
    //                |
    // x₂ =  0.0 ---> 0--|--|--|--|--|--|-->
    //                |  1  2  3  4  5  6
    //              1 -              x₂-axis
    //                |
    //              2 - <--- x₁ = 2.0
    //                |
    //              3 -
    //                |
    //                V
    //
    // And the edges to the neuron are scaling the inputs:
    // y = w₁x₁ + w₂x₂ + b
    // And we can plug in our values:
    // (-3.0 * 2.0) + (1.0 * 0.0)
    //         -6.0 + 0.0
    //
    // If we focus on the first two terms we can see that they are scaling
    // points on the x₁ and x₂ axis:
    //                ^
    //                |     ( w₁  * x₁)
    //             -6 |<---(-3.0 * 2.0)
    //                |
    //             -5 |
    //                |
    //             -4 -
    //                |
    //             -3 -
    //                |
    // x₁-axis     -2 -
    //                |
    //             -1 -
    //                |
    // (1.0 * 0)--->  0--|--|--|--|--|--|-->
    //                |  1  2  3  4  5  6
    //              1 -              x₂-axis
    //                |
    //              2 - <--- x₁ = 2.0
    //                |
    //              3 -
    //                |
    //                V
    //
    // In this case because x₂ is 0, the point will be (-6, 0). We then
    // add the bias which will give us the y value of the point. We can think
    // of y as coming out through the screen towards us reaching 0.7 units
    // outwards. That is it is a point above the (x₁, x₂) plane shown above.
    //
    // We then add the bias to the point (-6, 0) to get the final point:
    // y = -6.0 + 6.7 = 0.7
    // ( x₁,  x₂,   y)
    // (-6.0, 0.0, 0.7) which is a point in 3D space. The y value is the
    // height of the point. This is sometimes called the pre-activation value.
    // It is the y value, in this case 0.7 that will be passed to the activation
    // function which will transform it into the final output value of the
    // neuron.
    let mut n = &x1w1x2w2 + &b;
    n.label("n");
    println!("n pre_activation value: {n}");

    std::fs::write("plots/part1_single_neuron1.dot", n.dot()).unwrap();
    run_dot("part1_single_neuron1");

    // Print the tanh function for reference.
    let ys = xs.mapv(|x| f64::tanh(x));
    plot(&xs, &ys, "tanh");

    let mut o = n.tanh();
    o.label("o");

    std::fs::write("plots/part1_single_neuron2.dot", o.dot()).unwrap();
    run_dot("part1_single_neuron2");

    // Now lets perform the backpropagation.
    // do with regards to itself is just 1
    *o.grad.borrow_mut() = 1.0;
    // We need to calculate the local derivative of the tanh function.
    // This is the operation that this node performed:
    // o = tanh(n)
    // And the derivative of tanh is:
    // do/dn = 1 - tanh(n)^2
    // And we alreay have tanh(n) in o so we can just square it to get the
    // derivative:
    *n.grad.borrow_mut() = 1.0 - o.data.borrow().powf(2.0);
    // Next we have a plus/sum node which we know from before will just pass
    // the gradient through from the node ahead of it.
    *b.grad.borrow_mut() = *n.grad.borrow();
    *x1w1x2w2.grad.borrow_mut() = *n.grad.borrow();
    // The next nodes is also a sum node so we can just pass the gradient
    // through.
    *x1w1.grad.borrow_mut() = *x1w1x2w2.grad.borrow();
    *x2w2.grad.borrow_mut() = *x1w1x2w2.grad.borrow();
    // Next we have the multiplication nodes.
    *x1.grad.borrow_mut() = *w1.data.borrow() * x1w1.grad.borrow().clone();
    *w1.grad.borrow_mut() = *x1.data.borrow() * x1w1.grad.borrow().clone();
    *x2.grad.borrow_mut() = *w2.data.borrow() * x2w2.grad.borrow().clone();
    *w2.grad.borrow_mut() = *x2.data.borrow() * x2w2.grad.borrow().clone();

    println!("o: {o}");

    std::fs::write("plots/part1_single_neuron3.dot", o.dot()).unwrap();
    run_dot("part1_single_neuron3");

    // Now lets turn this manual backpropagation into functions that we can
    // call on our Value objects.

    // Inputs
    let x1 = Value::new_with_label(2.0, "x1");
    let x2 = Value::new_with_label(0.0, "x2");
    // Weights
    let w1 = Value::new_with_label(-3.0, "w1");
    let w2 = Value::new_with_label(1.0, "w2");
    // Bias of the neuron.
    //let b = Value::new_with_label(6.7, "b");
    // This magic number is a value use to make the numbers come out nice.
    let b = Value::new_with_label(6.8813735870195432, "b");
    // This is the edge to the 'x1w1' node
    let mut x1w1 = &x1 * &w1;
    x1w1.label("x1*w1");
    let mut x2w2 = &x2 * &w2;
    x2w2.label("x2*w2");
    let mut x1w1x2w2 = &x1w1 + &x2w2;
    x1w1x2w2.label("x1w1x + 2w2");
    let mut n = &x1w1x2w2 + &b;
    n.label("n");
    let mut o = n.tanh();
    o.label("o");
    println!("o: {o}");
    std::fs::write("plots/part1_single_neuron4.dot", o.dot()).unwrap();
    run_dot("part1_single_neuron4");
    *o.grad.borrow_mut() = 1.0;
    o.backward();
    n.backward();
    b.backward();
    x1w1x2w2.backward();
    x2w2.backward();
    x1w1.backward();
    std::fs::write("plots/part1_single_neuron5.dot", o.dot()).unwrap();
    run_dot("part1_single_neuron5");

    // Reset the values
    let x1 = Value::new_with_label(2.0, "x1");
    let x2 = Value::new_with_label(0.0, "x2");
    let w1 = Value::new_with_label(-3.0, "w1");
    let w2 = Value::new_with_label(1.0, "w2");
    let b = Value::new_with_label(6.8813735870195432, "b");
    let mut x1w1 = &x1 * &w1;
    x1w1.label("x1*w1");
    let mut x2w2 = &x2 * &w2;
    x2w2.label("x2*w2");
    let mut x1w1x2w2 = &x1w1 + &x2w2;
    x1w1x2w2.label("x1w1x + 2w2");
    let mut n = &x1w1x2w2 + &b;
    n.label("n");
    let mut o = n.tanh();
    o.label("o");
    Value::backwards(&o);
    std::fs::write("plots/part1_single_neuron6.dot", o.dot()).unwrap();
    run_dot("part1_single_neuron6");

    let a = Value::new_with_label(2.0, "a");
    let b = Value::new_with_label(4.0, "b");
    let c = &a / 2.0;
    println!("c: {c}");

    Ok(())
}

fn run_dot(file_name: &str) {
    use std::process::Command;
    Command::new("dot")
        .args(&[
            "-Tsvg",
            format!("plots/{}.dot", file_name).as_str(),
            "-o",
            format!("plots/{}.svg", file_name).as_str(),
        ])
        .output()
        .expect("failed to execute process");
}

fn plot(xs: &Array1<f64>, ys: &Array1<f64>, name: &str) {
    let mut curve = Curve::new();

    curve.draw(&xs.to_vec(), &ys.to_vec());

    let mut plot = Plot::new();
    plot.set_subplot(2, 2, 1)
        .set_horizontal_gap(0.1)
        .set_vertical_gap(0.2)
        .set_gaps(0.3, 0.4)
        .set_equal_axes(true)
        .set_hide_axes(false)
        .set_range(-1.0, 1.0, -1.0, 1.0)
        .set_range_from_vec(&[0.0, 1.0, 0.0, 1.0])
        .set_xmin(0.0)
        .set_xmax(1.0)
        .set_ymin(0.0)
        .set_ymax(1.0)
        .set_xrange(0.0, 1.0)
        .set_yrange(0.0, 1.0)
        .set_num_ticks_x(0)
        .set_num_ticks_x(8)
        .set_num_ticks_y(0)
        .set_num_ticks_y(5)
        .set_label_x("x-label")
        .set_label_y("y-label")
        .set_labels("x", "y")
        .clear_current_axes();
    plot.clear_current_figure();
    plot.set_title(name)
        .set_frame_borders(false)
        .set_frame_borders(true)
        .set_frame_borders(false);
    plot.grid_and_labels("x", "y");
    plot.add(&curve);
    //let _ = plot.save_and_show(&format!("./plots/{name}.svg")).unwrap();
    let _ = plot.save(&format!("./plots/{name}.svg")).unwrap();
}
