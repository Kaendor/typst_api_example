use criterion::{criterion_group, criterion_main, Criterion};
use typst_pdf_api::templates::{
    template_to_pdf, 
    german_invoice::{GermanTemplateData, InvoiceItem, Author, Client, BankAccount, Address}
};

fn bench_simple_pdf_generation(c: &mut Criterion) {
    c.bench_function("simple_pdf_generation", |b| {
        b.iter(|| {
            template_to_pdf("Hello, Typst!".to_string()).expect("PDF generation failed")
        })
    });
}

fn bench_german_invoice_pdf_generation(c: &mut Criterion) {
    c.bench_function("german_invoice_pdf_generation", |b| {
        b.iter(|| {
            let data = create_test_data();
            let template = data.into_typst_template();
            template_to_pdf(template).expect("German invoice PDF generation failed")
        })
    });
}

fn bench_template_generation_only(c: &mut Criterion) {
    c.bench_function("template_generation_only", |b| {
        b.iter(|| {
            let data = create_test_data();
            data.into_typst_template()
        })
    });
}

fn create_test_data() -> GermanTemplateData {
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
            email: "john@example.com".to_string(),
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

criterion_group!(
    benches,
    bench_simple_pdf_generation,
    bench_german_invoice_pdf_generation,
    bench_template_generation_only
);
criterion_main!(benches);