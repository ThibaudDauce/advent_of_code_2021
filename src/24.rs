use std::cmp;
use std::io::{stdin};
use std::collections::HashMap;

fn main()
{
    let mut operations = vec![];
    let lines: Vec<&str> = raw_input().trim().lines().map(|line| line.trim()).collect();

    let step_by_step = true;

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

        if step_by_step {
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

    if ! step_by_step {
        print(&alu.z, 1, 25, true);
    }

    for input_index in 0..14 {
        for value in 1..=9 {
            let z = simplify(alu.z.clone(), input_index, value);

            let min = z.content().min();
            let max = z.content().max();

            println!("Input n°{}. Value: `{}`. Min `{}`, Max `{}`", input_index + 1, value, min, max);
        }
    }

    // if let VarContent { source: Source::Maths(MathsKind::Add, additions ) } = alu.z {
    //     for addition in &additions {
    //         println!();
    //         println!();
    //         print(addition, 1, 2, true);
    //         println!();
    //         println!();
    //     }
    // }
}

fn simplify(var_content: VarContent, input_index: usize, value: i64) -> VarContent
{
    match var_content.source {
        Source::Inp(index) if index == input_index => get_var_content_from_value(value),
        Source::Maths(kind, parts) => {
            let mut result = None;
            for part in parts {
                let new_part = simplify(part, input_index, value);
                if let Some(inner_result) = result {
                    result = Some(merge_var_contents(kind, inner_result, new_part));
                } else {
                    result = Some(new_part);
                }
            }

            result.unwrap()
        }
        _ => var_content,
    }
}

fn get_var_content_from_value(value: i64) -> VarContent
{
    VarContent { source: Source::Value(value) }
}

fn get_var_content_from_input(index: usize) -> VarContent
{
    VarContent { source: Source::Inp(index) }
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

                merge_var_contents(kind, left, right)
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
    match (left.clone(), right.clone()) {
        (VarContent { source: Source::Value(left_val) }, VarContent { source: Source::Value(right_val) }) => {
            get_var_content_from_value(kind.execute(left_val, right_val))
        },

        // Comparaison
        (_, _) if kind == MathsKind::Eql && (left.content().max() < right.content().min() || left.content().min() > right.content().max()) => {
            get_var_content_from_value(0)
        },

        // Multiplication
        (VarContent { source: Source::Value(0) }, _) if kind == MathsKind::Mul => {
            get_var_content_from_value(0)
        },
        (_, VarContent { source: Source::Value(0) }) if kind == MathsKind::Mul => {
            get_var_content_from_value(0)
        },
        (VarContent { source: Source::Value(1) }, _) if kind == MathsKind::Mul => {
            right
        },
        (_, VarContent { source: Source::Value(1) }) if kind == MathsKind::Mul => {
            left
        },

        // Addition
        (VarContent { source: Source::Value(0) }, _) if kind == MathsKind::Add => {
            right
        },
        (_, VarContent { source: Source::Value(0) }) if kind == MathsKind::Add => {
            left
        },

        // Division
        (VarContent { source: Source::Value(0) }, _) if kind == MathsKind::Div => {
            get_var_content_from_value(0)
        },
        (_, VarContent { source: Source::Value(1) }) if kind == MathsKind::Div => {
            left
        },
        (_, VarContent { source: Source::Value(div_value) }) if kind == MathsKind::Div && left.content().min() >= 0 && left.content().max() < div_value => {
            get_var_content_from_value(0)
        },

        // Mod
        (_, VarContent { source: Source::Value(1) }) if kind == MathsKind::Mod => {
            get_var_content_from_value(0)
        },
        (_, VarContent { source: Source::Value(mod_value) }) if kind == MathsKind::Mod && left.content().max() <= mod_value => { // Selon l'énoncé : mod_value > 0 et left >= 0
            left
        },

        // Simplification d'un mod sur une addition ((x*26)+56+y)%26 = (0+56%26+y%26)%26
        (
            VarContent { source: Source::Maths(MathsKind::Add, additions ) },
            VarContent { source: Source::Value(mod_value) },
        ) if kind == MathsKind::Mod => {
            let mut removed = false;
            let mut new_additions = Vec::with_capacity(additions.len());
            'main_loop: for addition in additions {
                if let VarContent { source: Source::Value(value) } = addition {
                    if value % mod_value != 0 {
                        new_additions.push(get_var_content_from_value(value));
                    } else {
                        removed = true;
                    }
                } else if let VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) } = addition {
                    for multiplication in &multiplications {
                        if let Source::Value(multiplier) = multiplication.source {
                            if multiplier % mod_value == 0 {
                                removed = true;
                                continue 'main_loop;
                            }
                        }
                    }
                    new_additions.push(VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) });
                } else {
                    new_additions.push(addition.clone());
                }
            }

            let additions_var_content = if new_additions.is_empty() {
                removed = true;
                get_var_content_from_value(0)
            } else {
                VarContent { source: Source::Maths(MathsKind::Add, new_additions ) }
            };

            if removed {
                merge_var_contents(MathsKind::Mod, additions_var_content, get_var_content_from_value(mod_value))
            } else {
                VarContent { source: Source::Maths(kind, vec![left, right]) }
            }
        },

        // Simplification d'une division sur une addition ((x*26)+56+y)/26 = x+((56+y)/26)
        // (
        //     VarContent { source: Source::Maths(MathsKind::Add, additions ) },
        //     VarContent { source: Source::Value(mod_value) },
        // ) if kind == MathsKind::Mod => {
        //     let mut removed = false;
        //     let mut new_additions = Vec::with_capacity(additions.len());
        //     'main_loop: for addition in additions {
        //         if let VarContent { source: Source::Value(value) } = addition {
        //             if value % mod_value != 0 {
        //                 new_additions.push(get_var_content_from_value(value));
        //             } else {
        //                 removed = true;
        //             }
        //         } else if let VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) } = addition {
        //             for multiplication in &multiplications {
        //                 if let Source::Value(multiplier) = multiplication.source {
        //                     if multiplier % mod_value == 0 {
        //                         removed = true;
        //                         continue 'main_loop;
        //                     }
        //                 }
        //             }
        //             new_additions.push(VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) });
        //         } else {
        //             new_additions.push(addition.clone());
        //         }
        //     }

        //     let additions_var_content = if new_additions.is_empty() {
        //         removed = true;
        //         get_var_content_from_value(0)
        //     } else {
        //         VarContent { source: Source::Maths(MathsKind::Add, new_additions ) }
        //     };

        //     if removed {
        //         merge_var_contents(MathsKind::Mod, additions_var_content, get_var_content_from_value(mod_value))
        //     } else {
        //         VarContent { source: Source::Maths(kind, vec![left, right]) }
        //     }
        // },

        // Simplifications d'une multiplication suivant une division (x/26)*26 = x+(-1*(x%26))
        (
            VarContent { source: Source::Maths(MathsKind::Div, divisions ) },
            VarContent { source: Source::Value(mul_value) },
        ) if kind == MathsKind::Mul && divisions[1].value() == Some(mul_value) => {
            merge_var_contents(
                MathsKind::Add,
                divisions[0].clone(), 
                merge_var_contents(
                    MathsKind::Mul,
                    get_var_content_from_value(-1),
                    merge_var_contents(MathsKind::Mod, divisions[0].clone(), get_var_content_from_value(mul_value))
                ),
            )
        },

        // Simplifications de deux additions à la suite (x+2)+3 = (x+5)
        (
            VarContent { source: Source::Maths(MathsKind::Add, additions ) },
            VarContent { source: Source::Value(new_value) },
        ) if kind == MathsKind::Add => {
            append_values_to_contents(kind, &additions, &vec![get_var_content_from_value(new_value)])
        },
        (
            VarContent { source: Source::Value(new_value) },
            VarContent { source: Source::Maths(MathsKind::Add, additions ) },
        ) if kind == MathsKind::Add => {
            append_values_to_contents(kind, &additions, &vec![get_var_content_from_value(new_value)])
        },


        // Simplifications de deux multiplications à la suite (x*2)*3 = (x*6)
        (
            VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) },
            VarContent { source: Source::Value(new_value) },
        ) if kind == MathsKind::Mul => {
            append_values_to_contents(kind, &multiplications, &vec![get_var_content_from_value(new_value)])
        },
        (
            VarContent { source: Source::Value(new_value) },
            VarContent { source: Source::Maths(MathsKind::Mul, multiplications ) },
        ) if kind == MathsKind::Mul => {
            append_values_to_contents(kind, &multiplications, &vec![get_var_content_from_value(new_value)])
        },

        // Simplifications d'une addition de deux additions (x+2)+(y+4) = (x+y+6)
        (
            VarContent { source: Source::Maths(MathsKind::Add, left_additions ) },
            VarContent { source: Source::Maths(MathsKind::Add, right_additions ) },
        ) if kind == MathsKind::Add => {
            append_values_to_contents(kind, &left_additions, &right_additions)
        },

        // Simplification d'un multiplication d'une addition par un chiffre (x+y+2)*3 = (x*3)+(y*3)+6
        (
            VarContent { source: Source::Maths(MathsKind::Add, additions ) },
            VarContent { source: Source::Value(right_value) },
        ) if kind == MathsKind::Mul => {
            let mut new_additions = Vec::with_capacity(additions.len());
            for addition in additions {
                if let VarContent { source: Source::Value(value) } = addition {
                    new_additions.push(get_var_content_from_value(value * right_value));
                } else {
                    new_additions.push(merge_var_contents(MathsKind::Mul, addition.clone(), get_var_content_from_value(right_value)));
                }
            }

            VarContent { source: Source::Maths(MathsKind::Add, new_additions ) }
        },

        _ => VarContent { source: Source::Maths(kind, vec![left, right]) },
    }
}


