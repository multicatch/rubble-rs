# rubble-templates

A template engine in Rust. Allows compiling text templates into output text.

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
let functions: HashMap<String, Box<dyn Function>> = std_functions();

let mut variables: HashMap<String, String> = HashMap::new();
variables.insert("name".to_string(), "Joe".to_string());

let result = compile_template_from_file(file, variables, functions);

assert_eq!(result.ok(), Some("Hello there, Joe!".to_string()));
```

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

*Note*: The above functions are the example functions and are not actual functions that are available out-of-the-box.
They might be added in the future releases however.

### Standard functions

A set of standard, built-in functions is still being expanded. 
To see current list of standard functions, refer to the [rubble-templates documentation](https://multicatch.github.io/rubble-rs/rubble_templates/std_fun/index.html).

### Customizing

Compilation contains three phases:
* `parsing` - done by the template iterator, eg. `EvaluableMixedContentIterator<'a, T>`, which can extract template parts,
* `evaluation` - done by the `Evaluator`, which evaluates all code found by the iterator,
* `compiling` - done by the `Compiler`, which uses iterator to parse the content, feeds the `Evaluator` and then joins everything into output text.

You can implement your own iterators, evaluators or compilers. 
To modify the compilation process, you just need to use your own trait implementations instead of the default ones.

To show you how it works, here is what `compile_template_from_string` does in practice:
```rust
// parameters
let raw_input = "Hello there, {{ name }}!".to_string();
let variables: HashMap<String, String> = HashMap::new();
let functions: HashMap<String, Box<dyn Function>> = HashMap::new();

// prepare compilation evironment
let template = Template::from(raw_input);
let engine = SimpleEvaluationEngine::from(functions);
let compiler = TemplateCompiler::new(engine);

// compile template
compiler.compile(&template, Context::with_variables(variables))
```

### Custom functions

Evaluator can be extended with functions by using custom `Function` trait implementations.
To make this process easier, the following structs are available in `rubble-templates-core`, that can be used with static functions and lambdas:
* `SimpleFunction` - for `Fn(&[String]) -> String`. 
  Use this when you want to implement a simple function without any side effects.
* `FunctionWithContext` - for `Fn(&[String], &mut Context) -> Result<String, SyntaxError>`. 
  Use this when you want to use pre-evaluated parameters, but you still need variables. 
  Side effects can cause errors indicated by SyntaxError. 
* `FunctionWithAst` - for `Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError>`.
  Gives full access to `SyntaxNode`s of parameters and `Evaluator`. 
  Allows evaluating additional expressions, manipulating the AST or introducing DSL (domain-specific language).

Mind you, `Context` is a struct that holds variables and states that can be shared between function invocations. 
You can use it to store some properties.

For more info about functions, refer to the [rubble-templates-core documentation](https://multicatch.github.io/rubble-rs/rubble_templates_core/functions/index.html).

## Feedback

If you see any error or feel like suggesting a feature, [create an issue](https://github.com/multicatch/rubble-rs/issues).

Please attach the following when creating an issue:
* a brief description of what happened/what should be improved,
* an actual output (if possible),
* an expected output,
* a minimal reproducible example (a code fragment, a test, a repository, anything that proves that the problem exists).
