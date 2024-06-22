use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        // Calculate column widths
        let mut col_widths = self.headers.iter().map(|h| h.len()).collect::<Vec<_>>();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > col_widths[i] {
                    col_widths[i] = cell.len();
                }
            }
        }

        // Helper function to format a row
        let format_row = |row: &[String]| {
            row.iter().enumerate().map(|(i, cell)| {
                format!("{:^width$}", cell, width = col_widths[i])
            }).collect::<Vec<_>>().join(" | ")
        };

        // Print headers
        let header_row = format_row(&self.headers);
        writeln!(f, "| {} |", header_row)?;

        // Print separator
        let separator = col_widths.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("-+-");
        writeln!(f, "|-{}-|", separator)?;

        // Print rows
        for row in &self.body {
            let row_str = format_row(row);
            writeln!(f, "| {} |", row_str)?;
        }

        Ok(())
    }
}