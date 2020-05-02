# Find Up

Find Up traverses up from the current path searching for a file or a folder. The main use cases include finding a config file or a lock file for a CLI
that can run from anywhere within a given project.

For example, consider the following directory structure

```
/
└── Users
    └── Gilfoyle
        ├── unicorn.png
        └── foo
            └── bar
                ├── baz
                └── example.js
```

You would like to find the path to `unicorn.png` from within the `bar` directory. The usage for this case is:

```rs
use std::env;
use find_up::find;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_path = current_dir.to_str().unwrap();
    let find_unicorn = find_up::find(current_dir_path, "unicorn.png");

    if test_path.is_some() {
        println!("{}", test_path.unwrap());
    } else {
        println!("File not found!");
    }
}

/*
 *  Output:
 *  /Users/Gilfoyle/unicorn.png
 */
```

This project is inspired by [find-up](https://github.com/sindresorhus/find-up), a JavaScript library by [Sindre Sorhus](https://github.com/sindresorhus) built with the same end goal. It is a popular add on for building CLI and simple package managers with Node.js.

**Note:** This is still in early development phase and as such is not available as a public crate.
