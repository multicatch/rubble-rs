var searchIndex = JSON.parse('{\
"rubble_templates":{"doc":"General purpose API for rubble-templates.","t":[0,0,5,5,5,5,5,5,5,5,0,17,5,5,5,5,5,5,5,5,5],"n":["std_fun","math","math_functions","plus_function","minus_function","multiply_function","divide_function","modulo_function","reduce_numbers","as_numbers","strings","EMPTY_STRING","string_functions","concat_function","trim_function","right_brackets_function","quotes_function","std_functions","compile_template_from_file","compile_template_from_string","compile_template_from"],"q":["rubble_templates","rubble_templates::std_fun","rubble_templates::std_fun::math","","","","","","","","rubble_templates::std_fun","rubble_templates::std_fun::strings","","","","","","rubble_templates::std_fun","rubble_templates","",""],"d":["This module contains standard functions for …","Provides standard math functions.","Provides a set of math functions.","Adds (or concatenates) values. If any of the parameters …","Subtracts values. If any of the parameters is not …","Multiplies values. If any of the parameters is not …","Divides values. If any of the parameters is not …","Calculates the remainder (modulo). If any of the …","","","Provides standard string manipulation and utility …","","Provides a set of string related functions.","Concatenates the parameters.","Trims the parameters. If there is more than one …","Inserts “}}”. Ignores the parameters.","Inserts “. Ignores the parameters.","Provides a set of standard functions.","Compiles template from file.","Compiles template from String.","Compiles template from [Template]."],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[null,null,[[],[["hashmap",3],["box",3],["string",3]]],[[],["string",3]],[[["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[],[["syntaxerror",3],["result",4],["string",3]]],[[]],null,null,[[],[["hashmap",3],["box",3],["string",3]]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],[["hashmap",3],["box",3],["string",3]]],[[["pathbuf",3],["string",3],["box",3],["hashmap",3],["hashmap",3]],[["box",3],["result",4],["string",3]]],[[["box",3],["string",3],["hashmap",3],["hashmap",3]],[["result",4],["compilationerror",4],["string",3]]],[[["box",3],["string",3],["template",3],["hashmap",3],["hashmap",3]],[["result",4],["compilationerror",4],["string",3]]]],"p":[]},\
"rubble_templates_core":{"doc":"","t":[0,4,13,13,13,13,11,0,8,4,13,12,12,12,13,12,12,12,0,4,13,12,12,12,13,12,12,11,11,11,0,8,10,3,11,11,11,11,11,11,3,12,12,12,11,11,4,13,12,12,13,12,13,12,12,13,12,12,8,10,0,3,11,3,11,3,11,5,0,8,16,16,10,4,13,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["units","Position","Unknown","RelativeToInvocation","RelativeToCodeStart","Absolute","raw_value","template","EvaluableMixedContent","TemplateSlice","Text","value","start_position","end_position","Code","value","start_position","end_position","ast","SyntaxNode","NamedNode","identifier","starts_at","children","AnonymousNode","starts_at","children","is_anonymous","add_child","with_identifier","evaluator","Evaluator","evaluate","Context","empty","with_variables","set_variable","get_variable","save_state","get_state","SyntaxError","relative_pos","invocation_pos","description","new","at_position","EvaluationError","UnexpectedElements","last_expected","unexpected_elements","UnknownSymbol","symbol","InvalidArguments","description","arguments","InvalidValues","description","values","Function","evaluate","functions","SimpleFunction","new","FunctionWithContext","new","FunctionWithAst","new","resolve_params","compiler","Compiler","Item","ItemIterator","compile","CompilationError","EvaluationFailed","error","position","source","from","into","to_owned","clone_into","to_string","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","to_string","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_string","borrow","borrow_mut","try_from","try_into","type_id","evaluate","evaluate","evaluate","clone","clone","eq","ne","eq","ne","eq","ne","eq","ne","eq","ne","eq","ne","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt"],"q":["rubble_templates_core","rubble_templates_core::units","","","","","","rubble_templates_core","rubble_templates_core::template","","","rubble_templates_core::template::TemplateSlice","","","rubble_templates_core::template","rubble_templates_core::template::TemplateSlice","","","rubble_templates_core","rubble_templates_core::ast","","rubble_templates_core::ast::SyntaxNode","","","rubble_templates_core::ast","rubble_templates_core::ast::SyntaxNode","","rubble_templates_core::ast","","","rubble_templates_core","rubble_templates_core::evaluator","","","","","","","","","","","","","","","","","rubble_templates_core::evaluator::EvaluationError","","rubble_templates_core::evaluator","rubble_templates_core::evaluator::EvaluationError","rubble_templates_core::evaluator","rubble_templates_core::evaluator::EvaluationError","","rubble_templates_core::evaluator","rubble_templates_core::evaluator::EvaluationError","","rubble_templates_core::evaluator","","rubble_templates_core","rubble_templates_core::functions","","","","","","","rubble_templates_core","rubble_templates_core::compiler","","","","","","rubble_templates_core::compiler::CompilationError","","","rubble_templates_core::units","","","","","","","","","","rubble_templates_core::template","","","","","","","rubble_templates_core::ast","","","","","","","","","","rubble_templates_core::evaluator","","","","","","","","","","","","","","","","","","","","","rubble_templates_core::functions","","","","","","","","","","","","","","","","","","","","","rubble_templates_core::compiler","","","","","","","","rubble_templates_core::functions","","","rubble_templates_core::units","rubble_templates_core::ast","rubble_templates_core::units","","rubble_templates_core::template","","rubble_templates_core::ast","","rubble_templates_core::evaluator","","","","rubble_templates_core::compiler","","rubble_templates_core::units","rubble_templates_core::template","rubble_templates_core::ast","rubble_templates_core::evaluator","","rubble_templates_core::compiler","rubble_templates_core::units","rubble_templates_core::ast","rubble_templates_core::compiler"],"d":["Basic rubble-templates units.","Represents a position of node/symbol in template.","Used when there is no way to calculate the exact or …","Used to indicate a position relative to current function …","Used to indicate a position relative to the start of …","Used to indicate an absolute position in current template.","","An API for representation of parsed templates.","Represents content that can be both template parts and …","A slice of template that can be returned by an iterator.","","","","","","","","","An API for AST representation.","Represents a node in an AST","","","","","","","","","","","An API for evaluators that can be used to evaluate code …","Trait that describes an ability to evaluate code in …","","Context that is passed while evaluating an AST by an […","","","","","","","An error that can happen during evaluation with full info …","","","","Creates new [SyntaxError] with given [EvaluationError].","Creates new [SyntaxError] with given [EvaluationError] at …","An error that can happen during evaluation.","","","","","","","","","","","","A function that can be used to add features to the …","","An API for creating custom functions that extend the …","A wrapper for a <code>Fn(&[String]) -> String</code>, to be used in […","","A wrapper for a …","","A wrapper for a …","","Resolves a slice of [SyntaxNode]s to a <code>Vec</code> of strings.","An API for compilers that use a selected parser and …","Describes a struct that is able to compile a template.","Type of a single template part that can be …","Iterator that can provide template parts that need to be …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,1,1,0,0,0,2,3,3,3,2,4,4,4,0,0,5,6,6,6,5,7,7,5,5,5,0,0,8,0,9,9,9,9,9,9,0,10,10,10,10,10,0,11,12,12,11,13,11,14,14,11,15,15,0,16,0,0,17,0,18,0,19,0,0,0,20,20,20,0,21,22,22,22,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,5,5,5,5,5,5,5,5,5,5,9,9,9,9,9,9,9,10,10,10,10,10,10,10,11,11,11,11,11,11,11,17,17,17,17,17,17,17,18,18,18,18,18,18,18,19,19,19,19,19,19,19,21,21,21,21,21,21,21,21,17,18,19,1,5,1,1,2,2,5,5,10,10,11,11,21,21,1,2,5,10,11,21,1,5,21],"f":[null,null,null,null,null,null,[[],[["usize",15],["option",4]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["bool",15]],[[["syntaxnode",4]],["syntaxnode",4]],[[["position",4],["str",15]],["syntaxnode",4]],null,null,[[["context",3],["syntaxnode",4]],[["syntaxerror",3],["result",4],["string",3]]],null,[[],["context",3]],[[["hashmap",3],["string",3]],["context",3]],[[["str",15]]],[[["str",15]],[["string",3],["option",4]]],[[["any",8]]],[[],["option",4]],null,null,null,null,[[["evaluationerror",4]],["syntaxerror",3]],[[["position",4],["evaluationerror",4]],["syntaxerror",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[["evaluator",8],["context",3]],[["syntaxerror",3],["result",4],["string",3]]],null,null,[[],[["box",3],["simplefunction",3]]],null,[[],[["functionwithcontext",3],["box",3]]],null,[[],[["functionwithast",3],["box",3]]],[[["evaluator",8],["context",3]],[["syntaxerror",3],["vec",3],["result",4]]],null,null,null,null,[[["context",3]],[["string",3],["compilationerror",4],["result",4]]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["string",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["string",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[],["string",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["evaluator",8],["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[["evaluator",8],["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[["evaluator",8],["context",3]],[["syntaxerror",3],["result",4],["string",3]]],[[],["position",4]],[[],["syntaxnode",4]],[[["position",4]],["bool",15]],[[["position",4]],["bool",15]],[[["templateslice",4]],["bool",15]],[[["templateslice",4]],["bool",15]],[[["syntaxnode",4]],["bool",15]],[[["syntaxnode",4]],["bool",15]],[[["syntaxerror",3]],["bool",15]],[[["syntaxerror",3]],["bool",15]],[[["evaluationerror",4]],["bool",15]],[[["evaluationerror",4]],["bool",15]],[[["compilationerror",4]],["bool",15]],[[["compilationerror",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]]],"p":[[4,"Position"],[4,"TemplateSlice"],[13,"Text"],[13,"Code"],[4,"SyntaxNode"],[13,"NamedNode"],[13,"AnonymousNode"],[8,"Evaluator"],[3,"Context"],[3,"SyntaxError"],[4,"EvaluationError"],[13,"UnexpectedElements"],[13,"UnknownSymbol"],[13,"InvalidArguments"],[13,"InvalidValues"],[8,"Function"],[3,"SimpleFunction"],[3,"FunctionWithContext"],[3,"FunctionWithAst"],[8,"Compiler"],[4,"CompilationError"],[13,"EvaluationFailed"]]},\
"rubble_templates_evaluators":{"doc":"","t":[0,5,0,0,3,11,0,3,11,0,3,12,11,11,3,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["parser","parse_ast","simple","evaluator","SimpleEvaluationEngine","from","compiler","TemplateCompiler","new","template","Template","raw_content","read_from","from","EvaluableMixedContentIterator","source","current_position","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","from","into","into_iter","borrow","borrow_mut","try_from","try_into","type_id","next","eq","ne","fmt","evaluate","compile"],"q":["rubble_templates_evaluators","rubble_templates_evaluators::parser","rubble_templates_evaluators","rubble_templates_evaluators::simple","rubble_templates_evaluators::simple::evaluator","","rubble_templates_evaluators::simple","rubble_templates_evaluators::simple::compiler","","rubble_templates_evaluators::simple","rubble_templates_evaluators::simple::template","","","","","","","rubble_templates_evaluators::simple::evaluator","","","","","","","rubble_templates_evaluators::simple::compiler","","","","","","","rubble_templates_evaluators::simple::template","","","","","","","","","","","","","","","","","","","rubble_templates_evaluators::simple::evaluator","rubble_templates_evaluators::simple::compiler"],"d":["Default parser logic","Used for parsing AST for further evaluation.","Set of basic evaluators and compilers for templates","Evaluator compatible with <code>Template</code> and core AST …","Simple evaluation engine providing basic features like …","","Compiler for <code>Template</code>, evaluates code blocks and joins …","","","Simple representation of text template that will be …","A simple template that represents a source text.","","","","Iterates over some template source and returns code …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,0,0,2,0,0,3,3,3,0,4,4,1,1,1,1,1,1,1,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,3,3,3,1,2],"f":[null,[[["str",15]],["syntaxnode",4]],null,null,null,[[["string",3],["hashmap",3],["box",3]],["simpleevaluationengine",3]],null,null,[[],["templatecompiler",3]],null,null,null,[[["path",3]],[["result",4],["error",3],["template",3]]],[[["string",3]],["template",3]],null,null,null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["option",4]],[[["template",3]],["bool",15]],[[["template",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["context",3],["syntaxnode",4]],[["string",3],["syntaxerror",3],["result",4]]],[[["context",3]],[["string",3],["compilationerror",4],["result",4]]]],"p":[[3,"SimpleEvaluationEngine"],[3,"TemplateCompiler"],[3,"Template"],[3,"EvaluableMixedContentIterator"]]}\
}');
initSearch(searchIndex);