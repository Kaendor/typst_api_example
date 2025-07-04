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
    pub fn into_typst_template(self) -> String {
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
#import "@preview/classy-german-invoice:0.3.1": invoice

#show: invoice(
  "{invoice_number}",
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
  )
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

#[cfg(test)]
mod tests {

    use crate::templates::template_to_pdf;

    use super::*;

    impl GermanTemplateData {
        pub fn fake() -> Self {
            GermanTemplateData {
                invoice_number: "12345".to_string(),
                date: "2023-10-01".to_string(),
                items: vec![
                    InvoiceItem {
                        description: "Item 1".to_string(),
                        price: 100.0,
                    },
                    InvoiceItem {
                        description: "Item 2".to_string(),
                        price: 200.0,
                    },
                ],
                author: Author {
                    name: "John Doe".to_string(),
                    address: Address {
                        street: "123 Main St".to_string(),
                        city: "Berlin".to_string(),
                        zip_code: "10115".to_string(),
                        country: "Germany".to_string(),
                        tax_nb: "DE123456789".to_string(),
                        signature: None,
                    },
                    email: "toto".to_string(),
                },
                recipient: Client {
                    name: "Jane Smith".to_string(),
                    address: Address {
                        street: "456 Elm St".to_string(),
                        city: "Munich".to_string(),
                        zip_code: "80331".to_string(),
                        country: "Germany".to_string(),
                        tax_nb: "DE987654321".to_string(),
                        signature: None,
                    },
                },
                bank_account: BankAccount {
                    name: "John Doe".to_string(),
                    iban: "DE89370400440532013000".to_string(),
                    bic: "COBADEFFXXX".to_string(),
                    bank_name: "Commerzbank".to_string(),
                    gender: "Yes".to_string(),
                },
                vat_rate: 19.0,
                is_micro_business: true,
            }
        }
    }

    #[test]
    fn compile_german_data_into_german_invoice_pdf() {
        let data = GermanTemplateData::fake();
        let template = data.into_typst_template();
        let _pdf = template_to_pdf(template).expect("Failed to compile template");
    }
}
