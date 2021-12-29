use std::cmp;
use std::io::{stdin};

fn main()
{
    let mut operations = vec![];
    let lines: Vec<&str> = raw_input().trim().lines().map(|line| line.trim()).collect();

    for line in &lines {
        let (operation_string, params) = line.split_once(' ').unwrap();

        let operation = match operation_string {
            "inp" => Ops::Inp(parse_variable(params)),
            _ => {
                let (variable_string, var_or_val_string) = params.split_once(' ').unwrap();
                let variable = parse_variable(variable_string);
                let var_or_val = parse_var_or_val(var_or_val_string);

                match operation_string {
                    "add" => Ops::Maths(MathsKind::Add, variable, var_or_val),
                    "mul" => Ops::Maths(MathsKind::Mul, variable, var_or_val),
                    "div" => Ops::Maths(MathsKind::Div, variable, var_or_val),
                    "mod" => Ops::Maths(MathsKind::Mod, variable, var_or_val),
                    "eql" => Ops::Maths(MathsKind::Eql, variable, var_or_val),
                    _ => panic!(),
                }
            },
        };

        operations.push(operation);
    }

    let mut alu = Alu {
        w: get_var_content_from_value(0),
        x: get_var_content_from_value(0),
        y: get_var_content_from_value(0),
        z: get_var_content_from_value(0),

        next_input_index: 0,
        next_var_content_id: 40, // IDs from -26 to 26 for values inside ops / IDs from 30 to 39 for inputs
    };

    let operations_length = operations.len();

    for (index, operation) in operations.into_iter().enumerate() {
        println!("Operation {}/{}", index + 1, operations_length);
        println!("{}", lines[index]);

        alu.apply_ops(operation);

        println!();
        println!("\t------ W ------");
        print(&alu.w, 1, 5, true);
        println!();
        println!();
        println!("\t------ X ------");
        print(&alu.x, 1, 5, true);
        println!();
        println!();
        println!("\t------ Y ------");
        print(&alu.y, 1, 5, true);
        println!();
        println!();
        println!("\t------ Z ------");
        print(&alu.z, 1, 5, true);
        println!();
        println!();

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
    }
}

fn get_var_content_from_value(value: i64) -> VarContent
{
    VarContent { content: Content::Value(value), source: Source::Value(value) }
}

fn get_var_content_from_input(index: usize) -> VarContent
{
    VarContent { content: Content::Options(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), source: Source::Inp(index) }
}

#[derive(Debug)]
struct Alu {
    w: VarContent,
    x: VarContent,
    y: VarContent,
    z: VarContent,

    next_input_index: usize,
    next_var_content_id: i64,
}

