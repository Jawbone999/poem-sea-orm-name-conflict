pub use sea_orm_migration::prelude::*;

mod m20240802_161029_create_user_table;
mod m20240802_161049_create_team_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240802_161029_create_user_table::Migration),
            Box::new(m20240802_161049_create_team_table::Migration),
        ]
    }
}
