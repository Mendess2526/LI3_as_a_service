use crate::store::client::Client;
use crate::store::product::Product;
use crate::store::{Query8, Expense, ProductSales, TotalBilled};
use itertools::Itertools;

/// Query 1
pub fn stats(s: (usize, usize, usize)) -> String {
    format!(
        "clients: {}
producs: {}
sales: {}
",
        s.0, s.1, s.2
    )
}

// Query 2
pub fn list_by_first_letter(prods: Vec<&Product>) -> String {
    let mut response = String::new();
    let total = prods.len();
    for p in prods {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", total);
    response
}

// Query 3
pub fn total_billed(total_billed: TotalBilled) -> String {
    format!("{:#?}\n", total_billed)
}

// Query 4
pub fn never_bought(prods: Vec<&Product>) -> String {
    let mut response = String::new();
    for p in prods.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", prods.len());
    response
}

// Query 5
pub fn buyers_in_all_filials(clients: Vec<&Client>) -> String {
    let mut response = String::new();
    for p in clients.iter() {
        response += &format!("{}\n", p);
    }
    response
}

// Query 6
pub fn never_bought_never_purchased(n_buyers: usize, n_products: usize) -> String {
    format!("clients: {} | products: {}\n", n_buyers, n_products)
}

// Query 7
pub fn year_purchases(table: (Vec<u32>, Vec<u32>, Vec<u32>)) -> String {
    let mut response = String::new();
    response +=
        "       | Jan | Feb | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dec |\n";
    response +=
        "-------+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+\n";
    response += " One   |";
    for month in table.0.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response += " Two   |";
    for month in table.1.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response += " Three |";
    for month in table.2.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response
}

// Query 8
pub fn total_billed_between(billed: Query8) -> String {
    format!("{:#?}\n", billed)
}

// Query 9
pub fn product_buyers(clients: Vec<&str>) -> String {
    let mut response = String::new();
    for p in clients {
        response += &format!("{}\n", p);
    }
    response
}

// Query 10
pub fn top_purchases(prods: Vec<(&str, u32)>) -> String {
    let mut response = String::new();
    for p in prods {
        response += &format!("{} : {}\n", p.0, p.1);
    }
    response
}

// Query 11
pub fn top_sold_products(prods: Vec<ProductSales>) -> String {
    let mut response = String::new();
    for p in prods {
        response += &format!("{:#?}\n", p);
    }
    response
}

// Query 12
pub fn top_expense(top_expenses: Vec<Expense>) -> String {
    top_expenses.iter()
        .format("\n")
        .to_string()
}