impl Alu {
    fn apply_ops(&mut self, ops: Ops)
    {
        let into_variable = match ops {
            Ops::Inp(variable) => variable,
            Ops::Maths(_, variable, _) => variable,
        };

        let value: VarContent = match ops {
            Ops::Inp(_) => {
                let next_input_index = self.next_input_index;
                self.next_input_index += 1;
                get_var_content_from_input(next_input_index)
            },
            Ops::Maths(kind, left_var, right_var_or_val) => {
                let left  = self.get_var_content(left_var);
                let right = match right_var_or_val {
                    VarOrVal::Val(value) => get_var_content_from_value(value),
                    VarOrVal::Var(variable) => self.get_var_content(variable),
                };

                match (&left, &right) {
                    (VarContent { content : Content::Value(left_val), .. }, VarContent { content : Content::Value(right_val), .. }) => {
                        get_var_content_from_value(kind.execute(*left_val, *right_val))
                    },

                    // Comparaison
                    (_, _) if kind == MathsKind::Eql && (left.content.max() < right.content.min() || left.content.min() > right.content.max()) => {
                        get_var_content_from_value(0)
                    },

                    // Multiplication
                    (VarContent { content : Content::Value(0), .. }, _) if kind == MathsKind::Mul => {
                        get_var_content_from_value(0)
                    },
                    (_, VarContent { content : Content::Value(0), .. }) if kind == MathsKind::Mul => {
                        get_var_content_from_value(0)
                    },
                    (VarContent { content : Content::Value(1), .. }, _) if kind == MathsKind::Mul => {
                        right
                    },
                    (_, VarContent { content : Content::Value(1), .. }) if kind == MathsKind::Mul => {
                        left
                    },

                    // Addition
                    (VarContent { content : Content::Value(0), .. }, _) if kind == MathsKind::Add => {
                        right
                    },
                    (_, VarContent { content : Content::Value(0), .. }) if kind == MathsKind::Add => {
                        left
                    },

                    // Division
                    (VarContent { content : Content::Value(0), .. }, _) if kind == MathsKind::Div => {
                        get_var_content_from_value(0)
                    },
                    (_, VarContent { content : Content::Value(1), .. }) if kind == MathsKind::Div => {
                        left
                    },
                    (_, VarContent { content : Content::Value(div_value), .. }) if kind == MathsKind::Div && left.content.min() >= 0 && left.content.max() < *div_value => {
                        get_var_content_from_value(0)
                    },

                    // Mod
                    (_, VarContent { content : Content::Value(1), .. }) if kind == MathsKind::Mod => {
                        get_var_content_from_value(0)
                    },
                    (_, VarContent { content : Content::Value(mod_value), .. }) if kind == MathsKind::Mod && left.content.max() <= *mod_value => { // Selon l'énoncé : mod_value > 0 et left >= 0
                        left
                    },

                    // Simplifications de deux additions à la suite (x+2)+3 = x+5
                    (
                        VarContent { source: Source::Maths(MathsKind::Add, additions ), content: old_content },
                        VarContent { content : Content::Value(new_value), .. },
                    ) if kind == MathsKind::Add => {
                        let mut found_value = false;

                        for &mut addition in additions.iter_mut() {
                            if let VarContent { content: Content::Value(value), .. } = addition {
                                found_value = true;
                                addition = get_var_content_from_value(value + new_value);
                            }
                        }

                        if ! found_value {
                            additions.push(get_var_content_from_value(*new_value));
                        }

                        VarContent {
                            source: Source::Maths(MathsKind::Add, *additions ),
                            content: merge_content(MathsKind::Add, old_content, &Content::Value(*new_value))
                        }
                    },

                    // Simplifications d'une addition de deux additions (x+2)+(y+4) = (x+y)+6
                    // (
                    //     VarContent { source: Source::Maths(MathsKind::Add, inner_left_left, inner_left_right ), .. },
                    //     VarContent { source: Source::Maths(MathsKind::Add, inner_right_left, inner_right_right ), .. },
                    // ) if kind == MathsKind::Add => {
                    //     if let VarContent { content: Content::Value(inner_left_right_value), .. } = **inner_left_right {
                    //         if let VarContent { content: Content::Value(inner_right_right_value), .. } = **inner_right_right {
                    //             let new_left  = merge_var_contents(MathsKind::Add, *inner_left_left.clone(), *inner_right_left.clone());

                    //             merge_var_contents(kind, new_left, get_var_content_from_value(inner_left_right_value + inner_right_right_value))
                    //         } else {
                    //             merge_var_contents(kind, left, right)
                    //         }
                    //     } else {
                    //         merge_var_contents(kind, left, right)
                    //     }
                    // },

                    // // Simplification d'un multiplication d'une addition par un chiffre (x+2)*3 = (x*3)+6
                    // (
                    //     VarContent { source: Source::Maths(MathsKind::Add, inner_left, inner_right ), .. },
                    //     VarContent { content : Content::Value(right_value), .. },
                    // ) if kind == MathsKind::Mul => {
                    //     if let VarContent { content: Content::Value(previous_add), .. } = **inner_right {
                    //         let new_left  = merge_var_contents(MathsKind::Mul, *inner_left.clone(), get_var_content_from_value(*right_value));
                    //         let new_right = get_var_content_from_value(previous_add * right_value);

                    //         merge_var_contents(MathsKind::Add, new_left, new_right)
                    //     } else {
                    //         merge_var_contents(kind, left, right)
                    //     }
                    // },

                    _ => merge_var_contents(kind, left, right),
                }
            },
        };

        match into_variable {
            Var::W => self.w = value,
            Var::X => self.x = value,
            Var::Y => self.y = value,
            Var::Z => self.z = value, 
        }
    }

    fn get_var_content(&self, variable: Var) -> VarContent
    {
        match variable {
            Var::W => self.w.clone(),
            Var::X => self.x.clone(),
            Var::Y => self.y.clone(),
            Var::Z => self.z.clone(),
        }
    }
}

