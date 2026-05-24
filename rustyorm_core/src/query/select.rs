use crate::query::expr::Expr;
use crate::value::Value;

#[derive(Debug)]
pub struct Select {
    table: String,
    columns: Vec<String>,
    conditions: Vec<Expr>,
    params: Vec<Value>,
}

impl Select {
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            columns: vec![],
            conditions: vec![],
            params: vec![],
        }
    }

    pub fn column(mut self, col: impl Into<String>) -> Self {
        self.columns.push(col.into());
        self
    }

    pub fn where_eq(mut self, col: impl Into<String>, value: impl Into<Value>) -> Self {
        self.conditions.push(Expr::Eq {
            column: col.into(),
            value: value.into(),
        });

        self
    }

    pub fn to_sql(&self) -> String {
        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut sql = format!("SELECT {} FROM {}", columns, self.table);

        let mut params = vec![];

        if !self.conditions.is_empty() {
            sql.push_str(" WHERE ");

            let conditions = self
                .conditions
                .iter()
                .map(|expr| Self::build_condition(expr, &mut params))
                .collect::<Vec<_>>()
                .join(" AND ");

            sql.push_str(&conditions);
        }

        sql
    }

    pub fn params(&self) -> Vec<Value> {
        let mut params = vec![];

        for expr in &self.conditions {
            Self::build_condition(expr, &mut params);
        }

        params
    }

    fn build_condition(expr: &Expr, params: &mut Vec<Value>) -> String {
        match expr {
            Expr::Eq { column, value } => {
                params.push(value.clone());

                let idx = params.len();

                format!("{} = ${}", column, idx)
            }
        }
    }
}
