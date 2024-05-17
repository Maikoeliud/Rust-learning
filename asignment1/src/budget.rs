use std::io;

struct BudgetManager {
    income: f64,
    expenses: f64,
}

impl BudgetManager {
    fn new() -> Self {
        Self {
            income: 0.0,
            expenses: 0.0,
        }
    }

    fn add_income(&mut self, amount: f64) {
        self.income += amount;
        println!("Income added: ${}", amount);
    }

    fn add_expense(&mut self, amount: f64) {
        self.expenses += amount;
        println!("Expense added: ${}", amount);
    }

    fn view_budget(&self) {
        let balance = self.income - self.expenses;
        println!("Income: ${}", self.income);
        println!("Expenses: ${}", self.expenses);
        println!("Balance: ${}", balance);
    }
}

fn main() {
    let mut budget_manager = BudgetManager::new();

    loop {
        println!("Budget Manager");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View Budget");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the income amount:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                budget_manager.add_income(amount);
            }
            2 => {
                println!("Enter the expense amount:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                budget_manager.add_expense(amount);
            }
            3 => {
                budget_manager.view_budget();
            }
            4 => {
                println!("Exiting the budget manager.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}