use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn add(&mut self, department: &str, employee: &str) {
        let employees = self
            .departments
            .entry(department.to_lowercase())
            .or_insert(vec![]);
        employees.push(String::from(employee));
    }

    pub fn get_department(&self, department: &str) -> Vec<&str> {
        if let Some(department) = self.departments.get(&department.to_lowercase()) {
            let mut department: Vec<&str> = department.iter().map(|s| s.as_ref()).collect();
            department.sort();
            return department;
        }
        vec![]
    }

    pub fn get_company(&self) -> Vec<(&str, &str)> {
        let mut names: Vec<(&str, &str)> = self
            .departments
            .iter()
            .flat_map(|(dep, names)| names.iter().map(|name| (name.as_ref(), dep.as_ref())))
            .collect();
        names.sort_by_key(|(name, dep)| format!("{name}{dep}"));
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_employees() {
        let mut company = Company::new();
        company.add("Sales", "Amir");
        company.add("Sales", "Boon");
        company.add("Eng", "Boon");

        println!("{:#?}", company.departments);
        assert_eq!(company.departments.len(), 2, "Should have two departments");
        assert_eq!(
            company.get_department("sales").len(),
            2,
            "Should have two employees in sales"
        );
    }

    #[test]
    fn it_should_return_sorted_departments() {
        let mut company = Company::new();
        company.add("Sales", "Boon");
        company.add("Sales", "Amir");
        company.add("Eng", "Boon");

        assert!(company.get_department("Sales").is_sorted());
    }

    #[test]
    fn test_get_company_empty() {
        let company = Company::new();
        assert_eq!(company.get_company().len(), 0);
    }

    #[test]
    fn test_get_company_single_employee() {
        let mut company = Company::new();
        company.add("Sales", "Bob");
        assert_eq!(company.get_company(), vec![("Bob", "sales")]);
    }

    #[test]
    fn test_get_company_multiple_sorted() {
        let mut company = Company::new();
        company.add("Engineering", "Alice");
        company.add("Sales", "Bob");
        company.add("Marketing", "Charlie");
        assert_eq!(
            company.get_company(),
            vec![
                ("Alice", "engineering"),
                ("Bob", "sales"),
                ("Charlie", "marketing")
            ]
        );
    }

    #[test]
    fn test_get_company_same_name() {
        let mut company = Company::new();
        company.add("Sales", "Bob");
        company.add("Engineering", "Bob");
        assert_eq!(
            company.get_company(),
            vec![("Bob", "engineering"), ("Bob", "sales")]
        );
    }

    #[test]
    fn test_get_company_multiple_departments() {
        let mut company = Company::new();
        company.add("Sales", "Charlie");
        company.add("Sales", "Bob");
        company.add("Engineering", "Alice");
        company.add("Marketing", "Bob");
        assert_eq!(
            company.get_company(),
            vec![
                ("Alice", "engineering"),
                ("Bob", "marketing"),
                ("Bob", "sales"),
                ("Charlie", "sales")
            ]
        );
    }
}
