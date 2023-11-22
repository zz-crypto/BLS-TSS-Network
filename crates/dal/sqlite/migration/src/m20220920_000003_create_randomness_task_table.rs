use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RandomnessTask::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RandomnessTask::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::RequestId)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::SubscriptionId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::GroupIndex)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::RequestType)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::Params)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RandomnessTask::Requester).text().not_null())
                    .col(
                        ColumnDef::new(RandomnessTask::Seed)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::RequestConfirmations)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::CallbackGasLimit)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::CallbackMaxGasPrice)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::AssignmentBlockHeight)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::State)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RandomnessTask::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RandomnessTask::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RandomnessTask {
    Table,
    Id,
    RequestId,
    SubscriptionId,
    GroupIndex,
    RequestType,
    Params,
    Requester,
    Seed,
    RequestConfirmations,
    CallbackGasLimit,
    CallbackMaxGasPrice,
    AssignmentBlockHeight,
    State,
    CreateAt,
    UpdateAt,
}