fn append_values_to_contents(kind: MathsKind, previous_values: &Vec<VarContent>, new_values: &Vec<VarContent>) -> VarContent
{
    let (mut values, simplified_new_value): (Vec<VarContent>, Option<VarContent>) = if kind == MathsKind::Mul {
        // Simplification si on rajoute une multiplication à une liste de multiplications existantes dont une division par le même chiffre
        // (x/26)*y*26 = x*y

        let mut values_simplified: Option<Vec<VarContent>> = None;

        // Cette première boucle permet de trouver si nous avons une value brute dans les nouvelle valeurs
        let mut simplified = None;

        for new_value in new_values {
            if let VarContent { source: Source::Value(multiplier) } = new_value {
                let mut previous_values_simplified = vec![];

                // Si nous avons une valeur brute dans les nouvelles valeurs on recherche si nous avons une division dans les valeurs précédentes
                for previous_value in previous_values {
                    if let VarContent { source: Source::Maths(MathsKind::Div, division ) } = previous_value {
                        if let VarContent { source: Source::Value(divider) } = division[1] {
                            if *multiplier == divider && simplified.is_none() {
                                simplified = Some(new_value.clone());
                                previous_values_simplified.push(division[0].clone());
                            } else {
                                previous_values_simplified.push(previous_value.clone());
                            }
                        } else {
                            previous_values_simplified.push(previous_value.clone());
                        }
                    } else {
                        previous_values_simplified.push(previous_value.clone());
                    }
                }

                values_simplified = Some(previous_values_simplified);
                break;
            }
        }

        (values_simplified.unwrap_or_else(|| previous_values.clone()), simplified)
    } else {
        (previous_values.clone(), None)
    };

    'main_loop: for value in new_values {
        if let Some(simplified) = &simplified_new_value {
            if simplified == value {
                continue;
            }
        }
        if let VarContent { source: Source::Value(real_value) } = value {
            for previous_value in values.iter_mut() {
                if let VarContent { source: Source::Value(previous_real_value) } = previous_value {
                    *previous_value = get_var_content_from_value(kind.execute(*previous_real_value, *real_value));

                    continue 'main_loop;
                }
            }
        } 
        
        values.push(value.clone());
    }

    VarContent { source: Source::Maths(kind, values) }
}

