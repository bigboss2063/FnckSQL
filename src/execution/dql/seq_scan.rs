use crate::execution::{Executor, ReadExecutor};
use crate::planner::operator::scan::ScanOperator;
use crate::storage::{Iter, StatisticsMetaCache, TableCache, Transaction};
use crate::throw;

pub(crate) struct SeqScan {
    op: ScanOperator,
}

impl From<ScanOperator> for SeqScan {
    fn from(op: ScanOperator) -> Self {
        SeqScan { op }
    }
}

impl<'a, T: Transaction + 'a> ReadExecutor<'a, T> for SeqScan {
    fn execute(
        self,
        (table_cache, _): (&'a TableCache, &'a StatisticsMetaCache),
        transaction: &'a T,
    ) -> Executor<'a> {
        Box::new(
            #[coroutine]
            move || {
                let ScanOperator {
                    table_name,
                    columns,
                    limit,
                    ..
                } = self.op;

                let mut iter = transaction
                    .read(table_cache, table_name, limit, columns)
                    .unwrap();

                while let Some(tuple) = throw!(iter.next_tuple()) {
                    yield Ok(tuple);
                }
            },
        )
    }
}
