use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

type Child = Option<Box<Node>>;

//Binary Search Tree
struct Node {
    data: i32,
    left: Child,
    right: Child,
}
impl Node {
    fn new(new_data: i32) -> Self {
        Node {
            data: new_data,
            left: None,
            right: None,
        }
    }
}
struct BT {
    root: Child,
}

impl BT {
    fn new(data: i32) -> Self {
        BT {
            root: Some(Box::new(Node::new(data))),
        }
    }
    //Please don't stop doing this because of this doing

    //INSERT
    fn _insert(data: i32, cur_node: &mut Child) {
        if data < cur_node.as_ref().unwrap().data {
            if cur_node.as_ref().unwrap().left.is_none() {
                cur_node.as_mut().unwrap().left = Some(Box::new(Node::new(data)));
                return;
            } else {
                BT::_insert(data, &mut cur_node.as_mut().unwrap().left);
            }
        } else if data > cur_node.as_ref().unwrap().data {
            if cur_node.as_ref().unwrap().right.is_none() {
                cur_node.as_mut().unwrap().right = Some(Box::new(Node::new(data)));
                return;
            } else {
                BT::_insert(data, &mut cur_node.as_mut().unwrap().right);
            }
        } else {
            println!("Data in tree already");
        }
    }
    fn insert(&mut self, data: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(data)));
        } else {
            BT::_insert(data, &mut self.root);
        }
    }

    fn _print_tree(node: &mut Child) {
        if node.is_none() {
            return;
        }
        BT::_print_tree(&mut node.as_mut().unwrap().left);
        println!("{}", node.as_ref().unwrap().data);
        BT::_print_tree(&mut node.as_mut().unwrap().right);
    }

    fn print_tree(&mut self) {
        BT::_print_tree(&mut self.root)
    }

    fn _search(node: &Child, data: i32) -> Option<&Box<Node>> {
        let ret_node = node.as_ref().unwrap();
        match data.cmp(&ret_node.data) {
            Less => BT::_search(&ret_node.left, data),
            Greater => BT::_search(&ret_node.right, data),
            Equal => return node.as_ref(),
        }
    }

    fn search(&mut self, data: i32) {
        BT::_search(&mut self.root, data);
    }

    //REMOVE
    /*



    fn delete(&mut self, data: i32) {
        fn find_node_to_delete(node: &mut Node, data: i32) -> Option<&mut Node> {
            match data.cmp(&node.data) {
                Less => {
                    if let Some(ref mut left) = node.left {
                        find_node_to_delete(left, data)
                    } else {
                        None
                    }
                }
                Equal => Some(node),
                Greater => unimplemented!(),
            }
        }
        //find the node for deletion
        let node = find_node_to_delete(self, data);
        // once found, check if the node has children and delete accordingly

    }

    fn remove(&self, data: i32) {
        if self.root.is_none() {
            self._remove(data, self.root);
        }
    }
    fn _remove(&self, data: i32, cur_node: Node) -> Node {
        if cur_node.is_none() {
            return cur_node;
        }
        if data < cur_node.data {
            cur_node.left = self._remove(data, cur_node.left)
        } else if data > cur_node.data {
            cur_node.right = self._remove(data, cur_node.right);
        } else {
            if cur_node.left.is_none() && cur_node.right.is_none() {
                println!("Removing leaf node");
                if cur_node == self.root {
                    self.root = None;
                }
                //continue
            }
        }
    }
    */
}

//BST End

//Fibonacci
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

//99 Bottles of beer
fn nn_bottles(bt_n: i32) {
    if bt_n == 0 {
        println!(
            "No more bottles of beer on the wall, no more bottles of beer.\n
			Go to the store and buy some more, {} bottles of beer on the wall.",
            bt_n
        );
        return;
    }
    println!(
        "{} bottles of beer on the wall, {} bottles of beer",
        bt_n, bt_n
    );
    println!(
        "Take one down and pass it around, {} bottles of beer on the wall.\n",
        bt_n - 1
    );
    nn_bottles(bt_n - 1)
}

//C struct type
struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional_num: Option<i32>,
}

//tuple struct stype
struct TwoNums(i32, i32);

//Unit struct
struct Calculator;

//mplement methods on a struct
impl Data {
    //new function to create object with the default values assigned
    fn new() -> Self {
        Data {
            num1: 2,
            num2: 4,
            str1: "Some Swerth".to_string(),
            optional_num: None,
        }
    }
    fn sum(&self) -> i32 {
        self.num1 + self.num2
    }
}
//impl then name of the type
impl Calculator {
    fn add(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }
    fn sub(n1: i32, n2: i32) -> i32 {
        n1 - n2
    }
    fn mul(n1: i32, n2: i32) -> i32 {
        n1 * n2
    }
    fn div(n1: i32, n2: i32) -> f32 {
        (n1 / n2) as f32
    }
}

//make  a trait to add some functinonality to type data to make it something more useful
trait Transform {
    fn rev(&self) -> String;
    fn output_rev(&self) {
        println!("{}", self.rev());
    }
}

impl Transform for Data {
    fn rev(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}
fn main() {
    //println!("{}", x);

    //nn_bottles(99);
    let mut a = BT::new(4);

    a.insert(2);
    a.insert(9);
    a.print_tree();
    /*
        let a = Data{
            num1: 4,
            num2: 3,
            str1: "whatever".to_string(),
            optional_num: None
        };
        //println!("{}", a.sum());

        //create a new struct type but change only one variable
        let b  =  Data {num1:8,..a};
        //println!("{}", b.sum());
        //create an object with the new fn on the implement
        let mut c = Data::new();
        c.num1 = 3;
        //println!("{}", c.rev());
        c.output_rev();

        let d = TwoNums(4,3);
        //println!("{} {}",d.0,d.1);

        //Unit struct type object
        // println!("{}",Calculator::add(3,2));
        // println!("{}",Calculator::sub(3,2));
        // println!("{}",Calculator::mul(3,2));
        // println!("{}",Calculator::div(3,2));
        //let c: [&str:2] = ["engineer","man"];



        //Vectors and HashMaps

        let nums: Vec<i32> = Vec::new();
        let nums = vec![1,2,3];

        //inser values
        nums.push(4);
        nums.push(5);
        nums.push(6);

        // Access specific element(s)
        &nums[2];
        &nums[2..];
        nums.get(2);


        //remove elements
        nums.remove(2);

        //iterating over values

        for num in &nums {
            println!("{}",num);
        }
        //modifying vlues  in place
        for num in &nums {
            *num +=1;

        }
    //randomm types in vectors
        enum Value{
            Int(i32);
            Float(f32);
        };

        let random = vec![Value::Int(3), Value::Float(3.3)];

        //hash Maps (dont depend on the order)
        let mut numbers: HashMap <&str, i32>> = HashMap::new();
        //insert/update values

        numbers.insert("one",1);
        numbers.insert("two",2);
        numbers.insert("three",3);

        //access values

       // println!("{}", numbers.get("two").unwrap() );

        if numbers.contains_key("two"){
            //println!("{}", numbers.get("two").unwrap() );
        }

        match numbers.get("two"){
            Some(val) => println!("{}", val );
            None => println!("{Key does not exist")
        };

        //remove value
        numbers.remove("three");

        //iterate over values
        for(key, value) in &numbers {
            println!("{} => {}",key, value );
        }*/
}
