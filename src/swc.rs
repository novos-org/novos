use swc_core::common::{
    errors::Handler,
    sync::Lrc,
    FileName, SourceMap, GLOBALS, Mark,
};
use swc_core::ecma::ast::{EsVersion, Program};
use swc_core::ecma::codegen::{text_writer::JsWriter, Config as CodegenConfig, Emitter as CodegenEmitter};
use swc_core::ecma::minifier::{optimize, option::{MinifyOptions, ExtraOptions}};
// Confirmed: Syntax and TsConfig are in the parser module
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_core::ecma::transforms::base::fixer::fixer;
use swc_core::ecma::transforms::typescript::strip;
// Use FoldWith for the transformation and VisitMutWith for the fixer
use swc_core::ecma::visit::{FoldWith, VisitMutWith, as_folder};

pub fn compile_and_minify(src: &str, is_typescript: bool) -> anyhow::Result<String> {
    let cm = Lrc::new(SourceMap::default());
    
    let handler = Handler::with_emitter_writer(
        Box::new(std::io::stderr()),
        Some(cm.clone()),
    );

    GLOBALS.set(&Default::default(), || {
        let fm = cm.new_source_file(FileName::Anon.into(), src.to_string());
        
        let syntax = if is_typescript {
            Syntax::Typescript(TsConfig { 
                ..Default::default() 
            })
        } else {
            Syntax::Es(Default::default())
        };

        let lexer = Lexer::new(syntax, EsVersion::EsNext, StringInput::from(&*fm), None);
        let mut parser = Parser::new_from(lexer);

        let mut module = parser.parse_module().map_err(|e| {
            // Updated to use the correct diagnostic method for this version
            e.into_diagnostic(&handler).emit();
            anyhow::anyhow!("SWC Parsing failed")
        })?;

        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // 1. Strip TypeScript
        // confirmed: strip returns a 'Pass', we wrap it with as_folder
        let mut module = if is_typescript {
            module.fold_with(&mut as_folder(strip(top_level_mark, unresolved_mark)))
        } else {
            module
        };

        // 2. Minify
        let mut program = Program::Module(module);
        
        let extra_opts = ExtraOptions {
            top_level_mark,
            unresolved_mark,
            mangle_name_cache: None,
        };

        program = optimize(
            program,
            cm.clone(),
            None,
            None,
            &MinifyOptions { 
                compress: Some(Default::default()), 
                mangle: Some(Default::default()), 
                ..Default::default() 
            },
            &extra_opts,
        );

        // 3. Fixer (Ensures valid JS output)
        program.visit_mut_with(&mut fixer(None));

        // 4. Codegen
        let mut buf = Vec::new();
        {
            let mut emitter = CodegenEmitter {
                cfg: CodegenConfig::default().with_minify(true),
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };
            emitter.emit_program(&program).unwrap();
        }

        Ok(String::from_utf8(buf)?)
    })
}