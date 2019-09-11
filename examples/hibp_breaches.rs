fn main() -> Result<(), pwnage::HIBPError> {
    let api_key = String::from(""); // HIBP API Key
    let user_agent = String::from("GitHub.com/OptionalValue");

    let hibp = pwnage::HIBP::new(api_key, user_agent);

    let email_address = ""; // Email Address

    let breaches = hibp.breached_account_full(email_address)?;
    for breach in &breaches {
        println!(
            "Name: {:?} | Title: {:?} | Date: {:?}",
            breach.name, breach.title, breach.breach_date
        );
    }
    Ok(())
}