#[derive(Debug, Clone, PartialEq)]
struct VarContent {
    // content: Content,
    source: Source,
}

impl VarContent {
    fn content(&self) -> Content
    {
        match &self.source {
            Source::Value(value) => Content::Value(*value),
            Source::Inp(_) => Content::Options(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Source::Maths(kind, values) => {
                let mut new_content: Option<Content> = None;
                for value in values {
                    if let Some(content) = new_content {
                        new_content = Some(merge_content(*kind, &content, &value.content()));
                    } else {
                        new_content = Some(value.content());
                    }
                }
                new_content.unwrap()
            },
        }
    }

    fn value(&self) -> Option<i64>
    {
        match &self.source {
            Source::Value(value) => Some(*value),
            _ => None,
        }
    }
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

                (Content::Value(left), Content::Options(options)) => Content::Options(options.iter().map(|right| kind.execute(*left, *right)).collect()),
                (Content::Options(options), Content::Value(right)) => Content::Options(options.iter().map(|left| kind.execute(*left, *right)).collect()),

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
                    compute_new_range(
                        kind,
                        *left_options.iter().min().unwrap(),
                        *left_options.iter().max().unwrap(),
                        *right_options.iter().min().unwrap(),
                        *right_options.iter().max().unwrap(),
                    )
                    // let mut new_options = Vec::with_capacity(left_options.len() * right_options.len());

                    // for left_option in left_options {
                    //     for right_option in right_options {
                    //         new_options.push(kind.execute(*left_option, *right_option));
                    //     }
                    // }

                    // Content::Options(new_options)
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

// fn compute(var_content: &VarContent, inp: [i64; 14], mut cache: HashMap<i64, i64>) -> (i64, HashMap<i64, i64>)
// {
//     let (result, mut new_cache) = match &var_content.source {
//         Source::Inp(index) => (inp[*index], cache),
//         Source::Value(value) => (*value, cache),
//         Source::Maths(kind, values) => {
//             let mut values_iter = values.iter();
//             let (mut result, new_cache) = compute(values_iter.next().unwrap(), inp, cache);
//             cache = new_cache;

//             for value in values_iter {
//                 let (new_value, new_cache) = compute(value, inp, cache);
//                 cache = new_cache;
//                 result = kind.execute(result, new_value);
//             }

//             (result, cache)
//         }
//     };

//     // new_cache.insert(var_value.id, result);
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
//     } else if let Source::Maths(_, left, right) = &var_value.value {
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
        Source::Maths(kind, values) => {
            let mut first_value = true;
            
            for value in values {
                if first_value {
                    first_value = false;
                    print!("{}", prefix);
                } else {
                    print!(" ");
                    print_maths_kind(*kind);
                    print!(" ");
                }

                if deep < max {
                    println!("(");
                    print(value, deep + 1, max, false);
                    println!();
                    print!("{})", prefix);
                } else {
                    print!("…");
                }
            }
        }
    }

    if first {
        println!();
        println!("{}{}", prefix, content_as_string(&var_content.content()));
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

            if options_as_strings.len() > 10 {
                let left = options_as_strings.iter().take(5).cloned().collect::<Vec<String>>().join(", ");
                let right = options_as_strings.iter().rev().take(5).cloned().collect::<Vec<String>>().join(", ");
                format!("Options: {} … {}", left, right)
            } else {
                format!("Options: {}", options_as_strings.join(", "))
            }
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