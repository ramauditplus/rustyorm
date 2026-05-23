use crate::value::Value;

#[derive(Debug)]
pub struct Select {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
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
        let idx = self.params.len() + 1;

        self.conditions.push(format!("{} = ${}", col.into(), idx));
        self.params.push(value.into());

        self
    }

    pub fn to_sql(&self) -> String {
        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut sql = format!("select {} from {}", columns, self.table);

        if !self.conditions.is_empty() {
            sql.push_str(" where ");
            sql.push_str(&self.conditions.join(" AND "));
        };

        sql
    }

    pub fn params(&self) -> &[Value] {
        &self.params
    }
}
