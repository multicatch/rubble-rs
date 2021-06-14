# rubble-templates-evaluators

A set of parsers, evaluators and compilers for text templates.
Part of rubble-templates library.

## What is this?

This is a part of a template engine.
This engine allows compiling text templates into output text using variable substitution or code evaluation.

This particular crate contains parsers, evaluator and compiler implementations.
To read more about it, refer to [main rubble-templates README](https://github.com/multicatch/rubble-rs/) or [the documentation](https://multicatch.github.io/rubble-rs/rubble_templates_evaluators/).

### Available evaluators and compilers

* Default template schema (as in the [main README](https://github.com/multicatch/rubble-rs/)):
    * [`simple`](https://multicatch.github.io/rubble-rs/rubble_templates_evaluators/simple/index.html) - basic implementation:
        * `Template` - represents a template to parse,
        * `SimpleEvaluationEngine` - a basic evaluator that allows to use custom functions and variables,
        * `TemplateCompiler` - a default compiler that can be used with above structs.