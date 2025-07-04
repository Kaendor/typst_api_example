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

impl GermanTemplateData {
    pub fn into_pdf_params(self) -> String {
        let GermanTemplateData {
            invoice_number,
            date,
            items,
            author,
            recipient,
            bank_account,
            vat_rate,
            is_micro_business,
        } = self;

        let items_str: String = items
            .into_iter()
            .map(InvoiceItem::into_pdf_params)
            .collect::<Vec<_>>()
            .join(",\n    ");

        let author_str = author.into_pdf_params();

        let client_str = recipient.into_pdf_params();

        let bank_account_str = bank_account.into_pdf_params();

        format!(
            r#"
  {invoice_number},
  // Invoice date
  {date},
  // Items
  (
    {items_str}
  ),
  // Author
    {author_str},
  // Recipient
    {client_str},
  // Bank account
    {bank_account_str},
  // Umsatzsteuersatz (VAT)
  vat: 0.19,
  kleinunternehmer: true,
        "#
        )
    }
}

pub struct BankAccount {
    pub name: String,
    pub iban: String,
    pub bic: String,
    pub bank_name: String,
    pub gender: String,
}

impl BankAccount {
    pub fn into_pdf_params(self) -> String {
        let BankAccount {
            iban,
            bic,
            bank_name,
            name,
            gender,
        } = self;

        format!(
            r#"
  (
    name: "{name}",
    bank: "{bank_name}",
    iban: "{iban}",
    bic: "{bic}",
    // There is currently only one gendered term in this template.
    // You can overwrite it, or omit it and just choose the default.
    gender: (account_holder: "Kontoinhaberin")
    )
                "#
        )
    }
}

pub struct Client {
    pub name: String,
    pub address: Address,
}

impl Client {
    pub fn into_pdf_params(self) -> String {
        let Client {
            name,
            address:
                Address {
                    street,
                    city,
                    zip_code: zip,
                    country: _country,     // Not used in the template
                    tax_nb: _tax_nb,       // Not used in the template
                    signature: _signature, // Not used in the template
                },
        } = self;

        format!(
            r#"(
              name: "{name}",
              street: "{street}",
              zip: "{zip}",
              city: "{city}",
            )"#
        )
    }
}

pub struct InvoiceItem {
    pub description: String,
    pub price: f64,
}

impl InvoiceItem {
    pub fn into_pdf_params(self) -> String {
        let InvoiceItem { description, price } = self;

        format!(
            r#"(
              description: "{description}",
              price: {price},
            )"#
        )
    }
}

pub struct Author {
    pub name: String,
    pub address: Address,
    pub email: String,
}

impl Author {
    pub fn into_pdf_params(self) -> String {
        let Author {
            name,
            address:
                Address {
                    street,
                    city,
                    zip_code: zip,
                    country: _country, // Not used in the template
                    tax_nb,
                    // TODO: optional signature
                    signature: _signature,
                },
            email: _email, // Not used in the template
        } = self;

        format!(
            r#"(
                  name: "{name}",
                  street: "{street}",
                  zip: "{zip}",
                  city: "{city}",
                  tax_nr: "{tax_nb}",
                  // optional signature, can be omitted
                  // signature: image("example_signature.png", width: 5em)
                )"#
        )
    }
}

pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
    pub tax_nb: String,
    pub signature: Option<String>,
}