fn merge_var_contents(kind: MathsKind, left: VarContent, right: VarContent) -> VarContent
{
    let content = merge_content(kind, &left.content, &right.content);
    VarContent { content, source: Source::Maths(kind, vec![left, right]) }
}


#[derive(Debug, Clone, PartialEq)]
struct VarContent {
    content: Content,
    source: Source,
}

#[derive(Debug, Clone, PartialEq)]
enum Content {
    Value(i64),
    Range(i64, i64),
    Options(Vec<i64>),
}

impl Content {
    fn min(&self) -> i64
    {
        match self {
            Content::Value(value) => *value,
            Content::Range(min, _) => *min,
            Content::Options(options) => *options.iter().min().unwrap(),
        }
    }

    fn max(&self) -> i64
    {
        match self {
            Content::Value(value) => *value,
            Content::Range(_, max) => *max,
            Content::Options(options) => *options.iter().max().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Source {
    Value(i64),
    Inp(usize),
    Maths(MathsKind, Vec<VarContent>),
}


// use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Var {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VarOrVal {
    Val(i64),
    Var(Var),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ops {
    Inp(Var),
    Maths(MathsKind, Var, VarOrVal),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MathsKind {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl MathsKind {
    fn execute(&self, a: i64, b: i64) -> i64
    {
        match self {
            MathsKind::Add => a + b,
            MathsKind::Mul => a * b,
            MathsKind::Div => a / b,
            MathsKind::Mod => a % b,
            MathsKind::Eql => if a == b { 1 } else { 0 },
        }
    }
}

fn merge_content(kind: MathsKind, left_content: &Content, right_content: &Content) -> Content
{
    match kind {

        MathsKind::Mod => Content::Range(0, right_content.max() - 1),
        MathsKind::Eql => Content::Options(vec![0, 1]),
        kind => {
            match (left_content, right_content) {
                (Content::Value(_), Content::Value(_)) => panic!("Déjà géré plus haut car on peut simplement exécuter la fonction"),

                (Content::Value(left), Content::Options(options)) => Content::Options(options.into_iter().map(|right| kind.execute(*left, *right)).collect()),
                (Content::Options(options), Content::Value(right)) => Content::Options(options.into_iter().map(|left| kind.execute(*left, *right)).collect()),

                (Content::Value(left), Content::Range(right_min, right_max)) => Content::Range(
                    cmp::min(kind.execute(*left, *right_min), kind.execute(*left, *right_max)),
                    cmp::max(kind.execute(*left, *right_min), kind.execute(*left, *right_max)),
                ),
                (Content::Range(left_min, left_max), Content::Value(right)) => Content::Range(
                    cmp::min(kind.execute(*left_min, *right), kind.execute(*left_max, *right)),
                    cmp::max(kind.execute(*left_min, *right), kind.execute(*left_max, *right)),
                ),
                
                (Content::Range(left_min, left_max), right_content)  => compute_new_range(kind, *left_min, *left_max, right_content.min(), right_content.max()),
                (left_content, Content::Range(right_min, right_max)) => compute_new_range(kind, left_content.min(), left_content.max(), *right_min, *right_max),

                (Content::Options(left_options), Content::Options(right_options)) => {
                    let mut new_options = Vec::with_capacity(left_options.len() * right_options.len());

                    for left_option in left_options {
                        for right_option in right_options {
                            new_options.push(kind.execute(*left_option, *right_option));
                        }
                    }

                    Content::Options(new_options)
                }
            }
        }
    }
}

fn compute_new_range(kind: MathsKind, left_min: i64, left_max: i64, right_min: i64, right_max: i64) -> Content
{
    let bounds = [
        kind.execute(left_min, right_min),
        kind.execute(left_min, right_max),
        kind.execute(left_max, right_min),
        kind.execute(left_max, right_max)
    ];

    Content::Range(*bounds.iter().min().unwrap(), *bounds.iter().max().unwrap())
}

// enum MinMax {
//     Value(i64),
//     Inp(u32),
// }

// #[derive(Debug, Clone, PartialEq)]
// struct VarValue {
//     id: i64,
//     value: VarValueEnum,
//     min: i64,
//     max: i64,
//     depends_on_inputs: [bool; 14],
//     values_possible: Option<Vec<i64>>,
// }

// #[derive(Debug, Clone, PartialEq)]
// enum VarValueEnum {
//     Value(i64),
//     Inp(usize),
//     Maths(MathsType, Box<VarValue>, Box<VarValue>),
// }

// #[derive(Debug)]
// struct Alu {
//     w: VarValue,
//     x: VarValue,
//     y: VarValue,
//     z: VarValue,
// }

// fn main()
// {
//     let mut operations = vec![];
//     for line in raw_input().trim().lines() {
//         let (operation_string, params) = line.trim().split_once(' ').unwrap();

//         let operation = match operation_string {
//             "inp" => Ops::Inp(parse_variable(params)),
//             _ => {
//                 let (variable_string, var_or_val_string) = params.split_once(' ').unwrap();
//                 let variable = parse_variable(variable_string);
//                 let var_or_val = parse_var_or_val(var_or_val_string);

//                 match operation_string {
//                     "add" => Ops::Maths(MathsType::Add, variable, var_or_val),
//                     "mul" => Ops::Maths(MathsType::Mul, variable, var_or_val),
//                     "div" => Ops::Maths(MathsType::Div, variable, var_or_val),
//                     "mod" => Ops::Maths(MathsType::Mod, variable, var_or_val),
//                     "eql" => Ops::Maths(MathsType::Eql, variable, var_or_val),
//                     _ => panic!(),
//                 }
//             },
//         };

//         operations.push(operation);
//     }

//     let mut next_inp_index = 0;

//     let mut alu = Alu {
//         w: get_zero(),
//         x: get_zero(),
//         y: get_zero(),
//         z: get_zero(),
//     };

//     let mut next_var_value_id = 40; // vaues max value -26 to 26 / inp 30 to 39

//     let operations_length = operations.len();

//     for (index, operation) in operations.into_iter().enumerate() {
//         println!("Operation {}/{}", index + 1, operations_length);
//         // dbg!(&operation);
//         let (variable, value) = match operation {
//             Ops::Inp(var) => {
//                 let mut depends = [false; 14];
//                 depends[next_inp_index] = true;
//                 let result = (var, VarValue { id: 30 + next_inp_index as i64, value: VarValueEnum::Inp(next_inp_index), min: 1, max: 9, depends_on_inputs: depends, values_possible: Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]) });
//                 next_inp_index += 1;
//                 result
//             },
//             Ops::Maths(maths_type, left_var, right_var) => {
//                 let left = get_value(&alu, left_var);
//                 let right = match right_var {
//                     VarOrVal::Val(value) => VarValue { id: value, value: VarValueEnum::Value(value), min: value, max: value, depends_on_inputs: [false; 14], values_possible: Some(vec![value]) },
//                     VarOrVal::Var(variable) => get_value(&alu, variable),
//                 };

//                 let new_value = VarValueEnum::Maths(maths_type, Box::new(left.clone()), Box::new(right.clone()));
//                 let mut new_depends = [false; 14];
//                 for i in 0..14 {
//                     new_depends[i] = left.depends_on_inputs[i] || right.depends_on_inputs[i];
//                 }

//                 match (&left.value, &right.value) {
//                     (VarValueEnum::Value(left_value), VarValueEnum::Value(right_value)) => {
//                         let result = match maths_type {
//                             MathsType::Add => left_value + right_value,
//                             MathsType::Mul => left_value * right_value,
//                             MathsType::Div => left_value / right_value,
//                             MathsType::Mod => left_value % right_value,
//                             MathsType::Eql => if left_value == right_value { 1 } else { 0 },
//                         };

//                         let result = (left_var, VarValue { id: next_var_value_id, value: VarValueEnum::Value(result), min: result, max: result, depends_on_inputs: [false; 14], values_possible: Some(vec![result]) });
//                         next_var_value_id += 1;

//                         result
//                     },
//                     _ => {

//                         let (simple_value, min_max): (Option<VarValue>, Option<(i64, i64)>) = match maths_type {
//                             MathsType::Mul => {
//                                 let new_min = [left.min * right.min, left.min * right.max, left.max * right.min, left.max * right.max].into_iter().min().unwrap();
//                                 let new_max = [left.min * right.min, left.min * right.max, left.max * right.min, left.max * right.max].into_iter().max().unwrap();

//                                 if right.value == VarValueEnum::Value(0) {
//                                     (Some(get_zero()), None)
//                                 } else if right.value == VarValueEnum::Value(1) {
//                                     (Some(left), None)
//                                 } else if left.value == VarValueEnum::Value(0) {
//                                     (Some(get_zero()), None)
//                                 } else if left.value == VarValueEnum::Value(1) {
//                                     (Some(right), None)
//                                 } else {
//                                     (None, Some((new_min, new_max)))
//                                 }
//                             },
//                             MathsType::Add => {
//                                 let new_min = left.min + right.min;
//                                 let new_max = left.max + right.max;

//                                 if right.value == VarValueEnum::Value(0) {
//                                     (Some(left), Some((new_min, new_max)))
//                                 } else if left.value == VarValueEnum::Value(0) {
//                                     (Some(right), Some((new_min, new_max)))
//                                 } else {
//                                     (None, Some((new_min, new_max)))
//                                 }
//                             },
//                             MathsType::Div => {
//                                 let new_min = [left.min / right.min, left.min / right.max, left.max / right.min, left.max / right.max].into_iter().min().unwrap();
//                                 let new_max = [left.min / right.min, left.min / right.max, left.max / right.min, left.max / right.max].into_iter().max().unwrap();

//                                 if right.value == VarValueEnum::Value(1) {
//                                     (Some(left), None)
//                                 } else if left.value == VarValueEnum::Value(0) {
//                                     (Some(get_zero()), None)
//                                 } else if let VarValueEnum::Value(divider) = right.value {
//                                     if left.min >= 0 && left.max < divider {
//                                         (Some(get_zero()), None)
//                                     // } else if let VarValueEnum::Maths(MathsType::Add, add_left, add_right) = left.value {
//                                     //     (None, Some((new_min, new_max)))
//                                     } else {
//                                         (None, Some((new_min, new_max)))
//                                     }
//                                 } else {
//                                     (None, Some((new_min, new_max)))
//                                 }
//                             },
//                             MathsType::Mod => {
//                                 let new_min = 0;
//                                 let new_max = right.max - 1;

//                                 if right.value == VarValueEnum::Value(1) || new_min == new_max {
//                                     (Some(get_zero()), None)
//                                 } else if let VarValueEnum::Value(mod_value) = right.value {
//                                     if left.max < mod_value {
//                                         (Some(left), None)
//                                     } else if let VarValueEnum::Maths(MathsType::Add, add_left, add_right) = left.value {
//                                         if is_multiplication_by(&*add_left, mod_value).is_some() {
//                                             (Some(*add_right), None)
//                                         } else if is_multiplication_by(&*add_right, mod_value).is_some() {
//                                             (Some(*add_left), None)
//                                         } else {
//                                             (None, Some((new_min, new_max)))
//                                         }
//                                     } else {
//                                         (None, Some((new_min, new_max)))
//                                     }
//                                 } else {
//                                     (None, Some((new_min, new_max)))
//                                 }
//                             },
//                             MathsType::Eql => {
//                                 if left.max < right.min || left.min > right.max {
//                                     (Some(get_zero()), None)
//                                 } else {
//                                     (None, Some((0, 1)))
//                                 }
//                             },
//                         };

//                         let min_max = min_max.unwrap_or((i64::MIN, i64::MAX));
        
//                         let result = (
//                             left_var,
//                             simple_value.unwrap_or(VarValue { id: next_var_value_id, value: new_value, min: min_max.0, max: min_max.1, depends_on_inputs: new_depends, values_possible: None }),
//                         );
//                         next_var_value_id += 1;

//                         result
//                     }
//                 }

//             },
//         };

//         match variable {
//             Var::W => alu.w = value.clone(),
//             Var::X => alu.x = value.clone(),
//             Var::Y => alu.y = value.clone(),
//             Var::Z => alu.z = value.clone(),
//         }

//         let end = 110;
//         if index == end {
//             break;
//         }
//         if index >= end - 10 {
//             println!();
//             println!("\t------ W ------");
//             print(&alu.w, 1, 5, true);
//             println!();
//             println!();
//             println!("\t------ X ------");
//             print(&alu.x, 1, 5, true);
//             println!();
//             println!();
//             println!("\t------ Y ------");
//             print(&alu.y, 1, 5, true);
//             println!();
//             println!();
//             println!("\t------ Z ------");
//             print(&alu.z, 1, 5, true);
//             println!();
//             println!();
//         }
//     }

//     // let mut inp = [9; 14];
//     // let success = loop {
//     //     dbg!(&inp);

//     //     let (result, _) = compute(&alu.z, inp, HashMap::new());
//     //     if result == 0 {
//     //         break inp;
//     //     }

//     //     let mut moving_index = 0;
//     //     loop {
//     //         inp[13 - moving_index] -= 1;
//     //         if inp[13 - moving_index] != 0 {
//     //             break;
//     //         }
//     //         inp[13 - moving_index] = 9;
//     //         moving_index += 1;
//     //     }
//     // };

//     // dbg!(&success);

//     // print(&alu.z, 0, 8, true);

//     // should_be(&alu.z, 0);
//     // search_node(&alu.z);
// }

// fn is_multiplication_by(var_value: &VarValue, value: i64) -> Option<Box<VarValue>>
// {
//     if let VarValueEnum::Maths(MathsType::Mul, left, right) = &var_value.value {
//         if let VarValueEnum::Value(other_value) = left.value {
//             if other_value == value {
//                 Some(right.clone())
//             } else {
//                 None
//             }
//         } else if let VarValueEnum::Value(other_value) = right.value {
//             if other_value == value {
//                 Some(left.clone())
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     } else {
//         None
//     }
// }

// fn compute(var_value: &VarValue, inp: [i64; 14], mut cache: HashMap<i64, i64>) -> (i64, HashMap<i64, i64>)
// {
//     let (result, mut new_cache) = match &var_value.value {
//         VarValueEnum::Inp(index) => (inp[*index], cache),
//         VarValueEnum::Value(value) => (*value, cache),
//         VarValueEnum::Maths(maths_type, left, right) => {
//             let (left_value, new_cache) = compute(left, inp, cache);
//             cache = new_cache;
//             let (right_value, new_cache) = compute(right, inp, cache);
//             cache = new_cache;

//             (match maths_type {
//                 MathsType::Add => left_value + right_value,
//                 MathsType::Mul => left_value * right_value,
//                 MathsType::Div => left_value / right_value,
//                 MathsType::Mod => left_value % right_value,
//                 MathsType::Eql => if left_value == right_value { 1 } else { 0 },
//             }, cache)
//         }
//     };

//     new_cache.insert(var_value.id, result);
//     (result, new_cache)
// }

// fn search_node(var_value: &VarValue)
// {
//     let mut dependency = None;
//     for (index, depend) in var_value.depends_on_inputs.iter().enumerate() {
//         if *depend {
//             if dependency.is_some() {
//                 dependency = None;
//                 break;
//             } else {
//                 dependency = Some(index);
//             }
//         }
//     }

//     if let Some(index) = dependency {
//         println!("Found leaf graph with one dependency to n°{}", index);
//         for value in 1..=9 {
//             let mut inp = [0; 14];
//             inp[index] = value;
//             let (result, _) = compute(var_value, inp, HashMap::new());
//             println!("\t… result for {} is {}", value, result);
//         }
//     } else if let VarValueEnum::Maths(_, left, right) = &var_value.value {
//         search_node(left);
//         search_node(right);
//     }
// }

// fn should_not_be(var_value: &VarValue, value: i64)
// {
//     match &var_value.value {
//         VarValueEnum::Inp(index) => println!("Index n°{} should not be {}", index + 1, value),
//         VarValueEnum::Value(other_value) => {
//             if value == *other_value {
//                 panic!("value {} should not be {}", other_value, value);
//             }
//         },
//         VarValueEnum::Maths(maths_type, left, right) => {
//             match maths_type {
//                 _ => {
//                     println!();
//                     println!();
//                     print(var_value, 0, 6, true);
//                     print(&*left, 0, 2, true);
//                     println!("\t… should not be {}", value);
//                     panic!();
//                 }
//             }
//         }
//     }
// }

// fn should_be(var_value: &VarValue, value: i64)
// {
//     match &var_value.value {
//         VarValueEnum::Inp(index) => println!("Index n°{} should be {}", index + 1, value),
//         VarValueEnum::Value(other_value) => {
//             if value != *other_value {
//                 panic!("value {} should be {}", other_value, value);
//             }
//         },
//         VarValueEnum::Maths(maths_type, left, right) => {
//             match maths_type {
//                 MathsType::Add => {
//                     if value == 0 && var_value.min == 0 {
//                         should_be(left, 0);
//                         should_be(right, 0);
//                     }
//                 },
//                 MathsType::Mul => {
//                     if value == 0 {
//                         if left.min > 0 || left.max < 0 {
//                             should_be(right, 0);
//                         } else if right.min > 0 || right.max < 0 {
//                             should_be(left, 0);
//                         }
//                     }
//                 },
//                 MathsType::Div => {
//                     if value == 0 {
//                         should_be(left, 0);
//                     }
//                 },
//                 MathsType::Eql => {
//                     if let VarValueEnum::Value(other_value) = right.value {
//                         should_not_be(left, other_value);
//                     }
//                 },
//                 _ => {
//                     println!();
//                     println!();
//                     print(var_value, 0, 2, true);
//                     println!("\t… should be {}", value);
//                     panic!();
//                 }
//             }
//         }
//     }
// }


fn print(var_content: &VarContent, deep: usize, max: usize, first: bool)
{
    let prefix = format!("{:\t^1$}", "", deep);
    match &var_content.source {
        Source::Value(digit) => print!("{}Valeur `{}`", prefix, digit),
        Source::Inp(index) => print!("{}Input n°{}", prefix, index + 1),
        Source::Maths(kind, left, right) => {
            if deep < max {
                println!("{}(", prefix);
                print(left, deep + 1, max, false);
                println!();
                print!("{})", prefix);
            } else {
                print!("{}…", prefix);
            }

            print!(" ");
            print_maths_kind(*kind);
            print!(" ");
            
            if deep < max {
                println!("(");
                print(right, deep + 1, max, false);
                println!();
                print!("{})", prefix);
            } else {
                print!("…");
            }

        }
    }

    if first {
        println!();
        println!("{}{}", prefix, content_as_string(&var_content.content));
    }
}

fn print_maths_kind(maths_kind: MathsKind)
{
    match maths_kind {
        MathsKind::Add => print!("+"),
        MathsKind::Mul => print!("*"),
        MathsKind::Div => print!("/"),
        MathsKind::Mod => print!("%"),
        MathsKind::Eql => print!("=="),
    }
}

fn content_as_string(content: &Content) -> String
{
    match content {
        Content::Value(value) => format!("Valeur `{}`", value),
        Content::Range(min, max) => format!("Min `{}`, Max `{}`", min, max),
        Content::Options(options) => {
            let options_as_strings: Vec<String> = options.iter().map(|option| option.to_string()).collect();

            format!("Options: {}", options_as_strings.join(", "))
        },
    }
}

// fn get_zero() -> VarValue
// {
//     VarValue { id: 0, value: VarValueEnum::Value(0), min: 0, max: 0, depends_on_inputs: [false; 14] }
// }

// fn get_value(alu: &Alu, variable: Var) -> VarValue
// {
//     match variable {
//         Var::W => alu.w.clone(),
//         Var::X => alu.x.clone(),
//         Var::Y => alu.y.clone(),
//         Var::Z => alu.z.clone(),
//     }
// }

fn parse_variable(value: &str) -> Var
{
    match value {
        "w" => Var::W,
        "x" => Var::X,
        "y" => Var::Y,
        "z" => Var::Z,
        _ => panic!(),
    }
}

fn parse_var_or_val(value: &str) -> VarOrVal
{
    match value {
        "w" => VarOrVal::Var(Var::W),
        "x" => VarOrVal::Var(Var::X),
        "y" => VarOrVal::Var(Var::Y),
        "z" => VarOrVal::Var(Var::Z),
        digit => VarOrVal::Val(digit.parse().unwrap()),
    }
}

// fn test_input() -> &'static str
// {
//     ""
// }

fn raw_input() -> &'static str
{
    "
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 2
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 4
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 8
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 7
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 12
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 7
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x 0
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 10
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 14
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 2
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -12
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 8
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -3
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 11
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 5
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -2
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 11
    mul y x
    add z y
    "
}