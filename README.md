# rubble-templates

A template engine in Rust.

## What does it do?

It allows to use a template with some placeholders that will be replaced.
Consider the following example:

```text
Hello there, {{ name }}!
```

Depending on what value will be supplied for `name`, the output might be different.
For example, if one used `name` with value of `Joe`, it will generate the following output:

```text
Hello there, Joe!
```

This library evaluates such templates and compiles them into output text like in the example above.

## Usage

To simply compile a template, you can use `compile_template_from_file(file: PathBuf, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>)`.

```rust
let file = PathBuf::from("template-file.txt");
let functions: HashMap<String, Box<dyn Function>> = HashMap::new();

let mut variables: HashMap<String, String> = HashMap::new();
variables.insert("name".to_string(), "Joe".to_string());

let result = compile_template_from_file(file, variables, functions);

assert_eq!(result.ok(), Some("Hello there, Joe!".to_string()));
```

There are also some other utility functions:
* `compile_template_from_string(template: String, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>)` - can be used to compile text from a template as raw text. 
* `compile_template_fromcompile_template_from(template: Template, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>)` - can be used to compile text from a Template instance.

Those are handy functions to compile templates, but for some specialized cases, you might compile template using an `Evaluator` and `Compiler` directly.

### Syntax

By default, rubble-templates while parsing a template looks for all blocks starting `{{` and ending with `}}` those are marked as an evaluation spots with code that can be evaluated by an `Evaluator`.

Given the following example:
```text
Hello there, {{ name }}!
```

This template contains three parts:
* `Hello there, ` - raw text
* `{{ name }}` - code
* `!` - raw text

The second part will be passed to a given `Evaluator` by the `Compiler` in order to get a String output.
The code fragment will be substituted with the output.

The rubble-templates library can also evaluate more sophisticated code.
You can pass your own functions that can enrich the template.

This library uses Lisp-like syntax during evaluation.
All function calls look like the following:
```text
function_name arg0 arg1 arg2
```

To call a `plus` function (that can be hypothetically implemented) to calculate the result of 2 + 2 you will need to write
```text
The result is: {{ plus 2 2 }}
```

The parameters can also be grouped using parenthesis. This can be helpful in certain cases.
For example, given `plus` and `multiply` functions, you will need to use the following code to calculate (1 + 2) * 3:

```text
The result is: {{ multiply (plus 1 2) 3 }}
```

### Customizing

Compilation contains three phases:
* `parsing` - done by the template iterator, eg. `EvaluableMixedContentIterator<'a, T>`, which can extract template parts,
* `evaluation` - done by the `Evaluator`, which evaluates all code found by the iterator,
* `compiling` - done by the `Compiler`, which uses iterator to parse the content, feeds the `Evaluator` and then joins everything into output text.

You can implement your own iterators, evaluators or compilers. 
To modify the compilation process, you just need to use your own trait implementations instead of the default ones.

To show you how it works, here is what `compile_template_from_string` does in practice:
```rust
let raw_input = "Hello there, {{ name }}!".to_string();
let variables: HashMap<String, String> = HashMap::new();
let functions: HashMap<String, Box<dyn Function>> = HashMap::new();

// compilation below
let template = Template::from(raw_input);
let engine = SimpleEvaluationEngine::from(functions);
let compiler = TemplateCompiler::new(engine);

compiler.compile(&template, &variables)
```

### Custom functions

Evaluator can be extended with functions by using custom `Function` trait implementations.
To make this process easier, there is default implementation for `F where F: Fn(&dyn Evaluator, &Vec<SyntaxNode>, &HashMap<String, String>, usize) -> Result<String, EvaluationError>`,
so this means that you can use lambdas or Rust functions.

Example with function:
```rust
#[test]
fn should_compile_template() {
    let text = Template::from("{{hello}}\n2 + 2 = {{ plus 2 2 }}".to_string());
    
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.insert("plus".to_string(), Box::new(plus_function));

    let mut variables: HashMap<String, String> = HashMap::new();
    variables.insert("hello".to_string(), "Hello world!".to_string());

    let result = compile_template_from_string(text, variables, functions);

    assert_eq!(result.ok(), Some("Hello world!.\n2 + 2 = 4".to_string()));
}

fn plus_function(evaluator: &dyn Evaluator, parameters: &Vec<SyntaxNode>, variables: &HashMap<String, String>, _offset: usize) -> Result<String, EvaluationError> {
    Ok(
        parameters.iter()
            .map(|node|
                evaluator.evaluate(node.clone(), variables).unwrap().parse::<i32>().unwrap()
            )
            .sum::<i32>()
            .to_string()
    )
}
```

## Feedback

If you see any error or feel like suggesting a feature, [create an issue](https://github.com/multicatch/rubble-rs/issues).