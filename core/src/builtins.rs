use std::collections::HashMap;

use std::cell::RefCell;
use std::io::{stdin, stdout, Write};
use std::rc::Rc;
use substring::Substring;

// use crate::
use crate::afunc;
use crate::func;
use crate::{
    exceptions::Exception,
    runtime::{FunctionType, Object, RFunction, Value},
};

pub fn functions() -> HashMap<String, RFunction> {
    HashMap::from([
        func!("put", zsp_put, 1),
        func!("get", zsp_get, 0),
        func!("exit", zsp_exit, 0),
        func!("assert", zsp_assert, 2),
        func!("array", zsp_array_new, 0),
        func!("object", zsp_object_new, 0),
        func!("len", zsp_array_len, 1),
        func!("null", zsp_null_new, 0),
        func!("panic", zsp_panic, 0),
    ])
}

pub fn boolprototype<'a>() -> HashMap<String, Rc<RefCell<Value<'a>>>> {
    HashMap::new()
}
pub fn stringprototype<'a>() -> HashMap<String, Rc<RefCell<Value<'a>>>> {
    HashMap::from([
        afunc!("substr", string_substr, 2),
        afunc!("split", string_split, 1),
        afunc!("at", string_at, 1),
        afunc!("tolower", string_to_lower, 1),
        afunc!("replace", string_replace, 2),
    ])
}
pub fn arrayprototype<'a>() -> HashMap<String, Rc<RefCell<Value<'a>>>> {
    HashMap::from([

        afunc!("push",array_push,1),
        afunc!("pop",array_pop,0) 
    ])
}
fn array_pop(mut inp:Vec<Value>) -> Result<Value,Exception>{
    match inp[0].as_ref().borrow_mut().as_array().pop(){
        Some(v)=>{
           Ok(v.borrow_mut().clone())
        }
        None=>{
            Err(Exception::new(0, "ArrayIndexException","Cannot pop from a zero length array"))
        }
    }
}
fn array_push(mut inp:Vec<Value>) -> Result<Value,Exception>{
    let v = Rc::new(RefCell::new(inp[1].clone()));
    inp[0].as_ref().borrow_mut().as_array().push(v);
    Ok(Value::Null)
}
pub fn numberprototype<'a>() -> HashMap<String, Rc<RefCell<Value<'a>>>> {
    HashMap::from([
        afunc!("pow", number_pow, 1),
        afunc!("tostring", number_tostring, 1),
    ])
}
fn number_pow(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Number(inp[0].to_number().powf(inp[1].to_number())))
}
fn number_tostring(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::String(inp[0].to_number().to_string()))
}
fn string_substr(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::String(
        inp[0]
            .to_string()
            .substring(inp[1].to_number() as usize, inp[2].to_number() as usize)
            .to_string(),
    ))
}
fn string_at(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::String(
        inp[0].to_string().chars().collect::<Vec<char>>()[inp[1].to_number() as usize].to_string(),
    ))
}
fn string_to_lower(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::String(inp[0].to_string().to_lowercase()))
}
fn string_replace(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::String(
        inp[0]
            .to_string()
            .replace(&inp[1].to_string(), &inp[2].to_string()),
    ))
}
fn string_split(inp: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Array(
        inp[0]
            .to_string()
            .split(|c| c == inp[1].to_string().chars().collect::<Vec<char>>()[0])
            .map(|f| Rc::new(RefCell::new(Value::String(f.to_string()))))
            .collect(),
    ))
    // man i sure do love iterators (clueless)
}

fn zsp_array_new(_args: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Array(vec![]))
}

pub fn zsp_object_new(_args: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Object(Object {
        fields: HashMap::new(),
    }))
}
pub fn zsp_null_new(_args: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Null)
}
fn zsp_array_len(args: Vec<Value>) -> Result<Value, Exception> {
    Ok(Value::Number(args[0].clone().as_ref().borrow_mut().as_array().len() as f32))
}

fn zsp_put(args: Vec<Value>) -> Result<Value, Exception> {
    println!("{}", args[0].to_string());
    Ok(Value::Null)
}

fn zsp_get(_inp: Vec<Value>) -> Result<Value, Exception> {
    stdout().flush().unwrap();
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("you didn't enter a string");
    s = s.chars().filter(|c| c != &'\n' && c != &'\r').collect();
    // if let Ok(f) = s.parse::<f32>() {
    //     Ok(Value::Number(f))
    // } else if let Ok(b) = s.parse::<bool>() {
    //     Value::Bool(b)
    // } else {
    Ok(Value::String(s))
    // }
}
fn zsp_exit(_args: Vec<Value>) -> Result<Value, Exception> {
    dbg!("exiting");
    std::process::exit(0);
}
fn zsp_panic(_args: Vec<Value>) -> Result<Value, Exception> {
    panic!("panic from code")
}
fn zsp_assert(inp: Vec<Value>) -> Result<Value, Exception> {
    if inp[0] != inp[1] {
        panic!("FAILED ASSERT: {:?} not equal to {:?}", inp[0], inp[1]);
    }
    Ok(Value::Null)
}
