use std::path::Path;
use std::ffi::OsStr;
use std::sync::Mutex;

use url::Url;
use once_cell::sync::{OnceCell, Lazy};

static IAM : OnceCell<String> = OnceCell::new();
static FUNKS : Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));


fn set_iam(path:&String) -> bool
{
    let cmdnm = match Path::new(path).file_name()
    {
        Some(v) => v,
        None => {
            OsStr::new("Bye bye ...");
            return false;
        },
    };
    let _iam = match cmdnm.to_str()
    {
        Some(v) => crate::help::IAM.set(v.to_string()).unwrap(),
        None => todo!(),
    };
    true
}
fn push_funk(arg:&String)
{
    FUNKS.lock().unwrap().push(arg.to_string());
}

pub fn pop_funk() -> String
{
    let fnk = match FUNKS.lock().unwrap().pop()
        {
            Some(v) => v,
            None => "".to_string(),
        };
    fnk
}

pub fn usage()
{
    let iam = match crate::help::IAM.get()
    {
        Some(v) => v,
        None => todo!(),
    };
    println!("{} [-h|--help] [get] [post] [para] [https]", iam);
}

pub fn parse_argv(argc:usize, argv:Vec<String>) -> bool
{
    println!("[{}]{:?}", argc, argv);

    if set_iam(&argv[0]) == false
    {return false;}

    if argc < 2
    {
        usage();
        false
    }
    else
    {
        let mut ax = 1;
        while ax < argc
        {
            if &argv[ax] == "--help" || &argv[ax] == "-h"
            {
                usage();
                return false;
            }
            else if (&argv[ax] == "get") || (&argv[ax] == "post") || (&argv[ax] == "para") || (&argv[ax] == "https")
            {push_funk(&argv[ax]);}

            ax += 1;
        }
        true
    }
}
