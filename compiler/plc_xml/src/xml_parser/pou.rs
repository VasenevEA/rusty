use ast::ast::{AstStatement, Implementation, SourceRange};

use crate::model::pou::Pou;

use super::ParseSession;

impl Pou {
    fn transform(&self, session: &ParseSession) -> Vec<AstStatement> {
        let Some(fbd) = &self.body.function_block_diagram else {
            // empty body
            return vec![]
        };

        if cfg!(feature = "debug") {
            let statements = fbd.transform(session);
            println!("{statements:#?}");

            return statements;
        }

        fbd.transform(session)
    }

    pub fn build_implementation(&self, session: &ParseSession) -> Implementation {
        let statements = self.transform(session);

        Implementation {
            name: self.name.to_owned(),
            type_name: self.name.to_owned(),
            linkage: session.linkage,
            pou_type: self.pou_type.into(),
            statements,
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
