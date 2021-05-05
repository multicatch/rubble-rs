var searchIndex = JSON.parse('{\
"rubble_templates":{"doc":"","i":[[0,"template","rubble_templates","",null,null],[0,"content","rubble_templates::template","",null,null],[8,"EvaluableMixedContent","rubble_templates::template::content","Represents content that can be both template parts and …",null,null],[4,"TemplateSlice","","A slice of template that can be returned by an iterator.",null,null],[13,"Text","","",0,null],[12,"value","rubble_templates::template::content::TemplateSlice","",1,null],[12,"start_position","","",1,null],[12,"end_position","","",1,null],[13,"Code","rubble_templates::template::content","",0,null],[12,"value","rubble_templates::template::content::TemplateSlice","",2,null],[12,"start_position","","",2,null],[12,"end_position","","",2,null],[3,"EvaluableMixedContentIterator","rubble_templates::template::content","Iterates over some template source and returns code …",null,null],[3,"Template","rubble_templates::template","A simple template that represents a source text.",null,null],[12,"raw_content","","",3,null],[11,"read_from","","",3,[[["path",3]],[["template",3],["error",3],["result",4]]]],[11,"from","","",3,[[["string",3]],["template",3]]],[0,"evaluator","rubble_templates","",null,null],[0,"ast","rubble_templates::evaluator","",null,null],[4,"SyntaxNode","rubble_templates::evaluator::ast","Represents a node in an AST",null,null],[13,"NamedNode","","",4,null],[12,"identifier","rubble_templates::evaluator::ast::SyntaxNode","",5,null],[12,"starts_at","","",5,null],[12,"children","","",5,null],[13,"AnonymousNode","rubble_templates::evaluator::ast","",4,null],[12,"starts_at","rubble_templates::evaluator::ast::SyntaxNode","",6,null],[12,"children","","",6,null],[5,"parse_ast","rubble_templates::evaluator::ast","Used for parsing AST for further evaluation.",null,[[["str",15]],["syntaxnode",4]]],[0,"engine","rubble_templates::evaluator","",null,null],[3,"SimpleEvaluationEngine","rubble_templates::evaluator::engine","Simple evaluation engine providing basic features like …",null,null],[11,"from","","",7,[[["hashmap",3],["string",3],["box",3]],["simpleevaluationengine",3]]],[8,"Evaluator","rubble_templates::evaluator","Trait that describes an ability to evaluate code in …",null,null],[10,"evaluate","","",8,[[["hashmap",3],["syntaxnode",4]],[["result",4],["string",3],["syntaxerror",3]]]],[3,"SyntaxError","","An error that can happen during evaluation with full info …",null,null],[12,"relative_pos","","",9,null],[12,"description","","",9,null],[4,"EvaluationError","","An error that can happen during evaluation.",null,null],[13,"UnexpectedElements","","",10,null],[12,"last_expected","rubble_templates::evaluator::EvaluationError","",11,null],[12,"unexpected_elements","","",11,null],[13,"UnknownSymbol","rubble_templates::evaluator","",10,null],[12,"symbol","rubble_templates::evaluator::EvaluationError","",12,null],[13,"InvalidArguments","rubble_templates::evaluator","",10,null],[12,"description","rubble_templates::evaluator::EvaluationError","",13,null],[12,"arguments","","",13,null],[8,"Function","rubble_templates::evaluator","A function that can be used to add features to the …",null,null],[10,"evaluate","","",14,[[["hashmap",3],["usize",15],["evaluator",8]],[["result",4],["string",3],["syntaxerror",3]]]],[0,"compiler","rubble_templates","",null,null],[8,"Compiler","rubble_templates::compiler","Describes a struct that is able to compile a template.",null,null],[16,"Item","","",15,null],[16,"ItemIterator","","",15,null],[10,"compile","","",15,[[["hashmap",3]],[["compilationerror",4],["string",3],["result",4]]]],[4,"CompilationError","","",null,null],[13,"EvaluationFailed","","",16,null],[12,"error","rubble_templates::compiler::CompilationError","",17,null],[12,"position","","",17,null],[12,"source","","",17,null],[3,"TemplateCompiler","rubble_templates::compiler","",null,null],[11,"new","","",18,[[],["templatecompiler",3]]],[5,"compile_template_from_file","rubble_templates","Compiles template from file.",null,[[["hashmap",3],["string",3],["pathbuf",3],["hashmap",3],["box",3]],[["box",3],["result",4],["string",3]]]],[5,"compile_template_from_string","","Compiles template from String.",null,[[["hashmap",3],["string",3],["hashmap",3],["box",3]],[["compilationerror",4],["string",3],["result",4]]]],[5,"compile_template_from","","Compiles template from Template.",null,[[["hashmap",3],["string",3],["template",3],["hashmap",3],["box",3]],[["compilationerror",4],["string",3],["result",4]]]],[11,"from","rubble_templates::template::content","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"into_iter","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","rubble_templates::template","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","rubble_templates::evaluator::ast","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","rubble_templates::evaluator::engine","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","rubble_templates::evaluator","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","rubble_templates::compiler","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_string","","",16,[[],["string",3]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"evaluate","rubble_templates::evaluator::engine","",7,[[["hashmap",3],["syntaxnode",4]],[["result",4],["string",3],["syntaxerror",3]]]],[11,"compile","rubble_templates::compiler","",18,[[["hashmap",3]],[["compilationerror",4],["string",3],["result",4]]]],[11,"next","rubble_templates::template::content","",19,[[],["option",4]]],[11,"clone","rubble_templates::evaluator::ast","",4,[[],["syntaxnode",4]]],[11,"eq","rubble_templates::template::content","",0,[[["templateslice",4]],["bool",15]]],[11,"ne","","",0,[[["templateslice",4]],["bool",15]]],[11,"eq","rubble_templates::template","",3,[[["template",3]],["bool",15]]],[11,"ne","","",3,[[["template",3]],["bool",15]]],[11,"eq","rubble_templates::evaluator::ast","",4,[[["syntaxnode",4]],["bool",15]]],[11,"ne","","",4,[[["syntaxnode",4]],["bool",15]]],[11,"eq","rubble_templates::evaluator","",9,[[["syntaxerror",3]],["bool",15]]],[11,"ne","","",9,[[["syntaxerror",3]],["bool",15]]],[11,"eq","","",10,[[["evaluationerror",4]],["bool",15]]],[11,"ne","","",10,[[["evaluationerror",4]],["bool",15]]],[11,"eq","rubble_templates::compiler","",16,[[["compilationerror",4]],["bool",15]]],[11,"ne","","",16,[[["compilationerror",4]],["bool",15]]],[11,"fmt","rubble_templates::template::content","",0,[[["formatter",3]],["result",6]]],[11,"fmt","rubble_templates::template","",3,[[["formatter",3]],["result",6]]],[11,"fmt","rubble_templates::evaluator::ast","",4,[[["formatter",3]],["result",6]]],[11,"fmt","rubble_templates::evaluator","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","rubble_templates::compiler","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]]],"p":[[4,"TemplateSlice"],[13,"Text"],[13,"Code"],[3,"Template"],[4,"SyntaxNode"],[13,"NamedNode"],[13,"AnonymousNode"],[3,"SimpleEvaluationEngine"],[8,"Evaluator"],[3,"SyntaxError"],[4,"EvaluationError"],[13,"UnexpectedElements"],[13,"UnknownSymbol"],[13,"InvalidArguments"],[8,"Function"],[8,"Compiler"],[4,"CompilationError"],[13,"EvaluationFailed"],[3,"TemplateCompiler"],[3,"EvaluableMixedContentIterator"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);