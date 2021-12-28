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
    Maths(MathsType, Var, VarOrVal),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MathsType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

enum MinMax {
    Value(i64),
    Inp(u32),
}

#[derive(Debug, Clone, PartialEq)]
struct VarValue {
    value: VarValueEnum,
    min: i64,
    max: i64,
}

#[derive(Debug, Clone, PartialEq)]
enum VarValueEnum {
    Value(i64),
    Inp(u32),
    Maths(MathsType, Box<VarValue>, Box<VarValue>),
}

#[derive(Debug)]
struct Alu {
    w: VarValue,
    x: VarValue,
    y: VarValue,
    z: VarValue,
}

fn main()
{
    let mut operations = vec![];
    for line in raw_input().trim().lines() {
        let (operation_string, params) = line.trim().split_once(' ').unwrap();

        let operation = match operation_string {
            "inp" => Ops::Inp(parse_variable(params)),
            _ => {
                let (variable_string, var_or_val_string) = params.split_once(' ').unwrap();
                let variable = parse_variable(variable_string);
                let var_or_val = parse_var_or_val(var_or_val_string);

                match operation_string {
                    "add" => Ops::Maths(MathsType::Add, variable, var_or_val),
                    "mul" => Ops::Maths(MathsType::Mul, variable, var_or_val),
                    "div" => Ops::Maths(MathsType::Div, variable, var_or_val),
                    "mod" => Ops::Maths(MathsType::Mod, variable, var_or_val),
                    "eql" => Ops::Maths(MathsType::Eql, variable, var_or_val),
                    _ => panic!(),
                }
            },
        };

        operations.push(operation);
    }

    let mut next_inp_index = 0;

    let mut alu = Alu {
        w: get_zero(),
        x: get_zero(),
        y: get_zero(),
        z: get_zero(),
    };

    let operations_length = operations.len();

    for (index, operation) in operations.into_iter().enumerate() {
        println!("Operation {}/{}", index + 1, operations_length);
        // dbg!(&operation);
        let (variable, value) = match operation {
            Ops::Inp(var) => {
                let result = (var, VarValue { value: VarValueEnum::Inp(next_inp_index), min: 1, max: 9 });
                next_inp_index += 1;
                result
            },
            Ops::Maths(maths_type, left_var, right_var) => {
                let left = get_value(&alu, left_var);
                let right = match right_var {
                    VarOrVal::Val(value) => VarValue { value: VarValueEnum::Value(value), min: value, max: value },
                    VarOrVal::Var(variable) => get_value(&alu, variable),
                };

                match (&left.value, &right.value) {
                    (VarValueEnum::Value(left_value), VarValueEnum::Value(right_value)) => {
                        let result = match maths_type {
                            MathsType::Add => left_value + right_value,
                            MathsType::Mul => left_value * right_value,
                            MathsType::Div => left_value / right_value,
                            MathsType::Mod => left_value % right_value,
                            MathsType::Eql => if left_value == right_value { 1 } else { 0 },
                        };

                        (left_var, VarValue { value: VarValueEnum::Value(result), min: result, max: result })
                    },
                    _ => {
                        let new_value = VarValueEnum::Maths(maths_type, Box::new(left.clone()), Box::new(right.clone()));

                        let (simple_value, min_max): (Option<VarValue>, Option<(i64, i64)>) = match maths_type {
                            MathsType::Mul => {
                                let new_min = [left.min * right.min, left.min * right.max, left.max * right.min, left.max * right.max].into_iter().min().unwrap();
                                let new_max = [left.min * right.min, left.min * right.max, left.max * right.min, left.max * right.max].into_iter().max().unwrap();

                                if right.value == VarValueEnum::Value(0) {
                                    (Some(get_zero()), None)
                                } else if right.value == VarValueEnum::Value(1) {
                                    (Some(left), None)
                                } else {
                                    (None, Some((new_min, new_max)))
                                }
                            },
                            MathsType::Add => {
                                let new_min = left.min + right.min;
                                let new_max = left.max + right.max;

                                if right.value == VarValueEnum::Value(0) {
                                    (Some(left.clone()), Some((new_min, new_max)))
                                } else if left.value == VarValueEnum::Value(0) {
                                    (Some(right.clone()), Some((new_min, new_max)))
                                } else {
                                    (None, Some((new_min, new_max)))
                                }
                            },
                            MathsType::Div => {
                                let new_min = [left.min / right.min, left.min / right.max, left.max / right.min, left.max / right.max].into_iter().min().unwrap();
                                let new_max = [left.min / right.min, left.min / right.max, left.max / right.min, left.max / right.max].into_iter().max().unwrap();

                                if right.value == VarValueEnum::Value(1) {
                                    (Some(left.clone()), None)
                                } else if left.value == VarValueEnum::Value(1) {
                                    (Some(get_zero()), None)
                                } else if let VarValueEnum::Value(divider) = right.value {
                                    if left.min >= 0 && left.max < divider {
                                        (Some(get_zero()), None)
                                    } else {
                                        (None, Some((new_min, new_max)))
                                    }
                                } else {
                                    (None, Some((new_min, new_max)))
                                }
                            },
                            MathsType::Mod => {
                                let new_min = 0;
                                let new_max = right.max - 1;

                                if right_var == VarOrVal::Val(1) || new_min == new_max {
                                    (Some(get_zero()), None)
                                } else {
                                    (None, Some((new_min, new_max)))
                                }
                            },
                            MathsType::Eql => {
                                if left.max < right.min || left.min > right.max {
                                    (Some(get_zero()), None)
                                } else {
                                    (None, Some((0, 1)))
                                }
                            },
                        };

                        let min_max = min_max.unwrap_or((i64::MIN, i64::MAX));
        
                        (
                            left_var,
                            simple_value.unwrap_or(VarValue { value: new_value, min: min_max.0, max: min_max.1 }),
                        )
                    }
                }

            },
        };

        match variable {
            Var::W => alu.w = value,
            Var::X => alu.x = value,
            Var::Y => alu.y = value,
            Var::Z => alu.z = value,
        }

        // if index > 120 {
        //     break;
        // }
    }

    
    // print(&alu.z, 0, 3);
    should_be(&alu.z, 0);
}

