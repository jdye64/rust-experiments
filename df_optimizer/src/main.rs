use std::collections::HashSet;

use datafusion::arrow::datatypes::{Schema, Field, DataType};
use datafusion_common::Result;
use datafusion_expr::{Expr, LogicalPlan, col, LogicalPlanBuilder, logical_plan::table_scan, JoinType, sum};
use datafusion_optimizer::{OptimizerRule, optimizer::ApplyOrder, OptimizerConfig, OptimizerContext};

struct MyRule {
    my_set: HashSet<Expr>,
}

impl MyRule {
    pub fn new() -> Self {
        Self { my_set: HashSet::new() }
    }
    
    pub fn set_myset(mut self, value: HashSet<Expr>) {
        self.my_set = value;
    }

    pub fn set_insert(&mut self, expr: &Expr) {
        self.my_set.insert(expr.clone());
    }
}

pub trait MyOptimizerRule {
    /// Try and rewrite `plan` to an optimized form, returning None if the plan cannot be
    /// optimized by this rule.
    fn try_optimize(
        &mut self,
        plan: &LogicalPlan,
        config: &dyn OptimizerConfig,
    ) -> Result<Option<LogicalPlan>>;

    /// A human readable name for this optimizer rule
    fn name(&self) -> &str;

    /// How should the rule be applied by the optimizer? See comments on [`ApplyOrder`] for details.
    ///
    /// If a rule use default None, its should traverse recursively plan inside itself
    fn apply_order(&self) -> Option<ApplyOrder> {
        None
    }
}

impl MyOptimizerRule for MyRule {
    fn name(&self) -> &str {
        "my_rule"
    }

    fn apply_order(&self) -> Option<ApplyOrder> {
        Some(ApplyOrder::TopDown)
    }

    fn try_optimize(
        &mut self,
        plan: &LogicalPlan,
        _config: &dyn OptimizerConfig,
    ) -> Result<Option<LogicalPlan>> {
        // let mut new_set = HashSet::new();
        // new_set.insert(col("request_method"));
        // self.set_myset(new_set);
        self.set_insert(&col("request_method"));
        panic!("done with testing")
    }
}

fn test_table_scan(table_name: &str, column_name: &str) -> LogicalPlan {
    let schema = Schema::new(vec![
        Field::new(column_name, DataType::UInt32, false),
        Field::new("c", DataType::UInt32, false),
    ]);
    table_scan(Some(table_name), &schema, None)
        .expect("creating scan")
        .build()
        .expect("building plan")
}

fn main() {

    let plan = LogicalPlanBuilder::from(test_table_scan("df", "a"))
            .join(
                LogicalPlanBuilder::from(test_table_scan("df2", "b")).build().unwrap(),
                JoinType::Inner,
                (vec!["c"], vec!["c"]),
                None,
            ).unwrap()
            .aggregate(vec![col("df2.b")], vec![sum(col("df.a"))]).unwrap()
            .project(vec![sum(col("df.a")), col("df2.b")]).unwrap()
            .build().unwrap();
    
    let mut rule = MyRule::new();
    let optimized_plan = rule
        .try_optimize(&plan, &OptimizerContext::new())
        .expect("failed to optimize plan");
    let formatted_plan = format!("{}", optimized_plan.unwrap().display_indent());

    println!("Formatted Plan: {:?}", formatted_plan);
}
