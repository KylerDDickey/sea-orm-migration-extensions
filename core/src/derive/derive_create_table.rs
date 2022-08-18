use crate::codegen::CreateTableStmts;
use crate::emitters::Derive;

pub struct DeriveMigrationTrait;

impl Derive<CreateTableStmts> for DeriveMigrationTrait { }
