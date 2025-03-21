mod company;
mod parser;

pub use company::Company;
pub use parser::*;

pub enum CommandResult {
    Continue,
    Break,
}

pub fn execute(command: &str, company: &mut Company) -> CommandResult {
    match Parser::new(command).parsed() {
        Query::Add { name, department } => {
            println!("Adding {name} to {department}...");
            company.add(&department, &name);
        }
        Query::ListDepartment(department) => {
            println!("Listing {department}...");
            for (i, name) in company.get_department(&department).iter().enumerate() {
                println!("{}. {}", i + 1, name);
            }
        }
        Query::ListCompany => {
            println!("Listing company...");
            for (i, (name, department)) in company.get_company().iter().enumerate() {
                println!("{}. {} ({})", i + 1, name, department);
            }
        }
        Query::Exit => {
            println!("Exiting...");
            return CommandResult::Break;
        }
        Query::Help => {
            println!("List of available commands:");
            println!("----------------------------------------------------------------");
            println!("1. add <name> to <department> - Add an employee to a department.");
            println!("2. list <department> - List all employees in a department.");
            println!("3. list - List all employees in the company.");
            println!("4. exit - Exit the program.");
            println!("5. help - Show this help message.");
        }
        Query::Invalid => println!("Invalid command, please try again..."),
    }
    println!();
    CommandResult::Continue
}
