use crate::implementation::MigrationStatement;
use crate::emitters::Derive;

pub struct DeriveMigrationStatementsTrait;

impl Derive<MigrationStatement> for DeriveMigrationStatementsTrait { }
