pub const GERMAN_INVOICE_TEMPLATE: &str = include_str!("../../templates/german_invoice.typ");

pub struct GermanTemplateData {
    pub invoice_number: String,
    /// YYYY-MM-DD format
    pub date: String,
    pub items: Vec<InvoiceItem>,
    pub author: Author,
    pub recipient: Client,
    pub bank_account: BankAccount,
    pub vat_rate: f32,
    /// Whether the invoice is for a micro business or kleinunternehmer
    pub is_micro_business: bool,
}

pub struct BankAccount {
    pub iban: String,
    pub bic: String,
    pub bank_name: String,
}

pub struct Client {
    pub name: String,
    pub address: Address,
}

pub struct InvoiceItem {
    pub description: String,
    pub price: f64,
}

pub struct Author {
    pub name: String,
    pub address: Address,
    pub email: String,
}

pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
    pub tax_nb: String,
    pub signature: Option<String>,
}
