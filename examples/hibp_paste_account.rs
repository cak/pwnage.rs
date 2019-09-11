fn main() -> Result<(), pwnage::HIBPError> {
    let api_key = String::from(""); // HIBP API Key
    let user_agent = String::from("GitHub.com/OptionalValue");

    let hibp = pwnage::HIBP::new(api_key, user_agent);

    let email_address = ""; // Email Address

    let pastes = hibp.paste_account(email_address)?;
    for paste in &pastes {
        println!(
            "Source: {:?} | Title: {:?} | Date: {:?}",
            paste.source, paste.title, paste.date
        );
    }
    Ok(())
}