fn should_not_be(var_value: &VarValue, value: i64)
{
    match &var_value.value {
        VarValueEnum::Inp(index) => println!("Index n°{} should not be {}", index + 1, value),
        VarValueEnum::Value(other_value) => {
            if value == *other_value {
                panic!("value {} should not be {}", other_value, value);
            }
        },
        VarValueEnum::Maths(maths_type, left, right) => {
            match maths_type {
                _ => {
                    println!();
                    println!();
                    print(var_value, 0, 6);
                    println!("\t… should not be {}", value);
                    panic!();
                }
            }
        }
    }
}

fn should_be(var_value: &VarValue, value: i64)
{
    match &var_value.value {
        VarValueEnum::Inp(index) => println!("Index n°{} should be {}", index + 1, value),
        VarValueEnum::Value(other_value) => {
            if value != *other_value {
                panic!("value {} should be {}", other_value, value);
            }
        },
        VarValueEnum::Maths(maths_type, left, right) => {
            match maths_type {
                MathsType::Add => {
                    if value == 0 && var_value.min == 0 {
                        should_be(left, 0);
                        should_be(right, 0);
                    }
                },
                MathsType::Mul => {
                    if value == 0 {
                        if left.min > 0 || left.max < 0 {
                            should_be(right, 0);
                        } else if right.min > 0 || right.max < 0 {
                            should_be(left, 0);
                        }
                    }
                },
                MathsType::Div => {
                    if value == 0 {
                        should_be(left, 0);
                    }
                },
                MathsType::Eql => {
                    if let VarValueEnum::Value(other_value) = right.value {
                        should_not_be(left, other_value);
                    }
                },
                _ => {
                    println!();
                    println!();
                    print(var_value, 0, 2);
                    println!("\t… should be {}", value);
                    panic!();
                }
            }
        }
    }
}

fn print(var_value: &VarValue, deep: usize, max: usize)
{
    let prefix = format!("{:\t^1$}", "", deep);
    match &var_value.value {
        VarValueEnum::Value(digit) => print!("{}Valeur `{}`", prefix, digit),
        VarValueEnum::Inp(index) => print!("{}Input n°{}", prefix, index + 1),
        VarValueEnum::Maths(maths_type, left, right) => {
            if deep < max {
                println!("{}(", prefix);
                print(left, deep + 1, max);
                println!();
                print!("{})", prefix);
            } else {
                print!("{}…", prefix);
            }

            print!(" ");
            print_maths_type(*maths_type);
            print!(" ");
            
            if deep < max {
                println!("(");
                print(right, deep + 1, max);
                println!();
                print!("{})", prefix);
            } else {
                print!("…");
            }

        }
    }

    if deep == 0 {
        println!();
        println!("Min: {}, Max {}", var_value.min, var_value.max);
    }
}
fn print_maths_type(maths_type: MathsType)
{
    match maths_type {
        MathsType::Add => print!("+"),
        MathsType::Mul => print!("*"),
        MathsType::Div => print!("/"),
        MathsType::Mod => print!("%"),
        MathsType::Eql => print!("=="),
    }
}

fn get_zero() -> VarValue
{
    VarValue { value: VarValueEnum::Value(0), min: 0, max: 0}
}

fn get_value(alu: &Alu, variable: Var) -> VarValue
{
    match variable {
        Var::W => alu.w.clone(),
        Var::X => alu.x.clone(),
        Var::Y => alu.y.clone(),
        Var::Z => alu.z.clone(),
    }
}

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

fn test_input() -> &'static str
{
    ""
}

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