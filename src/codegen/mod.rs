use codegen::function::FunctionCodeGen;
use model::{ast, ir};
use semantics::global_context::GlobalContext;

mod function;

pub struct CodeGen<'a> {
    ast: &'a ast::Program,
    gctx: GlobalContext<'a>,
}

impl<'a> CodeGen<'a> {
    pub fn new(ast: &'a ast::Program, gctx: GlobalContext<'a>) -> CodeGen<'a> {
        CodeGen { ast, gctx }
    }

    pub fn generate_ir(&self) -> ir::Program {
        let mut prog_ir = ir::Program {
            structs: vec![],
            functions: vec![],
            // todo global strings
        };

        for def in &self.ast.defs {
            match def {
                ast::TopDef::FunDef(fun) => {
                    let gfun_cg = FunctionCodeGen::new(None, &self.gctx);
                    let fun_ir = gfun_cg.generate_function_ir(&fun);
                    prog_ir.functions.push(fun_ir);
                }
                ast::TopDef::ClassDef(_cl) => {
                    // todo
                    // let cl_desc = gctx.get_class_description(&cl.name.inner).expect(err_msg);
                    // let cl_ctx = FunctionContext::new(Some(cl_desc), &gctx);
                    // for it in &cl.items {
                    //     match &it.inner {
                    //         InnerClassItemDef::Field(_, _) => (),
                    //         InnerClassItemDef::Method(fun) => {
                    //             cl_ctx
                    //                 .analyze_function(&fun)
                    //                 .accumulate_errors_in(&mut errors);
                    //         }
                    //         InnerClassItemDef::Error => unreachable!(),
                    //     }
                    // }
                }
                ast::TopDef::Error => unreachable!(),
            }
        }

        prog_ir
    }
}
